use std::{num::ParseIntError, str::FromStr};

use anyhow::Result;

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const INPUT_FILE: &str = "inputs/day-2.txt";

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
    Ok(parse_lines_into_reports(lines)?
        .iter()
        .filter(|r| r.safe)
        .count())
}

fn part_2(lines: &[String]) -> Result<usize> {
    Ok(parse_lines_into_dampened_reports(lines)?
        .iter()
        .filter(|r| r.safe)
        .count())
}

/// Report for part 1.
struct Report {
    _data: Vec<u8>,
    safe: bool,
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .map(|v| v.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;

        let safe = is_safe_data(&data);

        Ok(Self { _data: data, safe })
    }
}

/// Checks for safe data.
fn is_safe_data(data: &[u8]) -> bool {
    assert!(data.len() > 1);

    data.windows(2).all(is_safe_increase) || data.windows(2).all(is_safe_decrease)
}

fn is_safe_increase(pair: &[u8]) -> bool {
    assert_eq!(pair.len(), 2);

    pair[1] > pair[0] && pair[1] - pair[0] <= 3
}

fn is_safe_decrease(pair: &[u8]) -> bool {
    assert_eq!(pair.len(), 2);

    pair[1] < pair[0] && pair[0] - pair[1] <= 3
}

fn parse_lines_into_reports(lines: &[String]) -> Result<Vec<Report>, ParseIntError> {
    lines
        .iter()
        .map(String::as_str)
        .map(Report::from_str)
        .collect()
}

/// Modified report for part 2.
struct DampenedReport {
    _data: Vec<u8>,
    safe: bool,
    _removed: Option<usize>,
}

impl FromStr for DampenedReport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .map(|v| v.parse::<u8>())
            .collect::<Result<Vec<_>, _>>()?;

        let (safe, removed) = is_safe_data_with_dampener(&data);

        Ok(Self {
            _data: data,
            safe,
            _removed: removed,
        })
    }
}

/// Checks for safe data, with allowance to remove one element.
///
/// Returns whether the data is safe, and the index of the element removed (if applicable).
fn is_safe_data_with_dampener(data: &[u8]) -> (bool, Option<usize>) {
    // Check the simple case first.
    if is_safe_data(data) {
        return (true, None);
    }

    // Data looks short enough to just cycle through removals.
    for i in 0..data.len() {
        let mut data = Vec::from(data);
        data.remove(i);

        if is_safe_data(&data) {
            return (true, Some(i));
        }
    }

    (false, None)
}

fn parse_lines_into_dampened_reports(
    lines: &[String],
) -> Result<Vec<DampenedReport>, ParseIntError> {
    lines
        .iter()
        .map(String::as_str)
        .map(DampenedReport::from_str)
        .collect()
}
