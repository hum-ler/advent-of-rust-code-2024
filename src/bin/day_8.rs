use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-8.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (antenna_map, row_count, col_count) = parse_input(input)?;

    Ok(antenna_map
        .values()
        .flat_map(|set| {
            set.iter()
                .combinations(2)
                .flat_map(|antenna| antinodes(antenna[0], antenna[1], row_count, col_count))
        })
        .unique()
        .count())
}

fn part_2(input: String) -> Result<usize> {
    let (antenna_map, row_count, col_count) = parse_input(input)?;

    Ok(antenna_map
        .values()
        .flat_map(|set| {
            set.iter().combinations(2).flat_map(|antenna| {
                antinodes_with_harmonics(antenna[0], antenna[1], row_count, col_count)
            })
        })
        .unique()
        .count())
}

type Coord = (usize, usize);

/// Parses input into a map of symbols to the set of antenna, plus the number of rows and columns in
/// the grid.
fn parse_input(input: String) -> Result<(HashMap<u8, HashSet<Coord>>, usize, usize)> {
    let mut coords: HashMap<u8, HashSet<Coord>> = HashMap::new();

    let input = input.split_terminator("\n").collect::<Vec<_>>();
    let row_count = input.len();
    let col_count = input.first().map_or(0, |s| s.len());

    for (row, line) in input.iter().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                coords.entry(byte).or_default().insert((row, col));
            }
        }
    }

    Ok((coords, row_count, col_count))
}

/// Calculates the antinodes on either side of first and second.
///
/// Takes into account the boundaries of the grid.
fn antinodes(first: &Coord, second: &Coord, row_count: usize, col_count: usize) -> Vec<Coord> {
    let row_diff = first.0.abs_diff(second.0);
    let col_diff = first.1.abs_diff(second.1);

    let mut antinodes = vec![];

    if first.0 <= second.0 && first.1 <= second.1 {
        // f
        //  s

        if first.0 >= row_diff && first.1 >= col_diff {
            antinodes.push((first.0 - row_diff, first.1 - col_diff));
        }

        if second.0 + row_diff < row_count && second.1 + col_diff < col_count {
            antinodes.push((second.0 + row_diff, second.1 + col_diff));
        }
    } else if first.0 >= second.0 && first.1 >= second.1 {
        // s
        //  f

        if second.0 >= row_diff && second.1 >= col_diff {
            antinodes.push((second.0 - row_diff, second.1 - col_diff));
        }

        if first.0 + row_diff < row_count && first.1 + col_diff < col_count {
            antinodes.push((first.0 + row_diff, first.1 + col_diff));
        }
    } else if first.0 <= second.0 && first.1 >= second.1 {
        //  f
        // s

        if first.0 >= row_diff && first.1 + col_diff < col_count {
            antinodes.push((first.0 - row_diff, first.1 + col_diff));
        }

        if second.0 + row_diff < row_count && second.1 >= col_diff {
            antinodes.push((second.0 + row_diff, second.1 - col_diff));
        }
    } else {
        //  s
        // f

        if second.0 >= row_diff && second.1 + col_diff < col_count {
            antinodes.push((second.0 - row_diff, second.1 + col_diff));
        }

        if first.0 + row_diff < row_count && first.1 >= col_diff {
            antinodes.push((first.0 + row_diff, first.1 - col_diff));
        }
    }

    antinodes
}

/// Calculates the antinodes (including harmonics) on either side of, and including first and
/// second.
///
/// Takes into account the boundaries of the grid.
fn antinodes_with_harmonics(
    first: &Coord,
    second: &Coord,
    row_count: usize,
    col_count: usize,
) -> Vec<Coord> {
    let row_diff = first.0.abs_diff(second.0);
    let col_diff = first.1.abs_diff(second.1);

    let mut antinodes = vec![*first, *second];

    if first.0 <= second.0 && first.1 <= second.1 {
        // f
        //  s

        let mut row = first.0;
        let mut col = first.1;
        while row >= row_diff && col >= col_diff {
            row -= row_diff;
            col -= col_diff;

            antinodes.push((row, col));
        }

        let mut row = second.0;
        let mut col = second.1;
        while row + row_diff < row_count && col + col_diff < col_count {
            row += row_diff;
            col += col_diff;

            antinodes.push((row, col));
        }
    } else if first.0 >= second.0 && first.1 >= second.1 {
        // s
        //  f

        let mut row = second.0;
        let mut col = second.1;
        while row >= row_diff && col >= col_diff {
            row -= row_diff;
            col -= col_diff;

            antinodes.push((row, col));
        }

        let mut row = first.0;
        let mut col = first.1;
        while row + row_diff < row_count && col + col_diff < col_count {
            row += row_diff;
            col += col_diff;

            antinodes.push((row, col));
        }
    } else if first.0 <= second.0 && first.1 >= second.1 {
        //  f
        // s

        let mut row = first.0;
        let mut col = first.1;
        while row >= row_diff && col + col_diff < col_count {
            row -= row_diff;
            col += col_diff;

            antinodes.push((row, col));
        }

        let mut row = second.0;
        let mut col = second.1;
        while row + row_diff < row_count && col >= col_diff {
            row += row_diff;
            col -= col_diff;

            antinodes.push((row, col));
        }
    } else {
        //  s
        // f

        let mut row = second.0;
        let mut col = second.1;
        while row >= row_diff && col + col_diff < col_count {
            row -= row_diff;
            col += col_diff;

            antinodes.push((row, col));
        }

        let mut row = first.0;
        let mut col = first.1;
        while row + row_diff < row_count && col >= col_diff {
            row += row_diff;
            col -= col_diff;

            antinodes.push((row, col));
        }
    }

    antinodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 14);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 34);

        Ok(())
    }
}
