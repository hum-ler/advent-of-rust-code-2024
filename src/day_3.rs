use std::{fs::read_to_string, num::ParseIntError};

use anyhow::{anyhow, Result};
use regex::Regex;

const EXAMPLE_1_INPUT: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const EXAMPLE_2_INPUT: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

const INPUT_FILE: &str = "inputs/day-3.txt";

pub fn run_example_1() -> Result<u32> {
    part_1(EXAMPLE_1_INPUT)
}

pub fn run_part_1() -> Result<u32> {
    part_1(read_to_string(INPUT_FILE)?.as_str())
}

pub fn run_example_2() -> Result<u32> {
    part_2(EXAMPLE_2_INPUT)
}

pub fn run_part_2() -> Result<u32> {
    part_2(read_to_string(INPUT_FILE)?.as_str())
}

fn part_1(input: &str) -> Result<u32> {
    mul(input)
}

fn part_2(input: &str) -> Result<u32> {
    // _mul_with_do_by_stripping(input)

    mul_with_do_by_scanning(input)
}

fn mul(input: &str) -> Result<u32> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?
        .captures_iter(input)
        .map(|c| {
            let [left, right] = c.extract().1;

            Ok(left.parse::<u32>()? * right.parse::<u32>()?)
        })
        .sum()
}

fn mul_with_do_by_scanning(input: &str) -> Result<u32> {
    let acc: Result<(u32, bool), ParseIntError> =
        Regex::new(r"(?<inst>mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)|don't\(\)|do\(\))")?
            .captures_iter(input)
            .try_fold((0u32, true), |acc, c| match &c["inst"] {
                "don't()" => Ok((acc.0, false)),
                "do()" => Ok((acc.0, true)),
                _ => {
                    if acc.1 {
                        let left = c["left"].parse::<u32>()?;
                        let right = c["right"].parse::<u32>()?;

                        Ok((acc.0 + left * right, true))
                    } else {
                        Ok(acc)
                    }
                }
            });

    // Rewrap the result because T is different.
    if let Ok(acc) = acc {
        Ok(acc.0)
    } else {
        Err(anyhow!("Cannot parse input: {}", acc.unwrap_err()))
    }
}

fn _mul_with_do_by_stripping(input: &str) -> Result<u32> {
    mul(_strip_donts(input).as_str())
}

/// Removes all `don't()`s until we hit a `do()` or the end of line.
fn _strip_donts(input: &str) -> String {
    let mut input = String::from(input);

    while let Some(start_index) = input.find("don't()") {
        let (header, search) = input.split_at(start_index);

        if let Some(end_index) = search.find("do()") {
            input = [header, search.split_at(end_index + 4).1].join("");
        } else {
            input = header.to_string();
        }
    }

    input
}
