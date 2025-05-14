use std::str::FromStr;

use anyhow::{Result, anyhow};
use nalgebra::{matrix, vector};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-13.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let claw_machines = input
        .split_terminator("\n\n")
        .map(ClawMachine::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(claw_machines
        .iter()
        .filter_map(ClawMachine::solve)
        .map(|solution| solution.0 * 3 + solution.1)
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let claw_machines = input
        .split_terminator("\n\n")
        .map(ClawMachine::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(claw_machines
        .into_iter()
        .map(|claw_machine| ClawMachine {
            prize: (
                10000000000000 + claw_machine.prize.0,
                10000000000000 + claw_machine.prize.1,
            ),
            ..claw_machine
        })
        .filter_map(|claw_machine| claw_machine.solve())
        .map(|solution| solution.0 * 3 + solution.1)
        .sum())
}

/// (x, y)
type Vector2 = (u64, u64);

struct ClawMachine {
    a: Vector2,
    b: Vector2,
    prize: Vector2,
}

impl FromStr for ClawMachine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() != 3 {
            return Err(anyhow!("Invalid input: {}", s));
        }

        let Some((a_x, a_y)) = lines[0]
            .strip_prefix("Button A: X+")
            .and_then(|s| s.split_once(", Y+"))
        else {
            return Err(anyhow!(
                "Cannot split input into button A X and Y: {}",
                lines[0]
            ));
        };
        let a = (a_x.parse()?, a_y.parse()?);

        let Some((b_x, b_y)) = lines[1]
            .strip_prefix("Button B: X+")
            .and_then(|s| s.split_once(", Y+"))
        else {
            return Err(anyhow!(
                "Cannot split input into button B X and Y: {}",
                lines[1]
            ));
        };
        let b = (b_x.parse()?, b_y.parse()?);

        let Some((prize_x, prize_y)) = lines[2]
            .strip_prefix("Prize: X=")
            .and_then(|s| s.split_once(", Y="))
        else {
            return Err(anyhow!(
                "Cannot split input into prize X and Y: {}",
                lines[2]
            ));
        };
        let prize = (prize_x.parse()?, prize_y.parse()?);

        Ok(Self { a, b, prize })
    }
}

impl ClawMachine {
    /// Solves number of A and B presses to reach the prize.
    fn solve(&self) -> Option<Vector2> {
        // Solve:
        // (i)  a.0 * x + b.0 * y = prize.0
        // (ii) a.1 * x + b.1 * y = prize.1

        let coefficients = matrix![
            self.a.0 as f64, self.b.0 as f64;
            self.a.1 as f64, self.b.1 as f64;
        ];
        let constants = vector![self.prize.0 as f64, self.prize.1 as f64];
        let solution = coefficients.lu().solve(&constants)?;

        if solution[0].is_sign_negative() || solution[1].is_sign_negative() {
            return None;
        }
        let x = solution[0].round() as u64;
        let y = solution[1].round() as u64;

        if x * self.a.0 + y * self.b.0 == self.prize.0
            && x * self.a.1 + y * self.b.1 == self.prize.1
        {
            Some((x, y))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
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

        assert_eq!(part_1(trim_newlines(example))?, 480);

        Ok(())
    }
}
