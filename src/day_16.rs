use anyhow::{anyhow, Result};
use itertools::Itertools;
use pathfinding::prelude::{astar_bag_collect, dijkstra};

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

const INPUT_FILE: &str = "inputs/day-16.txt";

pub fn run_example_1() -> Result<u32> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<u32> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<usize> {
    part_2(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<usize> {
    part_2(&file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: &[String]) -> Result<u32> {
    let (maze, start, end) = parse_lines_to_maze(lines)?;

    let start = (start, Facing::East);

    if let Some((_, cost)) = dijkstra(&start, |n| successors(n, &maze), |n| n.0 == end) {
        Ok(cost)
    } else {
        Err(anyhow!("Cannot find shortest path"))
    }
}

fn part_2(lines: &[String]) -> Result<usize> {
    let (maze, start, end) = parse_lines_to_maze(lines)?;

    let start = (start, Facing::East);

    // Note: a heuristic of 0 is essentially only just as good as Dijkstra.
    let paths = astar_bag_collect(&start, |n| successors(n, &maze), |_| 0, |n| n.0 == end);

    if let Some(paths_with_cost) = paths {
        Ok(paths_with_cost
            .0
            .iter()
            .flat_map(|p| p.iter().map(|n| n.0).collect::<Vec<_>>())
            .unique()
            .count())
    } else {
        Err(anyhow!("Cannot find all shortest paths"))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    North,
    East,
    South,
    West,
}

type Coord = (usize, usize);

type Node = (Coord, Facing);

/// Parse the input into the maze grid, and returns the start and end coordinates.
fn parse_lines_to_maze(lines: &[String]) -> Result<(Vec<Vec<u8>>, Coord, Coord)> {
    let mut maze = lines
        .iter()
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<Vec<_>>>();

    // Find the start coordinates.
    let Some(start_row) = maze.iter().position(|r| r.contains(&b'S')) else {
        return Err(anyhow!("Cannot locate start row"));
    };
    let Some(start_col) = maze[start_row].iter().position(|c| c == &b'S') else {
        return Err(anyhow!("Cannot locate start col"));
    };

    // Find the end coordinates.
    let Some(end_row) = maze.iter().position(|r| r.contains(&b'E')) else {
        return Err(anyhow!("Cannot locate end row"));
    };
    let Some(end_col) = maze[end_row].iter().position(|c| c == &b'E') else {
        return Err(anyhow!("Cannot locate end col"));
    };

    // Clear the 'S' and 'E' symbols.
    maze[start_row][start_col] = b'.';
    maze[end_row][end_col] = b'.';

    Ok((maze, (start_row, start_col), (end_row, end_col)))
}

/// Finds connected nodes from [node].
fn successors(node: &Node, maze: &[Vec<u8>]) -> Vec<(Node, u32)> {
    let &(coord, facing) = node;

    assert!(coord.0 > 0);
    assert!(coord.1 > 0);

    match facing {
        Facing::North => {
            let mut nodes = vec![((coord, Facing::West), 1000), ((coord, Facing::East), 1000)];

            if maze[coord.0 - 1][coord.1] == b'.' {
                nodes.push((((coord.0 - 1, coord.1), Facing::North), 1));
            }

            nodes
        }
        Facing::East => {
            let mut nodes = vec![
                ((coord, Facing::North), 1000),
                ((coord, Facing::South), 1000),
            ];

            if maze[coord.0][coord.1 + 1] == b'.' {
                nodes.push((((coord.0, coord.1 + 1), Facing::East), 1));
            }

            nodes
        }
        Facing::South => {
            let mut nodes = vec![((coord, Facing::East), 1000), ((coord, Facing::West), 1000)];

            if maze[coord.0 + 1][coord.1] == b'.' {
                nodes.push((((coord.0 + 1, coord.1), Facing::South), 1));
            }

            nodes
        }
        Facing::West => {
            let mut nodes = vec![
                ((coord, Facing::South), 1000),
                ((coord, Facing::North), 1000),
            ];

            if maze[coord.0][coord.1 - 1] == b'.' {
                nodes.push((((coord.0, coord.1 - 1), Facing::West), 1));
            }

            nodes
        }
    }
}
