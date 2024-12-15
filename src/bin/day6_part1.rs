use std::io::{self, Read};

#[derive(Copy, Clone)]
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

fn rotate_90(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn move_in_dir(
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    dir: Direction,
) -> Option<(usize, usize)> {
    match dir {
        Direction::Up => {
            if y >= 1 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        Direction::Right => {
            if x < width - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        }
        Direction::Down => {
            if y < height - 1 {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Direction::Left => {
            if x >= 1 {
                Some((x - 1, y))
            } else {
                None
            }
        }
    }
}

fn main() {
    let mut distinct_pos = 1u32;
    let mut first_line_width: Option<usize> = None;
    let mut orig_guard_dir: Option<Direction> = None;
    let mut orig_guard_idx: Option<usize> = None;
    let mut tiles: Vec<MapTile> = Vec::new();
    for byte in io::stdin().bytes().filter_map(Result::ok) {
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
                orig_guard_dir = match byte {
                    b'^' => Some(Direction::Up),
                    b'>' => Some(Direction::Right),
                    b'v' => Some(Direction::Down),
                    b'<' => Some(Direction::Left),
                    _ => unreachable!(),
                };
                orig_guard_idx = Some(tiles.len());
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
    let mut guard_dir = orig_guard_dir.unwrap();
    let mut guard_x = orig_guard_idx.unwrap() % width;
    let mut guard_y = orig_guard_idx.unwrap() / width;
    loop {
        if let Some((new_x, new_y)) = move_in_dir(width, height, guard_x, guard_y, guard_dir) {
            let new_idx = new_x + new_y * width;
            match &mut tiles[new_idx] {
                MapTile::Empty { visited } => {
                    guard_x = new_x;
                    guard_y = new_y;
                    if !*visited {
                        distinct_pos += 1;
                        *visited = true;
                    }
                }
                MapTile::Obstructed => {
                    guard_dir = rotate_90(guard_dir);
                }
            }
        } else {
            // left the mapped area
            break;
        }
    }
    println!("{distinct_pos}")
}
