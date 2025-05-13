use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-7.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let equations = parse_input_into_equations(input)?;

    Ok(equations
        .into_iter()
        .filter(is_solvable)
        .map(|equation| equation.1)
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let equations = parse_input_into_equations(input)?;

    Ok(equations
        .into_iter()
        .filter(is_solvable_with_concat)
        .map(|equation| equation.1)
        .sum())
}

/// (operands, test_value)
type Equation = (Vec<u64>, u64);

fn parse_input_into_equations(input: &str) -> Result<Vec<Equation>> {
    input
        .lines()
        .map(|line| {
            let Some((test_value, operands)) = line.split_once(": ") else {
                return Err(anyhow!(
                    "Cannot split into test value and operands: {}",
                    line
                ));
            };

            let test_value = test_value.parse()?;
            let operands = operands
                .split_whitespace()
                .map(|token| token.parse())
                .collect::<Result<Vec<_>, _>>()?;

            Ok((operands, test_value))
        })
        .collect()
}

fn is_solvable(equation: &Equation) -> bool {
    check_is_solvable(equation.1, equation.0[0], &equation.0[1..])
}

fn check_is_solvable(target: u64, left_operand: u64, right_operands: &[u64]) -> bool {
    if left_operand > target {
        return false;
    }

    if right_operands.is_empty() {
        return left_operand == target;
    }

    check_is_solvable(
        target,
        left_operand + right_operands[0],
        &right_operands[1..],
    ) || check_is_solvable(
        target,
        left_operand * right_operands[0],
        &right_operands[1..],
    )
}

fn is_solvable_with_concat(equation: &Equation) -> bool {
    check_is_solvable_with_concat(equation.1, equation.0[0], &equation.0[1..])
}

fn check_is_solvable_with_concat(target: u64, left_operand: u64, right_operands: &[u64]) -> bool {
    if left_operand > target {
        return false;
    }

    if right_operands.is_empty() {
        return left_operand == target;
    }

    check_is_solvable_with_concat(
        target,
        left_operand + right_operands[0],
        &right_operands[1..],
    ) || check_is_solvable_with_concat(
        target,
        left_operand * right_operands[0],
        &right_operands[1..],
    ) || check_is_solvable_with_concat(
        target,
        concat(left_operand, right_operands[0]),
        &right_operands[1..],
    )
}

/// Concatenates left_operand and right_operand into one number.
///
/// Both left_operand and right_operand must be > 0.
fn concat(left_operand: u64, right_operand: u64) -> u64 {
    let digits = right_operand.ilog10() + 1;

    left_operand * 10u64.pow(digits) + right_operand
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 3749);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 11387);

        Ok(())
    }
}
