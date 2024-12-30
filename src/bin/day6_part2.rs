use std::{
    collections::HashSet,
    io::{self, Read},
};

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
enum MapTile {
    Empty,
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

#[derive(Clone)]
struct Size {
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
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

#[derive(PartialEq, Eq, Hash, Clone)]
struct Placement {
    direction: Direction,
    position: Position,
}

impl Placement {
    fn move_forward(self: &Self, size: &Size) -> Option<Self> {
        let new_pos = self.position.move_in_dir(size, &self.direction);
        new_pos.map(|position| Placement {
            position,
            direction: self.direction.clone(),
        })
    }

    fn rotate_90(self: &Self) -> Self {
        let new_dir = self.direction.rotate_90();
        Placement {
            position: self.position.clone(),
            direction: new_dir,
        }
    }
}

#[derive(Clone)]
struct GuardData {
    past_placements: HashSet<Placement>,
    placement: Placement,
}

impl GuardData {
    fn from_placement(placement: Placement) -> Self {
        GuardData {
            past_placements: HashSet::from([placement.clone()]),
            placement,
        }
    }

    fn update_placement(self: &mut Self, new_placement: Placement) {
        self.past_placements.insert(new_placement.clone());
        self.placement = new_placement;
    }

    fn was_at_placement(self: &Self, placement: &Placement) -> bool {
        self.past_placements.contains(placement)
    }
}

#[derive(Clone)]
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
                    tiles.push(MapTile::Empty);
                }
                b'#' => {
                    tiles.push(MapTile::Obstructed);
                }
                b'X' => {
                    tiles.push(MapTile::Empty);
                }
                b'^' | b'>' | b'v' | b'<' => {
                    let i = tiles.len();
                    let guard_placement = Placement {
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
                    };
                    guard_data = Some(GuardData::from_placement(guard_placement));
                    tiles.push(MapTile::Empty);
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
    fn get_tile_by_idx(self: &Self, index: usize) -> Option<&MapTile> {
        self.tiles.get(index)
    }

    fn get_tile(self: &Self, pos: &Position) -> Option<&MapTile> {
        self.get_tile_by_idx(pos.x + pos.y * self.size.width)
    }

    fn get_mut_tile_by_idx(self: &mut Self, index: usize) -> Option<&mut MapTile> {
        self.tiles.get_mut(index)
    }

    fn get_mut_tile(self: &mut Self, pos: &Position) -> Option<&mut MapTile> {
        self.get_mut_tile_by_idx(pos.x + pos.y * self.size.width)
    }
}

fn will_guard_get_stuck(lab_map: &LabMap, guard_data: &GuardData) -> (bool, HashSet<Placement>) {
    let mut guard_data = guard_data.clone();
    loop {
        if let Some(new_placement) = guard_data.placement.move_forward(&lab_map.size) {
            match lab_map.get_tile(&new_placement.position) {
                Some(MapTile::Empty) => {
                    if guard_data.was_at_placement(&new_placement) {
                        return (true, guard_data.past_placements);
                    }
                    guard_data.update_placement(new_placement);
                }
                Some(MapTile::Obstructed) => {
                    let new_placement = guard_data.placement.rotate_90();
                    guard_data.update_placement(new_placement);
                }
                None => unreachable!(),
            }
        } else {
            // left the mapped area
            return (false, guard_data.past_placements);
        }
    }
}

fn main() {
    let mut distinct_pos = 0u32;
    let mut lab_data = LabData::from_bytes(io::stdin().bytes().filter_map(Result::ok));
    if let Some(ref mut guard_data) = lab_data.guard_data {
        let (_, initial_placements) = will_guard_get_stuck(&lab_data.map, guard_data);
        let initial_positions: HashSet<Position> = initial_placements.iter().map(|placement| placement.position.clone()).collect();
        let total = initial_positions.len();
        for (i, position) in initial_positions.iter().enumerate() {
            println!("Trying with obstacle at position {i} of {total}");
            let mut candidate_lab_map = lab_data.map.clone();
            *candidate_lab_map.get_mut_tile(&position).unwrap() = MapTile::Obstructed;
            if will_guard_get_stuck(&candidate_lab_map, guard_data).0 {
                distinct_pos += 1;
            }
        }
        println!("{distinct_pos}");
    } else {
        eprintln!("No guard found on map");
    }
}
