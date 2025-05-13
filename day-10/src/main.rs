use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-10.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = parse_input_into_grid(input)?;
    let trailheads = trailheads(&grid);

    let mut cache = HashMap::new();
    Ok(trailheads
        .into_iter()
        .map(|trailhead| connected_peaks(trailhead, &grid, &mut cache).len())
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let grid = parse_input_into_grid(input)?;
    let trailheads = trailheads(&grid);

    let mut cache = HashMap::new();
    Ok(trailheads
        .into_iter()
        .map(|trailhead| count_paths_to_peaks(trailhead, &grid, &mut cache))
        .sum())
}

struct Grid {
    heights: Vec<Vec<u32>>,
    size: usize,
}

fn parse_input_into_grid(input: &str) -> Result<Grid> {
    let lines = input.lines().collect::<Vec<_>>();

    let size = lines.len();
    let heights = lines
        .into_iter()
        .map(|line| {
            line.bytes()
                .map(|byte| {
                    if byte.is_ascii_digit() {
                        Ok((byte - b'0') as u32)
                    } else {
                        Err(anyhow!("Invalid byte: {}", byte))
                    }
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(Grid { heights, size })
}

/// (row, col)
type Coord = (usize, usize);

fn trailheads(grid: &Grid) -> Vec<Coord> {
    grid.heights
        .iter()
        .enumerate()
        .flat_map(|(row, values)| {
            values.iter().enumerate().filter_map(
                move |(col, value)| {
                    if *value == 0 { Some((row, col)) } else { None }
                },
            )
        })
        .collect()
}

fn connected_peaks(
    coord: Coord,
    grid: &Grid,
    cache: &mut HashMap<Coord, HashSet<Coord>>,
) -> HashSet<Coord> {
    if cache.contains_key(&coord) {
        return cache[&coord].clone();
    }

    let (row, col) = coord;
    let height = grid.heights[row][col];

    if height == 9 {
        cache.entry(coord).or_default().insert((row, col));
        return cache[&coord].clone();
    }

    let mut successors = Vec::new();
    if row > 0 && grid.heights[row - 1][col] == height + 1 {
        successors.push((row - 1, col));
    }
    if col < grid.size - 1 && grid.heights[row][col + 1] == height + 1 {
        successors.push((row, col + 1));
    }
    if row < grid.size - 1 && grid.heights[row + 1][col] == height + 1 {
        successors.push((row + 1, col));
    }
    if col > 0 && grid.heights[row][col - 1] == height + 1 {
        successors.push((row, col - 1));
    }

    let peaks = successors
        .into_iter()
        .flat_map(|successor| connected_peaks(successor, grid, cache))
        .collect::<HashSet<_>>();
    cache.entry(coord).or_insert(peaks);
    cache[&coord].clone()
}

fn count_paths_to_peaks(coord: Coord, grid: &Grid, cache: &mut HashMap<Coord, u32>) -> u32 {
    if cache.contains_key(&coord) {
        return cache[&coord];
    }

    let (row, col) = coord;
    let height = grid.heights[row][col];

    if height == 9 {
        return *cache.entry(coord).or_insert(1);
    }

    let mut successors = Vec::new();
    if row > 0 && grid.heights[row - 1][col] == height + 1 {
        successors.push((row - 1, col));
    }
    if col < grid.size - 1 && grid.heights[row][col + 1] == height + 1 {
        successors.push((row, col + 1));
    }
    if row < grid.size - 1 && grid.heights[row + 1][col] == height + 1 {
        successors.push((row + 1, col));
    }
    if col > 0 && grid.heights[row][col - 1] == height + 1 {
        successors.push((row, col - 1));
    }

    let paths = successors
        .into_iter()
        .map(|successor| count_paths_to_peaks(successor, grid, cache))
        .sum();
    *cache.entry(coord).or_insert(paths)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 36);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 81);

        Ok(())
    }
}
