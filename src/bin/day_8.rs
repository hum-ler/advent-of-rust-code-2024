use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

const INPUT_FILE: &str = "inputs/day-8.txt";

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (hash_map, row_count, column_count) = parse_input_into_sets(&input)?;

    Ok(hash_map
        .values()
        .flat_map(|s| {
            s.iter()
                .combinations(2)
                .flat_map(|c| antinodes(c[0], c[1], row_count, column_count))
        })
        .unique()
        .count())
}

fn part_2(input: String) -> Result<usize> {
    let (hash_map, row_count, column_count) = parse_input_into_sets(&input)?;

    Ok(hash_map
        .values()
        .flat_map(|s| {
            s.iter()
                .combinations(2)
                .flat_map(|c| antinodes_with_harmonics(c[0], c[1], row_count, column_count))
        })
        .unique()
        .count())
}

type Coord = (usize, usize);

/// Parses input into a map of symbols to the set of antenna, plus the number of rows and columns in
/// the grid.
fn parse_input_into_sets(input: &str) -> Result<(HashMap<u8, HashSet<Coord>>, usize, usize)> {
    let mut coords: HashMap<u8, HashSet<Coord>> = HashMap::new();

    let input = input.split_terminator("\n").collect::<Vec<_>>();
    let row_count = input.len();
    let column_count = input.first().map_or(0, |s| s.len());

    for (row, line) in input.iter().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte != b'.' {
                coords.entry(byte).or_default().insert((row, col));
            }
        }
    }

    Ok((coords, row_count, column_count))
}

/// Calculates the antinodes on either side of [first] and [second].
///
/// Takes into account the boundaries of the grid.
fn antinodes(first: &Coord, second: &Coord, row_count: usize, column_count: usize) -> Vec<Coord> {
    let row_diff = first.0.abs_diff(second.0);
    let column_diff = first.1.abs_diff(second.1);

    let mut antinodes = vec![];

    if first.0 <= second.0 && first.1 <= second.1 {
        // f
        //  s

        if first.0 >= row_diff && first.1 >= column_diff {
            antinodes.push((first.0 - row_diff, first.1 - column_diff));
        }

        if second.0 + row_diff < row_count && second.1 + column_diff < column_count {
            antinodes.push((second.0 + row_diff, second.1 + column_diff));
        }
    } else if first.0 >= second.0 && first.1 >= second.1 {
        // s
        //  f

        if second.0 >= row_diff && second.1 >= column_diff {
            antinodes.push((second.0 - row_diff, second.1 - column_diff));
        }

        if first.0 + row_diff < row_count && first.1 + column_diff < column_count {
            antinodes.push((first.0 + row_diff, first.1 + column_diff));
        }
    } else if first.0 <= second.0 && first.1 >= second.1 {
        //  f
        // s

        if first.0 >= row_diff && first.1 + column_diff < column_count {
            antinodes.push((first.0 - row_diff, first.1 + column_diff));
        }

        if second.0 + row_diff < row_count && second.1 >= column_diff {
            antinodes.push((second.0 + row_diff, second.1 - column_diff));
        }
    } else {
        //  s
        // f

        if second.0 >= row_diff && second.1 + column_diff < column_count {
            antinodes.push((second.0 - row_diff, second.1 + column_diff));
        }

        if first.0 + row_diff < row_count && first.1 >= column_diff {
            antinodes.push((first.0 + row_diff, first.1 - column_diff));
        }
    }

    antinodes
}

/// Calculates the antinodes (including harmonics) on either side of, and including [first] and
/// [second].
///
/// Takes into account the boundaries of the grid.
fn antinodes_with_harmonics(
    first: &Coord,
    second: &Coord,
    row_count: usize,
    column_count: usize,
) -> Vec<Coord> {
    let row_diff = first.0.abs_diff(second.0);
    let column_diff = first.1.abs_diff(second.1);

    let mut antinodes = vec![*first, *second];

    if first.0 <= second.0 && first.1 <= second.1 {
        // f
        //  s

        let mut row = first.0;
        let mut column = first.1;
        while row >= row_diff && column >= column_diff {
            row -= row_diff;
            column -= column_diff;

            antinodes.push((row, column));
        }

        let mut row = second.0;
        let mut column = second.1;
        while row + row_diff < row_count && column + column_diff < column_count {
            row += row_diff;
            column += column_diff;

            antinodes.push((row, column));
        }
    } else if first.0 >= second.0 && first.1 >= second.1 {
        // s
        //  f

        let mut row = second.0;
        let mut column = second.1;
        while row >= row_diff && column >= column_diff {
            row -= row_diff;
            column -= column_diff;

            antinodes.push((row, column));
        }

        let mut row = first.0;
        let mut column = first.1;
        while row + row_diff < row_count && column + column_diff < column_count {
            row += row_diff;
            column += column_diff;

            antinodes.push((row, column));
        }
    } else if first.0 <= second.0 && first.1 >= second.1 {
        //  f
        // s

        let mut row = first.0;
        let mut column = first.1;
        while row >= row_diff && column + column_diff < column_count {
            row -= row_diff;
            column += column_diff;

            antinodes.push((row, column));
        }

        let mut row = second.0;
        let mut column = second.1;
        while row + row_diff < row_count && column >= column_diff {
            row += row_diff;
            column -= column_diff;

            antinodes.push((row, column));
        }
    } else {
        //  s
        // f

        let mut row = second.0;
        let mut column = second.1;
        while row >= row_diff && column + column_diff < column_count {
            row -= row_diff;
            column += column_diff;

            antinodes.push((row, column));
        }

        let mut row = first.0;
        let mut column = first.1;
        while row + row_diff < row_count && column >= column_diff {
            row += row_diff;
            column -= column_diff;

            antinodes.push((row, column));
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
