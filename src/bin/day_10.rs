use anyhow::Result;
use itertools::Itertools;

const INPUT_FILE: &str = "inputs/day-10.txt";

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (grid, grid_size) = parse_input_to_grid(&input);

    Ok(find_trailheads(&grid)
        .iter()
        .map(|h| find_ends(h, &grid, &grid_size).iter().unique().count())
        .sum())
}

fn part_2(input: String) -> Result<usize> {
    let (grid, grid_size) = parse_input_to_grid(&input);

    Ok(find_trailheads(&grid)
        .iter()
        .map(|h| find_ends(h, &grid, &grid_size).len())
        .sum())
}

#[derive(Eq, Hash, PartialEq)]
struct Node {
    value: u8,
    row: usize,
    col: usize,
}

impl Node {
    pub fn new(value: u8, row: usize, col: usize) -> Self {
        Self { value, row, col }
    }
}

type GridSize = (usize, usize);

/// Parses [input] into a grid of [Node]s, and the row- and column counts.
fn parse_input_to_grid(input: &str) -> (Vec<Vec<Node>>, GridSize) {
    let input = input.split_terminator("\n").collect::<Vec<_>>();
    let row_count = input.len();
    let col_count = input.first().map_or(0, |s| s.len());

    let grid = input
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(col, byte)| Node::new(byte - b'0', row, col))
                .collect()
        })
        .collect();

    (grid, (row_count, col_count))
}

/// Finds [Node]s with value = 0.
fn find_trailheads(grid: &[Vec<Node>]) -> Vec<&Node> {
    grid.iter()
        .flat_map(|r| r.iter().filter(|n| n.value == 0))
        .collect()
}

/// Finds adjacent [Node]s that have value 1 greater than [node].
fn find_follow_up_nodes<'a>(
    node: &Node,
    grid: &'a [Vec<Node>],
    grid_size: &GridSize,
) -> Vec<&'a Node> {
    let mut follow_up_nodes = Vec::default();

    if node.value == 9 {
        return follow_up_nodes;
    }

    // N
    if node.row > 0 && grid[node.row - 1][node.col].value == node.value + 1 {
        follow_up_nodes.push(&grid[node.row - 1][node.col]);
    }

    // E
    if node.col < grid_size.1 - 1 && grid[node.row][node.col + 1].value == node.value + 1 {
        follow_up_nodes.push(&grid[node.row][node.col + 1]);
    }

    // S
    if node.row < grid_size.0 - 1 && grid[node.row + 1][node.col].value == node.value + 1 {
        follow_up_nodes.push(&grid[node.row + 1][node.col]);
    }

    // W
    if node.col > 0 && grid[node.row][node.col - 1].value == node.value + 1 {
        follow_up_nodes.push(&grid[node.row][node.col - 1]);
    }

    follow_up_nodes
}

/// Finds all the end [Node]s (i.e. value = 9) that can be reached from [trailhead].
///
/// If the same end node can be reached via multiple paths, it will be repeated in the result.
fn find_ends<'a>(trailhead: &Node, grid: &'a [Vec<Node>], grid_size: &GridSize) -> Vec<&'a Node> {
    let mut ends: Vec<&Node> = Vec::default();

    let mut trail_check = find_follow_up_nodes(trailhead, grid, grid_size);

    while let Some(node) = trail_check.pop() {
        if node.value == 9 {
            ends.push(node);
            continue;
        }

        trail_check.extend(find_follow_up_nodes(node, grid, grid_size).iter());
    }

    ends
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 36);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 81);

        Ok(())
    }
}
