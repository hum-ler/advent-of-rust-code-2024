use std::{fs::read_to_string, path::Path};

use anyhow::{anyhow, Result};
use clap::{Parser, ValueEnum};

mod day_1;
mod day_2;
mod day_3;

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
        _ => return Err(anyhow!("Invalid DAY_NUMBER or RUN_CODE")),
    }

    Ok(())
}

/// Splits a literal into separate lines of owned Strings.
/// Throws away lines that are totally empty.
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
