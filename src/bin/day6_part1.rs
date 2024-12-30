use std::io::{self, Read};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum MapTile {
    Empty { visited: bool },
    Obstructed,
}

impl Direction {
    fn rotate_90(self: &Self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Size {
    width: usize,
    height: usize,
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_dir(self: &Self, size: &Size, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::Up if self.y >= 1 => Some(Position {
                x: self.x,
                y: self.y - 1,
            }),
            Direction::Right if self.x < size.width - 1 => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::Down if self.y < size.height - 1 => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::Left if self.x >= 1 => Some(Position {
                x: self.x - 1,
                y: self.y,
            }),
            _ => None,
        }
    }
}

struct GuardData {
    direction: Direction,
    position: Position,
}

struct LabMap {
    size: Size,
    tiles: Vec<MapTile>,
}

struct LabData {
    map: LabMap,
    guard_data: Option<GuardData>,
}

impl LabData {
    fn from_bytes(bytes: impl Iterator<Item = u8>) -> Self {
        let mut first_line_width: Option<usize> = None;
        let mut guard_data: Option<GuardData> = None;
        let mut tiles: Vec<MapTile> = Vec::new();
        for byte in bytes {
            match byte {
                b'.' => {
                    tiles.push(MapTile::Empty { visited: false });
                }
                b'#' => {
                    tiles.push(MapTile::Obstructed);
                }
                b'X' => {
                    tiles.push(MapTile::Empty { visited: true });
                }
                b'^' | b'>' | b'v' | b'<' => {
                    let i = tiles.len();
                    guard_data = Some(GuardData {
                        direction: match byte {
                            b'^' => Direction::Up,
                            b'>' => Direction::Right,
                            b'v' => Direction::Down,
                            b'<' => Direction::Left,
                            _ => unreachable!(),
                        },
                        position: Position {
                            x: first_line_width.map_or(i, |width| i % width),
                            y: first_line_width.map_or(0, |width| i / width),
                        },
                    });
                    tiles.push(MapTile::Empty { visited: true });
                }
                b'\n' => {
                    if first_line_width.is_none() {
                        first_line_width = Some(tiles.len());
                    }
                }
                _ => {}
            }
        }
        let width = first_line_width.unwrap_or_else(|| tiles.len());
        let height = tiles.len() / width;
        let size = Size { width, height };
        LabData {
            map: LabMap { size, tiles },
            guard_data,
        }
    }
}

impl LabMap {
    fn get_mut_tile_by_idx(self: &mut Self, index: usize) -> Option<&mut MapTile> {
        self.tiles.get_mut(index)
    }

    fn get_mut_tile(self: &mut Self, pos: &Position) -> Option<&mut MapTile> {
        self.get_mut_tile_by_idx(pos.x + pos.y * self.size.width)
    }
}

fn main() {
    let mut distinct_pos = 1u32;
    let mut lab_data = LabData::from_bytes(io::stdin().bytes().filter_map(Result::ok));
    if let Some(ref mut guard_data) = lab_data.guard_data {
        loop {
            if let Some(new_pos) = guard_data
                .position
                .move_in_dir(&lab_data.map.size, &guard_data.direction)
            {
                match lab_data.map.get_mut_tile(&new_pos) {
                    Some(MapTile::Empty { ref mut visited }) => {
                        guard_data.position = new_pos;
                        if !*visited {
                            distinct_pos += 1;
                            *visited = true;
                        }
                    }
                    Some(MapTile::Obstructed) => {
                        guard_data.direction = guard_data.direction.rotate_90();
                    }
                    None => unreachable!(),
                }
            } else {
                // left the mapped area
                break;
            }
        }
    } else {
        eprintln!("No guard found on map");
    }
    println!("{distinct_pos}")
}
