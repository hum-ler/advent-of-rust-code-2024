use std::{num::ParseIntError, str::FromStr};

use anyhow::{anyhow, Result};

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

const INPUT_FILE: &str = "inputs/day-7.txt";

pub fn run_example_1() -> Result<u64> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<u64> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<u64> {
    part_2(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<u64> {
    part_2(&file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: &[String]) -> Result<u64> {
    parse_lines_to_equations(lines)?
        .iter()
        .try_fold(0, |acc, e| {
            if e.operators(false)?.is_some() {
                Ok(acc + e.test_value)
            } else {
                Ok(acc)
            }
        })
}

fn part_2(lines: &[String]) -> Result<u64> {
    parse_lines_to_equations(lines)?
        .iter()
        .try_fold(0, |acc, e| {
            if e.operators(true)?.is_some() {
                Ok(acc + e.test_value)
            } else {
                Ok(acc)
            }
        })
}

#[derive(Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

struct Equation {
    test_value: u64,
    operands: Vec<u64>,
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let input = s.split(": ").collect::<Vec<&str>>();
        if input.len() != 2 {
            return Err(anyhow!("Cannot split input : {}", s));
        }

        let test_value = input[0].parse::<u64>()?;
        let operands = input[1]
            .split(" ")
            .map(|o| o.parse::<u64>())
            .collect::<Result<Vec<u64>, _>>()?;

        Ok(Self {
            test_value,
            operands,
        })
    }
}

impl Equation {
    /// Searches for the first sequence of [Operator]s that returns the test value.
    ///
    /// Returns `Ok(None)` if no such sequence is found.
    ///
    /// Set [allow_concatenate] to true to allow [Operator::Concatenate] to appear in the sequence
    /// (as in part 2). Otherwise, only [Operator::Add] and [Operator::Multiply] will be used.
    pub fn operators(&self, allow_concatenate: bool) -> Result<Option<Vec<Operator>>> {
        assert!(self.operands.len() >= 2);

        // Initialize the search queue.
        let mut check_queue: Vec<(u64, Vec<Operator>)> = vec![
            (self.operands[0], vec![Operator::Add]),
            (self.operands[0], vec![Operator::Multiply]),
        ];
        if allow_concatenate {
            check_queue.push((self.operands[0], vec![Operator::Concatenate]));
        }

        while !check_queue.is_empty() {
            let mut check = check_queue
                .pop()
                .ok_or(anyhow!("Cannot pop but queue is not empty"))?;

            let intermediate_value = match check.1[check.1.len() - 1] {
                Operator::Add => check.0 + self.operands[check.1.len()],
                Operator::Multiply => check.0 * self.operands[check.1.len()],
                Operator::Concatenate => concatenate(check.0, self.operands[check.1.len()])?,
            };

            // Failure -- intermediate_value can only get bigger.
            if intermediate_value > self.test_value {
                continue;
            }

            // Termination -- there are only n - 1 slots between n operands.
            if check.1.len() == self.operands.len() - 1 {
                // Success.
                if intermediate_value == self.test_value && check.1.len() == self.operands.len() - 1
                {
                    return Ok(Some(check.1));
                }

                continue;
            }

            // Otherwise, push more operators and continue search.

            let mut new_add = check.1.clone();
            new_add.push(Operator::Add);
            check_queue.push((intermediate_value, new_add));

            let mut new_multiply = check.1.clone();
            new_multiply.push(Operator::Multiply);
            check_queue.push((intermediate_value, new_multiply));

            if allow_concatenate {
                // Reuse check.
                check.1.push(Operator::Concatenate);
                check_queue.push((intermediate_value, check.1));
            }
        }

        Ok(None)
    }
}

fn parse_lines_to_equations(lines: &[String]) -> Result<Vec<Equation>> {
    lines
        .iter()
        .map(String::as_str)
        .map(Equation::from_str)
        .collect()
}

fn concatenate(first: u64, second: u64) -> Result<u64, ParseIntError> {
    (first.to_string() + &second.to_string()).parse()
}
