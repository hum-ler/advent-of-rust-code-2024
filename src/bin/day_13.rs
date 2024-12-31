use std::str::FromStr;

use anyhow::{anyhow, Result};
use regex::Regex;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-13.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u64> {
    let machines = parse_input_into_machines(input)?;

    Ok(machines.iter().filter_map(ClawMachine::cost).sum())
}

fn part_2(input: String) -> Result<u64> {
    let machines = parse_input_into_machines(input)?;

    // Add 10000000000000 to prize values.
    let machines = machines
        .into_iter()
        .map(ClawMachine::extend_prize)
        .collect::<Result<Vec<_>>>()?;

    Ok(machines.iter().filter_map(ClawMachine::cost).sum())
}

struct Vector2 {
    x: u64,
    y: u64,
}

struct ClawMachine {
    button_a: Vector2,
    button_b: Vector2,
    prize: Vector2,
    prize_is_extended: bool,
}

impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(captures) = Regex::new(r"(?s)Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+).+Button B: X\+(?<bx>\d+), Y\+(?<by>\d+).+Prize: X=(?<px>\d+), Y=(?<py>\d+)")?.captures(s) else {
            return Err(anyhow!("Cannot parse regex: {}", s));
        };

        let ax = captures["ax"].parse()?;
        let ay = captures["ay"].parse()?;
        let bx = captures["bx"].parse()?;
        let by = captures["by"].parse()?;
        let px = captures["px"].parse()?;
        let py = captures["py"].parse()?;

        Ok(Self {
            button_a: Vector2 { x: ax, y: ay },
            button_b: Vector2 { x: bx, y: by },
            prize: Vector2 { x: px, y: py },
            prize_is_extended: false,
        })
    }
}

impl ClawMachine {
    /// Adds 10000000000000 to prize x- and y-values.
    ///
    /// Consumes the original [ClawMachine].
    pub fn extend_prize(mut self) -> Result<Self> {
        if self.prize_is_extended {
            return Err(anyhow!("Prize value is already extended"));
        }

        self.prize.x += 10000000000000;
        self.prize.y += 10000000000000;
        self.prize_is_extended = true;

        Ok(self)
    }

    pub fn solve(&self) -> Option<(u64, u64)> {
        // Use f64 for arithmetic.
        let ax = self.button_a.x as f64;
        let ay = self.button_a.y as f64;
        let bx = self.button_b.x as f64;
        let by = self.button_b.y as f64;
        let px = self.prize.x as f64;
        let py = self.prize.y as f64;

        // Based on:
        //   ax * a_presses + bx * b_presses = px
        //   ay * a_presses + by * b_presses = py
        // We derive the formula below:
        let a_presses = ((px - (bx * py) / by) / (ax - (bx * ay) / by)).round();
        let b_presses = ((px - ax * a_presses) / bx).round();
        if a_presses.is_sign_negative() || b_presses.is_sign_negative() {
            return None;
        }

        // Cast back to unsigned and check solution.
        let a_presses = a_presses as u64;
        let b_presses = b_presses as u64;

        if self.button_a.x * a_presses + self.button_b.x * b_presses == self.prize.x
            && self.button_a.y * a_presses + self.button_b.y * b_presses == self.prize.y
        {
            Some((a_presses, b_presses))
        } else {
            None
        }
    }

    pub fn cost(&self) -> Option<u64> {
        self.solve().map(|(a, b)| a * 3 + b)
    }
}

fn parse_input_into_machines(input: String) -> Result<Vec<ClawMachine>> {
    input.split_terminator("\n\n").map(ClawMachine::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 480);

        Ok(())
    }
}
