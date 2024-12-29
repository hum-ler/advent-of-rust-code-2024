use anyhow::{anyhow, Result};
use pathfinding::prelude::dijkstra;

const INPUT_FILE: &str = "inputs/day-18.txt";

const INPUT_GRID_SIZE: usize = 71;

const INPUT_BYTE_COUNT: usize = 1024;

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    part_1_with_sizes(input, INPUT_GRID_SIZE, Some(INPUT_BYTE_COUNT))
}

fn part_2(input: String) -> Result<String> {
    part_2_with_sizes(input, INPUT_GRID_SIZE, INPUT_BYTE_COUNT)
}

fn part_1_with_sizes(input: String, grid_size: usize, input_size: Option<usize>) -> Result<usize> {
    let grid = parse_input_to_grid(input, grid_size, input_size)?;

    let Some((shortest_path, _)) = dijkstra(
        &(0, 0),
        |n| find_successors(n, &grid, grid_size),
        |n| *n == (grid_size - 1, grid_size - 1),
    ) else {
        return Err(anyhow!("Cannot find shortest path"));
    };

    Ok(shortest_path.len() - 1)
}

fn part_2_with_sizes(input: String, grid_size: usize, skip_checks: usize) -> Result<String> {
    let nodes = parse_input_to_nodes(input, grid_size)?;

    let blocking_node = first_blocker(&nodes, grid_size, skip_checks)?;

    Ok(format!("{},{}", blocking_node.0, blocking_node.1))
}

/// Parses input into a grid of free space and "bytes".
///
/// Only the first [input_size] lines (terminated by '\n') in [input] are processed in creating the
/// grid. If [input_size] is None, then the entire [input] is used.
fn parse_input_to_grid(
    input: String,
    grid_size: usize,
    input_size: Option<usize>,
) -> Result<Vec<Vec<u8>>> {
    let mut grid = vec![vec![b'.'; grid_size]; grid_size];

    let input = input.split_terminator("\n").collect::<Vec<_>>();

    let input_size = input_size.unwrap_or(input.len());

    input[0..input_size].iter().try_for_each(|b| {
        let b_vec = b
            .split(",")
            .map(str::parse::<usize>)
            .collect::<Result<Vec<usize>, _>>()?;
        if b_vec.len() != 2 {
            return Err(anyhow!("Cannot parse input: {}", b));
        };

        let row = b_vec[1];
        let col = b_vec[0];
        if row >= grid_size || col >= grid_size {
            return Err(anyhow!("Invalid input, out of bounds: {}", b));
        }

        grid[row][col] = b'#';

        Ok(())
    })?;

    Ok(grid)
}

type Node = (usize, usize);

/// Finds nodes that are connected to [node].
fn find_successors(node: &Node, grid: &[Vec<u8>], grid_size: usize) -> Vec<(Node, u32)> {
    let mut nodes = Vec::default();

    let &(col, row) = node;

    // N
    if row > 0 && grid[row - 1][col] == b'.' {
        nodes.push(((col, row - 1), 1));
    }

    // E
    if col < grid_size - 1 && grid[row][col + 1] == b'.' {
        nodes.push(((col + 1, row), 1));
    }

    // S
    if row < grid_size - 1 && grid[row + 1][col] == b'.' {
        nodes.push(((col, row + 1), 1));
    }

    // W
    if col > 0 && grid[row][col - 1] == b'.' {
        nodes.push(((col - 1, row), 1));
    }

    nodes
}

fn parse_input_to_nodes(input: String, grid_size: usize) -> Result<Vec<Node>> {
    input
        .split_terminator("\n")
        .map(|b| {
            let b_vec = b
                .split(",")
                .map(str::parse::<usize>)
                .collect::<Result<Vec<usize>, _>>()?;
            if b_vec.len() != 2 {
                return Err(anyhow!("Cannot parse input: {}", b));
            };

            let row = b_vec[1];
            let col = b_vec[0];
            if row >= grid_size || col >= grid_size {
                return Err(anyhow!("Invalid input, out of bounds: {}", b));
            }

            Ok((col, row))
        })
        .collect()
}

/// Finds the first node in [nodes] that prevents any path from (0, 0) to
/// ([grid_size] - 1, [grid_size] - 1).
///
/// Skips checking for paths for the first [skip_checks] nodes.
fn first_blocker(nodes: &[Node], grid_size: usize, skip_checks: usize) -> Result<Node> {
    let mut grid = vec![vec![b'.'; grid_size]; grid_size];

    for (index, node) in nodes.iter().enumerate() {
        grid[node.1][node.0] = b'#';

        // Make use of part 1 by skipping the checks until INPUT_BYTE_COUNT.
        if index < skip_checks {
            continue;
        }

        // Actually, we just need any path, not the shortest one, so Dijkstra might not be the
        // smartest choice here.
        if dijkstra(
            &(0, 0),
            |n| find_successors(n, &grid, grid_size),
            |n| *n == (grid_size - 1, grid_size - 1),
        )
        .is_none()
        {
            return Ok(*node);
        }
    }

    Err(anyhow!("Cannot find any blocker"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
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

    const EXAMPLE_GRID_SIZE: usize = 7;

    const EXAMPLE_BYTE_COUNT: usize = 12;

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1_with_sizes(
                EXAMPLE_INPUT.trim().to_string(),
                EXAMPLE_GRID_SIZE,
                Some(EXAMPLE_BYTE_COUNT)
            )?,
            22
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            part_2_with_sizes(
                EXAMPLE_INPUT.trim().to_string(),
                EXAMPLE_GRID_SIZE,
                EXAMPLE_BYTE_COUNT
            )?,
            "6,1".to_string()
        );

        Ok(())
    }
}
