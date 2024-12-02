use std::collections::HashMap;

use anyhow::Result;

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

const INPUT_FILE: &str = "inputs/day-1.txt";

pub fn run_example_1() -> Result<u32> {
    part_1(string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<u32> {
    part_1(file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<u32> {
    part_2(string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<u32> {
    part_2(file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: Vec<String>) -> Result<u32> {
    let (mut left, mut right) = parse_lines_into_lists(lines);

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum::<u32>())
}

fn part_2(lines: Vec<String>) -> Result<u32> {
    let (left, right) = parse_lines_into_lists(lines);

    let mut cache: HashMap<u32, u32> = HashMap::new();

    Ok(left
        .iter()
        .map(|v| {
            if !cache.contains_key(v) {
                let count = right.iter().filter(|u| *u == v).count() as u32; // FIXME: may fail to cast

                cache.insert(*v, count);
            }

            v * cache[v]
        })
        .sum::<u32>())
}

/// Converts the input lines into left and right lists of u32s.
fn parse_lines_into_lists(lines: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    lines
        .iter()
        .flat_map(|s| s.split_whitespace())
        .filter_map(|v| v.parse::<u32>().ok()) // FIXME: skips over errors
        .collect::<Vec<u32>>()
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip()
}
