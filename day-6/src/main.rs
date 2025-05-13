use std::collections::HashSet;

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-6.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (grid, guard) = parse_input_into_grid_and_guard(input);

    let footprints = footprints(guard, &grid);

    Ok(footprints.len())
}

fn part_2(input: &str) -> Result<u32> {
    let (grid, guard) = parse_input_into_grid_and_guard(input);

    let mut loop_obstacles = 0;

    // The guard is diverted only if we place an obstacle on the original path.
    for coord in footprints(guard, &grid) {
        if guard.pos == coord {
            continue;
        }

        let mut test_grid = grid.clone();
        test_grid.obstacles.insert(coord);
        if is_loop(guard, test_grid) {
            loop_obstacles += 1;
        }
    }

    Ok(loop_obstacles)
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

/// (row, col)
type Coord = (usize, usize);

#[derive(Clone, Default)]
struct Grid {
    obstacles: HashSet<Coord>,
    size: usize,
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
struct Guard {
    pos: Coord,
    facing: Direction,
}

impl Guard {
    fn proceed(mut self, grid: &Grid) -> Option<Self> {
        match self.facing {
            Direction::Up => {
                if self.pos.0 == 0 {
                    return None;
                }

                if grid.obstacles.contains(&(self.pos.0 - 1, self.pos.1)) {
                    self.facing = self.facing.turn_right();
                } else {
                    self.pos = (self.pos.0 - 1, self.pos.1);
                }
            }
            Direction::Right => {
                if self.pos.1 == grid.size - 1 {
                    return None;
                }

                if grid.obstacles.contains(&(self.pos.0, self.pos.1 + 1)) {
                    self.facing = self.facing.turn_right();
                } else {
                    self.pos = (self.pos.0, self.pos.1 + 1);
                }
            }
            Direction::Down => {
                if self.pos.0 == grid.size - 1 {
                    return None;
                }

                if grid.obstacles.contains(&(self.pos.0 + 1, self.pos.1)) {
                    self.facing = self.facing.turn_right();
                } else {
                    self.pos = (self.pos.0 + 1, self.pos.1);
                }
            }
            Direction::Left => {
                if self.pos.1 == 0 {
                    return None;
                }

                if grid.obstacles.contains(&(self.pos.0, self.pos.1 - 1)) {
                    self.facing = self.facing.turn_right();
                } else {
                    self.pos = (self.pos.0, self.pos.1 - 1);
                }
            }
        }

        Some(self)
    }
}

fn parse_input_into_grid_and_guard(input: &str) -> (Grid, Guard) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut grid = Grid {
        size: lines.len(),
        ..Default::default()
    };
    let mut guard = Guard::default();

    for (row, line) in lines.into_iter().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => {
                    grid.obstacles.insert((row, col));
                }
                b'^' => {
                    guard = Guard {
                        pos: (row, col),
                        facing: Direction::Up,
                    }
                }
                _ => (),
            }
        }
    }

    (grid, guard)
}

fn footprints(mut guard: Guard, grid: &Grid) -> HashSet<Coord> {
    let mut footprints = HashSet::new();
    footprints.insert(guard.pos);

    while let Some(new_guard) = guard.proceed(grid) {
        guard = new_guard;
        footprints.insert(guard.pos);
    }

    footprints
}

fn is_loop(mut guard: Guard, grid: Grid) -> bool {
    let mut footprints = HashSet::new();
    footprints.insert(guard);

    while let Some(new_guard) = guard.proceed(&grid) {
        guard = new_guard;

        if !footprints.insert(guard) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 41);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 6);

        Ok(())
    }
}
