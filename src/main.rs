use std::{fs::read_to_string, path::Path};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};

mod day_1;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

#[derive(Parser)]
struct Args {
    day_number: u8,

    #[arg(value_enum)]
    run_code: RunCode,
}

#[derive(Clone, ValueEnum)]
enum RunCode {
    E1,
    E2,
    P1,
    P2,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match (args.day_number, args.run_code) {
        (1, RunCode::E1) => println!("{:?}", day_1::run_example_1()),
        (1, RunCode::E2) => println!("{:?}", day_1::run_example_2()),
        (1, RunCode::P1) => println!("{:?}", day_1::run_part_1()),
        (1, RunCode::P2) => println!("{:?}", day_1::run_part_2()),
        (2, RunCode::E1) => println!("{:?}", day_2::run_example_1()),
        (2, RunCode::E2) => println!("{:?}", day_2::run_example_2()),
        (2, RunCode::P1) => println!("{:?}", day_2::run_part_1()),
        (2, RunCode::P2) => println!("{:?}", day_2::run_part_2()),
        (3, RunCode::E1) => println!("{:?}", day_3::run_example_1()),
        (3, RunCode::E2) => println!("{:?}", day_3::run_example_2()),
        (3, RunCode::P1) => println!("{:?}", day_3::run_part_1()),
        (3, RunCode::P2) => println!("{:?}", day_3::run_part_2()),
        (4, RunCode::E1) => println!("{:?}", day_4::run_example_1()),
        (4, RunCode::E2) => println!("{:?}", day_4::run_example_2()),
        (4, RunCode::P1) => println!("{:?}", day_4::run_part_1()),
        (4, RunCode::P2) => println!("{:?}", day_4::run_part_2()),
        (5, RunCode::E1) => println!("{:?}", day_5::run_example_1()),
        (5, RunCode::E2) => println!("{:?}", day_5::run_example_2()),
        (5, RunCode::P1) => println!("{:?}", day_5::run_part_1()),
        (5, RunCode::P2) => println!("{:?}", day_5::run_part_2()),
        (6, RunCode::E1) => println!("{:?}", day_6::run_example_1()),
        (6, RunCode::E2) => println!("{:?}", day_6::run_example_2()),
        (6, RunCode::P1) => println!("{:?}", day_6::run_part_1()),
        (6, RunCode::P2) => println!("{:?}", day_6::run_part_2()),
        (7, RunCode::E1) => println!("{:?}", day_7::run_example_1()),
        (7, RunCode::E2) => println!("{:?}", day_7::run_example_2()),
        (7, RunCode::P1) => println!("{:?}", day_7::run_part_1()),
        (7, RunCode::P2) => println!("{:?}", day_7::run_part_2()),
        (8, RunCode::E1) => println!("{:?}", day_8::run_example_1()),
        (8, RunCode::E2) => println!("{:?}", day_8::run_example_2()),
        (8, RunCode::P1) => println!("{:?}", day_8::run_part_1()),
        (8, RunCode::P2) => println!("{:?}", day_8::run_part_2()),
        (9, RunCode::E1) => println!("{:?}", day_9::run_example_1()),
        (9, RunCode::E2) => println!("{:?}", day_9::run_example_2()),
        (9, RunCode::P1) => println!("{:?}", day_9::run_part_1()),
        (9, RunCode::P2) => println!("{:?}", day_9::run_part_2()),
        (10, RunCode::E1) => println!("{:?}", day_10::run_example_1()),
        (10, RunCode::E2) => println!("{:?}", day_10::run_example_2()),
        (10, RunCode::P1) => println!("{:?}", day_10::run_part_1()),
        (10, RunCode::P2) => println!("{:?}", day_10::run_part_2()),
        (11, RunCode::E1) => println!("{:?}", day_11::run_example_1()),
        (11, RunCode::E2) => println!("No example 2"),
        (11, RunCode::P1) => println!("{:?}", day_11::run_part_1()),
        (11, RunCode::P2) => println!("{:?}", day_11::run_part_2()),
        (12, RunCode::E1) => println!("{:?}", day_12::run_example_1()),
        (12, RunCode::E2) => println!("{:?}", day_12::run_example_2()),
        (12, RunCode::P1) => println!("{:?}", day_12::run_part_1()),
        (12, RunCode::P2) => println!("{:?}", day_12::run_part_2()),
        (13, RunCode::E1) => println!("{:?}", day_13::run_example_1()),
        (13, RunCode::E2) => println!("No example 2"),
        (13, RunCode::P1) => println!("{:?}", day_13::run_part_1()),
        (13, RunCode::P2) => println!("{:?}", day_13::run_part_2()),
        (14, RunCode::E1) => println!("{:?}", day_14::run_example_1()),
        (14, RunCode::E2) => println!("No example 2"),
        (14, RunCode::P1) => println!("{:?}", day_14::run_part_1()),
        (14, RunCode::P2) => println!("{:?}", day_14::run_part_2()),
        (15, RunCode::E1) => println!("{:?}", day_15::run_example_1()),
        (15, RunCode::E2) => println!("{:?}", day_15::run_example_2()),
        (15, RunCode::P1) => println!("{:?}", day_15::run_part_1()),
        (15, RunCode::P2) => println!("{:?}", day_15::run_part_2()),
        (16, RunCode::E1) => println!("{:?}", day_16::run_example_1()),
        (16, RunCode::E2) => println!("{:?}", day_16::run_example_2()),
        (16, RunCode::P1) => println!("{:?}", day_16::run_part_1()),
        (16, RunCode::P2) => println!("{:?}", day_16::run_part_2()),
        (17, RunCode::E1) => println!("{:?}", day_17::run_example_1()),
        (17, RunCode::E2) => println!("{:?}", day_17::run_example_2()),
        (17, RunCode::P1) => println!("{:?}", day_17::run_part_1()),
        (17, RunCode::P2) => println!("No solution"),
        _ => return Err(anyhow!("Invalid DAY_NUMBER or RUN_CODE")),
    }

    Ok(())
}

/// Trims and splits a literal into separate lines of owned Strings.
pub fn string_to_lines(input: &str) -> Vec<String> {
    input
        .trim()
        .split_terminator('\n')
        .map(String::from)
        .collect()
}

/// Wraps [read_to_string] and [string_to_lines] to read a file into lines.
pub fn file_to_lines<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    Ok(string_to_lines(read_to_string(path)?.as_str()))
}
