use std::collections::HashSet;

use anyhow::{Result, anyhow};
use pathfinding::prelude::dijkstra;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-18.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    shortest_path_through_grid(input, 71, 1024)
}

fn part_2(input: &str) -> Result<String> {
    first_blocking_byte(input, 71, 1024)
}

/// (x, y)
type Coord = (usize, usize);

fn shortest_path_through_grid(bytes: &str, grid_size: usize, bytes_count: usize) -> Result<u32> {
    let grid = bytes
        .lines()
        .take(bytes_count)
        .map(|line| {
            let Some((x, y)) = line.split_once(",") else {
                return Err(anyhow!("Cannot split input into x and y: {}", line));
            };

            Ok((x.parse()?, y.parse()?))
        })
        .collect::<Result<HashSet<Coord>>>()?;

    dijkstra(
        &(0, 0),
        |node| successors(node, &grid, grid_size),
        |node| *node == (grid_size - 1, grid_size - 1),
    )
    .map(|shortest_path| shortest_path.1)
    .ok_or(anyhow!("Cannot find shortest path"))
}

fn successors(coord: &Coord, grid: &HashSet<Coord>, grid_size: usize) -> Vec<(Coord, u32)> {
    let mut successors = Vec::new();

    let &(x, y) = coord;
    if y > 0 && !grid.contains(&(x, y - 1)) {
        successors.push(((x, y - 1), 1));
    }
    if x < grid_size - 1 && !grid.contains(&(x + 1, y)) {
        successors.push(((x + 1, y), 1));
    }
    if y < grid_size - 1 && !grid.contains(&(x, y + 1)) {
        successors.push(((x, y + 1), 1));
    }
    if x > 0 && !grid.contains(&(x - 1, y)) {
        successors.push(((x - 1, y), 1));
    }

    successors
}

/// Finds the first byte that blocks all path between S and E.
///
/// Use skip_bytes_count (bytes_count from part 1) to skip over the blocked path checks where we
/// know for certain that an unhindered path exists.
fn first_blocking_byte(bytes: &str, grid_size: usize, skip_bytes_count: usize) -> Result<String> {
    let bytes = parse_input_into_bytes(bytes)?;

    bytes
        .into_iter()
        .scan(HashSet::new(), |state, coord| {
            state.insert(coord);

            Some((state.clone(), coord))
        })
        .skip(skip_bytes_count)
        .find(|(grid, _)| {
            dijkstra(
                &(0, 0),
                |node| successors(node, grid, grid_size),
                |node| *node == (grid_size - 1, grid_size - 1),
            )
            .is_none()
        })
        .map(|(_, coord)| format!("{},{}", coord.0, coord.1))
        .ok_or(anyhow!("Cannot find first byte that blocks path"))
}

fn parse_input_into_bytes(input: &str) -> Result<Vec<Coord>> {
    input
        .lines()
        .map(|line| {
            let Some((x, y)) = line.split_once(",") else {
                return Err(anyhow!("Cannot split input into x and y: {}", line));
            };

            Ok((x.parse()?, y.parse()?))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            shortest_path_through_grid(trim_newlines(EXAMPLE), 7, 12)?,
            22
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(first_blocking_byte(trim_newlines(EXAMPLE), 7, 12)?, "6,1");

        Ok(())
    }
}
