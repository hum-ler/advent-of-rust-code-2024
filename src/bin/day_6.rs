use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-6.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (mut guard, mut grid, grid_size) = parse_input(input)?;

    while TraversalOutcome::Exited != traverse(&mut guard, &mut grid, &grid_size)? {}

    Ok(grid
        .values()
        .filter(|t| match t {
            CoordType::Obstacle | CoordType::Free => false,
            CoordType::VisitedNorth
            | CoordType::VisitedEast
            | CoordType::VisitedSouth
            | CoordType::VisitedWest => true,
        })
        .count())
}

fn part_2(input: String) -> Result<usize> {
    // Make use of the result from part 1.
    let potential_positions = list_potential_positions(input.clone())?;

    let (guard, grid, grid_size) = parse_input(input)?;

    count_obstacle_positions(&potential_positions, &guard, &grid, &grid_size)
}

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

#[derive(Clone, PartialEq)]
enum CoordType {
    Obstacle,
    Free,
    VisitedNorth,
    VisitedEast,
    VisitedSouth,
    VisitedWest,
}

#[derive(PartialEq)]
enum TraversalOutcome {
    /// Traversal should continue.
    Continue,

    /// Guard has exited the grid.
    Exited,

    /// Guard has entered a [Coord] visited previously from the same [Direction].
    Loop,
}

/// Derives the list of potential positions for the obstacle.
///
/// The positions are those visited by the guard before introducing the additional obstacle.
fn list_potential_positions(input: String) -> Result<Vec<Coord>> {
    let (mut guard, mut grid, grid_size) = parse_input(input)?;

    // The starting location of the guard will also be marked as CoordType::VisitedNorth.
    let Some(guard_starting_position) = guard.0 else {
        return Err(anyhow!("Invalid guard starting position"));
    };

    while TraversalOutcome::Exited != traverse(&mut guard, &mut grid, &grid_size)? {}

    Ok(grid
        .into_iter()
        .filter_map(|(coord, coord_type)| {
            if coord == guard_starting_position {
                return None;
            }

            match coord_type {
                CoordType::Obstacle | CoordType::Free => None,
                CoordType::VisitedNorth
                | CoordType::VisitedEast
                | CoordType::VisitedSouth
                | CoordType::VisitedWest => Some(coord),
            }
        })
        .collect())
}

/// Parses input into a [Guard], a map of [Coord]s against terrain type, and the size of the grid.
fn parse_input(input: String) -> Result<(Guard, HashMap<Coord, CoordType>, GridSize)> {
    let mut guard = (None, Direction::N);
    let mut grid = HashMap::new();

    let input = input.split_terminator("\n").collect::<Vec<_>>();
    let grid_size = (input.len(), input[0].len());

    for (row, line) in input.iter().enumerate() {
        for (col, byte) in line.as_bytes().iter().enumerate() {
            match byte {
                b'#' => {
                    grid.insert((row, col), CoordType::Obstacle);
                }
                b'.' => {
                    grid.insert((row, col), CoordType::Free);
                }
                b'^' => {
                    grid.insert((row, col), CoordType::VisitedNorth);
                    guard = (Some((row, col)), Direction::N);
                }
                x => {
                    return Err(anyhow!("Unexpect input: {}", x));
                }
            }
        }
    }

    Ok((guard, grid, grid_size))
}

/// Counts the number of position where new obstacles can cause loops.
///
/// Loops through each free position, places an obstacle, and then runs the traversal simulation.
fn count_obstacle_positions(
    potential_positions: &[Coord],
    guard: &Guard,
    grid: &HashMap<Coord, CoordType>,
    grid_size: &GridSize,
) -> Result<usize> {
    let mut count = 0;

    for &(row, col) in potential_positions {
        // Set up this iteration.
        let mut guard = (guard.0, guard.1.clone());
        let mut grid = grid.clone();
        grid.insert((row, col), CoordType::Obstacle);

        loop {
            match traverse(&mut guard, &mut grid, grid_size)? {
                TraversalOutcome::Continue => continue,
                TraversalOutcome::Exited => break,
                TraversalOutcome::Loop => {
                    count += 1;
                    break;
                }
            }
        }
    }

    Ok(count)
}

/// Moves the guard one step, or turns the guard 90 degrees if blocked.
///
/// In normal traversal, the guard and grid are updated accordingly, and
/// [TraversalOutcome::Continue] is returned.
///
/// [TraversalOutcome::Exited] and [TraversalOutcome::Loop] returns immediately -- the guard and
/// grid are not modified.
fn traverse(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    grid_size: &GridSize,
) -> Result<TraversalOutcome> {
    match guard.1 {
        Direction::N => traverse_n(guard, grid),
        Direction::E => traverse_e(guard, grid, grid_size.1),
        Direction::S => traverse_s(guard, grid, grid_size.0),
        Direction::W => traverse_w(guard, grid),
    }
}

fn traverse_n(guard: &mut Guard, grid: &mut HashMap<Coord, CoordType>) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 == 0 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0 - 1, coord.1)] {
        CoordType::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::E);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::Free
        | CoordType::VisitedEast
        | CoordType::VisitedSouth
        | CoordType::VisitedWest => {
            grid.entry((coord.0 - 1, coord.1))
                .and_modify(|ct| *ct = CoordType::VisitedNorth);
            *guard = (Some((coord.0 - 1, coord.1)), Direction::N);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::VisitedNorth => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_e(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    column_count: usize,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 >= column_count - 1 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0, coord.1 + 1)] {
        CoordType::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::S);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::Free
        | CoordType::VisitedNorth
        | CoordType::VisitedSouth
        | CoordType::VisitedWest => {
            grid.entry((coord.0, coord.1 + 1))
                .and_modify(|ct| *ct = CoordType::VisitedEast);
            *guard = (Some((coord.0, coord.1 + 1)), Direction::E);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::VisitedEast => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_s(
    guard: &mut Guard,
    grid: &mut HashMap<Coord, CoordType>,
    row_count: usize,
) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.0 >= row_count - 1 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0 + 1, coord.1)] {
        CoordType::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::W);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::Free
        | CoordType::VisitedNorth
        | CoordType::VisitedEast
        | CoordType::VisitedWest => {
            grid.entry((coord.0 + 1, coord.1))
                .and_modify(|ct| *ct = CoordType::VisitedSouth);
            *guard = (Some((coord.0 + 1, coord.1)), Direction::S);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::VisitedSouth => Ok(TraversalOutcome::Loop),
    }
}

fn traverse_w(guard: &mut Guard, grid: &mut HashMap<Coord, CoordType>) -> Result<TraversalOutcome> {
    let Some(coord) = guard.0 else {
        return Err(anyhow!("Coord cannot be None"));
    };

    if coord.1 == 0 {
        return Ok(TraversalOutcome::Exited);
    }

    match grid[&(coord.0, coord.1 - 1)] {
        CoordType::Obstacle => {
            *guard = (Some((coord.0, coord.1)), Direction::N);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::Free
        | CoordType::VisitedNorth
        | CoordType::VisitedEast
        | CoordType::VisitedSouth => {
            grid.entry((coord.0, coord.1 - 1))
                .and_modify(|ct| *ct = CoordType::VisitedWest);
            *guard = (Some((coord.0, coord.1 - 1)), Direction::W);
            Ok(TraversalOutcome::Continue)
        }
        CoordType::VisitedWest => Ok(TraversalOutcome::Loop),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 41);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 6);

        Ok(())
    }
}
