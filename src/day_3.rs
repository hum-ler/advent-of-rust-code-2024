use std::fs::read_to_string;

use anyhow::Result;
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
    Ok(mul(input))
}

fn part_2(input: &str) -> Result<u32> {
    // Ok(_mul_with_do_by_stripping(input))

    Ok(mul_with_do_by_scanning(input))
}

fn mul(input: &str) -> u32 {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .expect("Cannot create regex")
        .captures_iter(input)
        .map(|c| {
            let [left, right] = c.extract().1;

            left.parse::<u32>().expect("Cannot parse captured left")
                * right.parse::<u32>().expect("Cannot parse captured right")
        })
        .sum()
}

fn mul_with_do_by_scanning(input: &str) -> u32 {
    Regex::new(r"(?<inst>mul\((?<left>\d{1,3}),(?<right>\d{1,3})\)|don't\(\)|do\(\))")
        .expect("Cannot create regex")
        .captures_iter(input)
        .fold((0u32, true), |acc, c| match &c["inst"] {
            "don't()" => (acc.0, false),
            "do()" => (acc.0, true),
            _ => {
                if acc.1 {
                    (
                        acc.0
                            + c["left"]
                                .parse::<u32>()
                                .expect("Cannot parse captured left")
                                * c["right"]
                                    .parse::<u32>()
                                    .expect("Cannot parse captured right"),
                        true,
                    )
                } else {
                    acc
                }
            }
        })
        .0
}

fn _mul_with_do_by_stripping(input: &str) -> u32 {
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
