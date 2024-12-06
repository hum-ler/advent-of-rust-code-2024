use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
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

const INPUT_FILE: &str = "inputs/day-6.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<usize> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<usize> {
    part_2(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<usize> {
    part_2(&file_to_lines(INPUT_FILE)?)
}

fn part_1(input: &[String]) -> Result<usize> {
    let (mut guard, mut grid, grid_size) = parse_input(input)?;

    while guard.0.is_some() {
        traverse(&mut guard, &mut grid, &grid_size)?;
    }

    Ok(grid.values().filter(|t| **t == CoordType::Visited).count())
}

fn part_2(input: &[String]) -> Result<usize> {
    let (guard, grid, grid_size) = parse_input_with_direction(input)?;

    count_obstacle_positions(&guard, &grid, &grid_size)
}

// Direction that the guard is facing.
#[derive(Clone)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Coord = (usize, usize);

type Guard = (Option<Coord>, Direction);

type GridSize = (usize, usize);

/// Grid values for part 1.
#[derive(PartialEq)]
enum CoordType {
    Obstacle,
    Free,
    Visited,
}

/// Parses [input] into a [Guard], a map of coords against terrain type, and the size of the grid.
fn parse_input(input: &[String]) -> Result<(Guard, HashMap<Coord, CoordType>, GridSize)> {
    let mut guard = (None, Direction::N);
    let mut grid = HashMap::new();

    let grid_size = (input.len(), input[0].len());

    input.iter().enumerate().try_for_each(|(row, l)| {
        l.as_bytes().iter().enumerate().try_for_each(|(column, b)| {
            match b {
                b'#' => {
                    grid.insert((row, column), CoordType::Obstacle);
                }
                b'.' => {
                    grid.insert((row, column), CoordType::Free);
                }
                b'^' => {
                    grid.insert((row, column), CoordType::Visited);
                    guard = (Some((row, column)), Direction::N);
                }
                x => {
                    return Err(anyhow!("Unexpect input: {}", x));
                }
            }

            Ok(())
        })
    })?;

    Ok((guard, grid, grid_size))
}

/// Moves the guard one step, or turns the guard 90 degrees if blocked.
///
/// If the guard leaves the grid, its coords will be set to None.
fn traverse(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    grid_size: &GridSize,
) -> Result<()> {
    match guard.1 {
        Direction::N => traverse_n(guard, grid)?,
        Direction::E => traverse_e(guard, grid, grid_size.1)?,
        Direction::S => traverse_s(guard, grid, grid_size.0)?,
        Direction::W => traverse_w(guard, grid)?,
    }

    Ok(())
}

fn traverse_n(guard: &mut Guard, grid: &mut HashMap<Coord, CoordType>) -> Result<()> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 == 0 {
        // Exiting the grid.
        *guard = (None, Direction::N);
    } else {
        match grid[&(coord.0 - 1, coord.1)] {
            CoordType::Obstacle => {
                *guard = (Some((coord.0, coord.1)), Direction::E);
            }
            CoordType::Free => {
                grid.insert((coord.0 - 1, coord.1), CoordType::Visited);
                *guard = (Some((coord.0 - 1, coord.1)), Direction::N);
            }
            CoordType::Visited => {
                *guard = (Some((coord.0 - 1, coord.1)), Direction::N);
            }
        }
    }

    Ok(())
}

fn traverse_e(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    column_count: usize,
) -> Result<()> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 >= column_count - 1 {
        // Exiting the grid.
        *guard = (None, Direction::E);
    } else {
        match grid[&(coord.0, coord.1 + 1)] {
            CoordType::Obstacle => {
                *guard = (Some((coord.0, coord.1)), Direction::S);
            }
            CoordType::Free => {
                grid.insert((coord.0, coord.1 + 1), CoordType::Visited);
                *guard = (Some((coord.0, coord.1 + 1)), Direction::E);
            }
            CoordType::Visited => {
                *guard = (Some((coord.0, coord.1 + 1)), Direction::E);
            }
        }
    }

    Ok(())
}

fn traverse_s(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    row_count: usize,
) -> Result<()> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 >= row_count - 1 {
        // Exiting the grid.
        *guard = (None, Direction::S);
    } else {
        match grid[&(coord.0 + 1, coord.1)] {
            CoordType::Obstacle => {
                *guard = (Some((coord.0, coord.1)), Direction::W);
            }
            CoordType::Free => {
                grid.insert((coord.0 + 1, coord.1), CoordType::Visited);
                *guard = (Some((coord.0 + 1, coord.1)), Direction::S);
            }
            CoordType::Visited => {
                *guard = (Some((coord.0 + 1, coord.1)), Direction::S);
            }
        }
    }

    Ok(())
}

fn traverse_w(guard: &mut Guard, grid: &mut HashMap<Coord, CoordType>) -> Result<()> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 == 0 {
        // Exiting the grid.
        *guard = (None, Direction::W);
    } else {
        match grid[&(coord.0, coord.1 - 1)] {
            CoordType::Obstacle => {
                *guard = (Some((coord.0, coord.1)), Direction::N);
            }
            CoordType::Free => {
                grid.insert((coord.0, coord.1 - 1), CoordType::Visited);
                *guard = (Some((coord.0, coord.1 - 1)), Direction::W);
            }
            CoordType::Visited => {
                *guard = (Some((coord.0, coord.1 - 1)), Direction::W);
            }
        }
    }

    Ok(())
}

/// Grid values for part 2.
#[derive(Clone, PartialEq)]
enum CoordTypeWithDirection {
    Obstacle,
    Free,
    VisitedNorth,
    VisitedEast,
    VisitedSouth,
    VisitedWest,
}

/// The outcome from calling [traverse_with_direction].
#[derive(PartialEq)]
enum TraversalOutcome {
    Continue,
    Exited,
    Loop,
}

fn parse_input_with_direction(
    input: &[String],
) -> Result<(Guard, HashMap<Coord, CoordTypeWithDirection>, GridSize)> {
    let mut guard = (None, Direction::N);
    let mut grid = HashMap::new();

    let grid_size = (input.len(), input[0].len());

    input.iter().enumerate().try_for_each(|(row, l)| {
        l.as_bytes().iter().enumerate().try_for_each(|(column, b)| {
            match b {
                b'#' => {
                    grid.insert((row, column), CoordTypeWithDirection::Obstacle);
                }
                b'.' => {
                    grid.insert((row, column), CoordTypeWithDirection::Free);
                }
                b'^' => {
                    grid.insert((row, column), CoordTypeWithDirection::VisitedNorth);
                    guard = (Some((row, column)), Direction::N);
                }
                x => {
                    return Err(anyhow!("Unexpect input: {}", x));
                }
            }

            Ok(())
        })
    })?;

    Ok((guard, grid, grid_size))
}

/// Counts the number of position where new obstacles can cause loops.
///
/// Loops through each free position, places an obstacle, and then runs the traversal simulation.
fn count_obstacle_positions(
    guard: &Guard,
    grid: &HashMap<Coord, CoordTypeWithDirection>,
    grid_size: &GridSize,
) -> Result<usize> {
    let mut count = 0;

    for row in 0..grid_size.0 {
        for column in 0..grid_size.1 {
            if grid[&(row, column)] != CoordTypeWithDirection::Free {
                continue;
            }

            // Set up this iteration.
            let mut guard = (guard.0, guard.1.clone());
            let mut grid = grid.clone();
            grid.insert((row, column), CoordTypeWithDirection::Obstacle);

            loop {
                match traverse_with_direction(&mut guard, &mut grid, grid_size)? {
                    TraversalOutcome::Continue => continue,
                    TraversalOutcome::Exited => break,
                    TraversalOutcome::Loop => {
                        count += 1;
                        break;
                    }
                }
            }
        }
    }

    Ok(count)
}

fn traverse_with_direction(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordTypeWithDirection>,
    grid_size: &GridSize,
) -> Result<TraversalOutcome> {
    match guard.1 {
        Direction::N => traverse_n_with_direction(guard, grid),
        Direction::E => traverse_e_with_direction(guard, grid, grid_size.1),
        Direction::S => traverse_s_with_direction(guard, grid, grid_size.0),
        Direction::W => traverse_w_with_direction(guard, grid),
    }
}

fn traverse_n_with_direction(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordTypeWithDirection>,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 == 0 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0 - 1, coord.1)] {
        CoordTypeWithDirection::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::E);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::Free
        | CoordTypeWithDirection::VisitedEast
        | CoordTypeWithDirection::VisitedSouth
        | CoordTypeWithDirection::VisitedWest => {
            grid.insert((coord.0 - 1, coord.1), CoordTypeWithDirection::VisitedNorth);
            *guard = (Some((coord.0 - 1, coord.1)), Direction::N);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::VisitedNorth => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_e_with_direction(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordTypeWithDirection>,
    column_count: usize,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 >= column_count - 1 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0, coord.1 + 1)] {
        CoordTypeWithDirection::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::S);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::Free
        | CoordTypeWithDirection::VisitedNorth
        | CoordTypeWithDirection::VisitedSouth
        | CoordTypeWithDirection::VisitedWest => {
            grid.insert((coord.0, coord.1 + 1), CoordTypeWithDirection::VisitedEast);
            *guard = (Some((coord.0, coord.1 + 1)), Direction::E);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::VisitedEast => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_s_with_direction(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordTypeWithDirection>,
    row_count: usize,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 >= row_count - 1 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0 + 1, coord.1)] {
        CoordTypeWithDirection::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::W);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::Free
        | CoordTypeWithDirection::VisitedNorth
        | CoordTypeWithDirection::VisitedEast
        | CoordTypeWithDirection::VisitedWest => {
            grid.insert((coord.0 + 1, coord.1), CoordTypeWithDirection::VisitedSouth);
            *guard = (Some((coord.0 + 1, coord.1)), Direction::S);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::VisitedSouth => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_w_with_direction(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordTypeWithDirection>,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 == 0 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0, coord.1 - 1)] {
        CoordTypeWithDirection::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::N);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::Free
        | CoordTypeWithDirection::VisitedNorth
        | CoordTypeWithDirection::VisitedEast
        | CoordTypeWithDirection::VisitedSouth => {
            grid.insert((coord.0, coord.1 - 1), CoordTypeWithDirection::VisitedWest);
            *guard = (Some((coord.0, coord.1 - 1)), Direction::W);
            Ok(TraversalOutcome::Continue)
        }
        CoordTypeWithDirection::VisitedWest => Ok(TraversalOutcome::Loop),
    }
}
