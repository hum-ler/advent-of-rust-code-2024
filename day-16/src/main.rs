use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};
use pathfinding::prelude::{astar, astar_bag};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-16.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let grid = Grid::from_str(input)?;

    let start = ((grid.size - 2, 1), Direction::E);
    let end_coord = (1, grid.size - 2);

    astar(
        &start,
        |node| successors(node, &grid),
        |_| 0,
        |node| node.0 == end_coord,
    )
    .map(|shortest_path| shortest_path.1)
    .ok_or(anyhow!("Cannot find shortest path"))
}

fn part_2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;

    let start = ((grid.size - 2, 1), Direction::E);
    let end_coord = (1, grid.size - 2);

    let Some((shortest_paths, _)) = astar_bag(
        &start,
        |node| successors(node, &grid),
        |_| 0,
        |node| node.0 == end_coord,
    ) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(shortest_paths
        .into_iter()
        .flatten()
        .map(|node| node.0)
        .collect::<HashSet<_>>()
        .len())
}

struct Grid {
    layout: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let layout = lines
            .into_iter()
            .map(|line| {
                line.bytes()
                    .map(|byte| {
                        if matches!(byte, b'S' | b'E') {
                            b'.'
                        } else {
                            byte
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(Self { layout, size })
    }
}

/// (row, col)
type Coord = (usize, usize);

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

type Node = (Coord, Direction);

fn successors(node: &Node, grid: &Grid) -> Vec<(Node, u32)> {
    let mut successors = Vec::new();

    let &((row, col), direction) = node;
    match direction {
        Direction::N => {
            // Move forward.
            if grid.layout[row - 1][col] == b'.' {
                successors.push((((row - 1, col), Direction::N), 1));
            }

            // Turn 90 degrees.
            successors.push((((row, col), Direction::W), 1000));
            successors.push((((row, col), Direction::E), 1000));
        }
        Direction::E => {
            if grid.layout[row][col + 1] == b'.' {
                successors.push((((row, col + 1), Direction::E), 1));
            }

            successors.push((((row, col), Direction::N), 1000));
            successors.push((((row, col), Direction::S), 1000));
        }
        Direction::S => {
            if grid.layout[row + 1][col] == b'.' {
                successors.push((((row + 1, col), Direction::S), 1));
            }

            successors.push((((row, col), Direction::E), 1000));
            successors.push((((row, col), Direction::W), 1000));
        }
        Direction::W => {
            if grid.layout[row][col - 1] == b'.' {
                successors.push((((row, col - 1), Direction::W), 1));
            }

            successors.push((((row, col), Direction::S), 1000));
            successors.push((((row, col), Direction::N), 1000));
        }
    }

    successors
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_A: &str = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    const EXAMPLE_B: &str = r"
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

    #[test]
    fn example_1a() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_A))?, 7036);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_B))?, 11048);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_A))?, 45);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_B))?, 64);

        Ok(())
    }
}
