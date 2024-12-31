use std::str::FromStr;

use anyhow::Result;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-2.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    Ok(parse_input_into_reports(input)?
        .into_iter()
        .filter(Report::is_safe)
        .count())
}

fn part_2(input: String) -> Result<usize> {
    Ok(parse_input_into_reports(input)?
        .into_iter()
        .filter(Report::is_safe_with_dampener)
        .count())
}

#[derive(Clone)]
struct Report(Vec<u8>);

impl FromStr for Report {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(data))
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        assert!(self.0.len() > 1);

        self.0.windows(2).all(Report::is_safe_increase)
            || self.0.windows(2).all(Report::is_safe_decrease)
    }

    fn is_safe_with_dampener(&self) -> bool {
        // Check the simple case first.
        if self.is_safe() {
            return true;
        }

        // Data looks short enough to just cycle through removals.
        for i in 0..self.0.len() {
            let mut report = self.clone();
            report.0.remove(i);

            if report.is_safe() {
                return true;
            }
        }

        false
    }

    fn is_safe_increase(pair: &[u8]) -> bool {
        assert_eq!(pair.len(), 2);

        pair[1] > pair[0] && pair[1] - pair[0] <= 3
    }

    fn is_safe_decrease(pair: &[u8]) -> bool {
        assert_eq!(pair.len(), 2);

        pair[1] < pair[0] && pair[0] - pair[1] <= 3
    }
}

fn parse_input_into_reports(lines: String) -> Result<Vec<Report>> {
    lines.split_terminator("\n").map(Report::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 2);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 4);

        Ok(())
    }
}
