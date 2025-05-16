use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};
use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-20.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    count_shortcuts(input, 2, 100)
}

fn part_2(input: &str) -> Result<usize> {
    count_shortcuts(input, 20, 100)
}

/// (row, col)
type Coord = (usize, usize);

struct Grid {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut walls = HashSet::new();
        for (row, line) in lines.into_iter().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                match byte {
                    b'S' => start = (row, col),
                    b'E' => end = (row, col),
                    b'#' => {
                        walls.insert((row, col));
                    }
                    _ => (),
                }
            }
        }

        Ok(Self { walls, start, end })
    }
}

fn count_shortcuts(grid: &str, max_shortcut: usize, min_savings: usize) -> Result<usize> {
    let grid = Grid::from_str(grid)?;

    let Some((shortest_path, _)) = dijkstra(
        &grid.start,
        |node| successors(node, &grid),
        |node| *node == grid.end,
    ) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok((0..shortest_path.len())
        .zip(shortest_path)
        .tuple_combinations()
        .filter(|((index_1, coord_1), (index_2, coord_2))| {
            // If the shortcut runs along the shortest path, savings will be 0.

            let shortcut = manhatten_distance(coord_1, coord_2);
            let savings = index_1.abs_diff(*index_2) - shortcut;

            shortcut <= max_shortcut && savings >= min_savings
        })
        .count())
}

fn successors(coord: &Coord, grid: &Grid) -> Vec<(Coord, u32)> {
    let &(row, col) = coord;

    [
        (row - 1, col),
        (row, col + 1),
        (row + 1, col),
        (row, col - 1),
    ]
    .into_iter()
    .filter_map(|neighbour| {
        if !grid.walls.contains(&neighbour) {
            Some((neighbour, 1))
        } else {
            None
        }
    })
    .collect()
}

fn manhatten_distance(coord: &Coord, other: &Coord) -> usize {
    coord.0.abs_diff(other.0) + coord.1.abs_diff(other.1)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(count_shortcuts(trim_newlines(EXAMPLE), 2, 2)?, 44);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(count_shortcuts(trim_newlines(EXAMPLE), 20, 50)?, 285);

        Ok(())
    }
}
