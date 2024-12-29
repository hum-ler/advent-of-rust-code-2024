use anyhow::{anyhow, Result};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

const INPUT_FILE: &str = "inputs/day-20.txt";

const PART_1_CHEAT_DURATION: usize = 2;

const PART_2_CHEAT_DURATION: usize = 20;

const TARGET_TIME_SAVED: usize = 100;

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (grid, start, end) = parse_input_to_grid(input)?;

    let Some((path, _)) = dijkstra(&start, |n| successors(n, &grid), |n| *n == end) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(find_shortcuts(
        &path,
        PART_1_CHEAT_DURATION,
        TARGET_TIME_SAVED,
    ))
}

fn part_2(input: String) -> Result<usize> {
    let (grid, start, end) = parse_input_to_grid(input)?;

    let Some((path, _)) = dijkstra(&start, |n| successors(n, &grid), |n| *n == end) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(find_shortcuts(
        &path,
        PART_2_CHEAT_DURATION,
        TARGET_TIME_SAVED,
    ))
}

type Coord = (usize, usize);

fn parse_input_to_grid(input: String) -> Result<(Vec<Vec<u8>>, Coord, Coord)> {
    let mut grid = input
        .split_terminator("\n")
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<Vec<_>>>();

    // Find the start coordinates.
    let Some(start_row) = grid.iter().position(|r| r.contains(&b'S')) else {
        return Err(anyhow!("Cannot locate start row"));
    };
    let Some(start_col) = grid[start_row].iter().position(|c| c == &b'S') else {
        return Err(anyhow!("Cannot locate start col"));
    };

    // Find the end coordinates.
    let Some(end_row) = grid.iter().position(|r| r.contains(&b'E')) else {
        return Err(anyhow!("Cannot locate end row"));
    };
    let Some(end_col) = grid[end_row].iter().position(|c| c == &b'E') else {
        return Err(anyhow!("Cannot locate end col"));
    };

    // Clear the 'S' and 'E' symbols.
    grid[start_row][start_col] = b'.';
    grid[end_row][end_col] = b'.';

    Ok((grid, (start_row, start_col), (end_row, end_col)))
}

/// Finds connected nodes from [node].
fn successors(node: &Coord, grid: &[Vec<u8>]) -> Vec<(Coord, usize)> {
    let &(row, col) = node;

    assert!(row > 0);
    assert!(col > 0);

    let mut nodes: Vec<(Coord, usize)> = Vec::default();

    // N
    if grid[row - 1][col] == b'.' {
        nodes.push(((row - 1, col), 1));
    }

    // E
    if grid[row][col + 1] == b'.' {
        nodes.push(((row, col + 1), 1));
    }

    // S
    if grid[row + 1][col] == b'.' {
        nodes.push(((row + 1, col), 1));
    }

    // W
    if grid[row][col - 1] == b'.' {
        nodes.push(((row, col - 1), 1));
    }

    nodes
}

/// Solves part 1 by scanning for single-thickness walls that can skipped along the shortest path.
fn _part_1_by_finding_walls(input: String) -> Result<usize> {
    let (grid, start, end) = parse_input_to_grid(input)?;

    let Some((path, length)) = dijkstra(&start, |n| successors(n, &grid), |n| *n == end) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    let skippable_walls = _find_skippable_walls(&path, &grid);

    let mut acceptable_skips = 0usize;

    for skip in skippable_walls {
        let mut modified_grid = grid.clone();
        modified_grid[skip.0][skip.1] = b'.';

        if let Some((_, new_length)) =
            dijkstra(&start, |n| successors(n, &modified_grid), |n| *n == end)
        {
            if new_length < length && length - new_length >= TARGET_TIME_SAVED {
                acceptable_skips += 1;
            }
        }
    }

    Ok(acceptable_skips)
}

fn _find_skippable_walls(path: &[Coord], grid: &[Vec<u8>]) -> Vec<Coord> {
    let grid_size = grid.len();

    path.iter()
        .flat_map(|n| _find_skippable_walls_from_node(n, grid, grid_size))
        .unique()
        .collect()
}

fn _find_skippable_walls_from_node(node: &Coord, grid: &[Vec<u8>], grid_size: usize) -> Vec<Coord> {
    // Cutting a corner does not provide any savings, so we are looking at straight line cuts.

    let &(row, col) = node;

    let mut nodes = Vec::default();

    // N
    if row > 1 && grid[row - 1][col] == b'#' && grid[row - 2][col] == b'.' {
        nodes.push((row - 1, col));
    }

    // E
    if col < grid_size - 2 && grid[row][col + 1] == b'#' && grid[row][col + 2] == b'.' {
        nodes.push((row, col + 1));
    }

    // S
    if row < grid_size - 2 && grid[row + 1][col] == b'#' && grid[row + 2][col] == b'.' {
        nodes.push((row + 1, col));
    }

    // W
    if col > 1 && grid[row][col - 1] == b'#' && grid[row][col - 2] == b'.' {
        nodes.push((row, col - 1));
    }

    nodes
}

fn manhattan_distance(node_1: &Coord, node_2: &Coord) -> usize {
    node_1.0.abs_diff(node_2.0) + node_1.1.abs_diff(node_2.1)
}

/// Counts cheats that save at least [minimum_time_saved].
fn find_shortcuts(path: &[Coord], cheat_duration: usize, minimum_time_saved: usize) -> usize {
    // Looks through the [path] and find pairs that can be connected within [cheat_duration] i.e.
    // potential cheats.
    //
    // The difference in the positions of the node pair in [path] is the potential time saved by
    // skipping over the intermediate nodes in [path]. We do need to substract the necessary time it
    // takes to traverse between the pair in the most direct line because the savings are in
    // avoiding the deviations from this line. This also takes care of the case whereby the pair are
    // already in the most direct line from each other in [path].

    path.iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((index_1, node_1), (index_2, node_2))| {
            let index_diff = index_1.abs_diff(*index_2);
            let shortcut = manhattan_distance(node_1, node_2);

            shortcut <= cheat_duration && index_diff - shortcut >= minimum_time_saved
        })
        .count()
}