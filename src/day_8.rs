use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;

use crate::{file_to_lines, string_to_lines};

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

const INPUT_FILE: &str = "inputs/day-8.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<usize> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<usize> {
    part_2(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<usize> {
    part_2(&file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: &[String]) -> Result<usize> {
    let (hash_map, row_count, column_count) = parse_lines_into_sets(lines)?;

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

fn part_2(lines: &[String]) -> Result<usize> {
    let (hash_map, row_count, column_count) = parse_lines_into_sets(lines)?;

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

/// Parses input into a hash of a symbol to the set of its antenna, plus the total counts of rows
/// and columns in the grid.
fn parse_lines_into_sets(lines: &[String]) -> Result<(HashMap<u8, HashSet<Coord>>, usize, usize)> {
    let mut hash_map: HashMap<u8, HashSet<Coord>> = HashMap::new();

    lines
        .iter()
        .enumerate()
        .try_for_each::<_, Result<()>>(|(row, s)| {
            s.as_bytes()
                .iter()
                .enumerate()
                .try_for_each::<_, Result<()>>(|(column, c)| {
                    match c {
                        b'.' => (),
                        c => {
                            if !hash_map.contains_key(c) {
                                hash_map.insert(*c, HashSet::new());
                            }

                            let hash_set = hash_map
                                .get_mut(c)
                                .ok_or(anyhow!("Cannot get set: {}", c))?;
                            hash_set.insert((row, column));
                        }
                    }

                    Ok(())
                })?;

            Ok(())
        })?;

    let row_count = lines.len();
    let column_count = lines[0].len();

    Ok((hash_map, row_count, column_count))
}

/// [Improved by Gemini] Parses input into a hash of a symbol to the set of its antenna, plus the
/// total counts of rows and columns in the grid.
fn _gemini_parse_lines_into_sets(
    lines: &[String],
) -> Result<(HashMap<u8, HashSet<Coord>>, usize, usize)> {
    let mut symbol_coords: HashMap<u8, HashSet<Coord>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        for (col, c) in line.bytes().enumerate() {
            if c != b'.' {
                symbol_coords.entry(c).or_default().insert((row, col));
            }
        }
    }

    let row_count = lines.len();
    let col_count = lines.first().map_or(0, String::len); // Handle empty input

    Ok((symbol_coords, row_count, col_count))
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
