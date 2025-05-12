use std::cmp::Ordering;

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-2.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    Ok(parse_input_into_reports(input)?
        .into_iter()
        .filter(|report| is_safe(report))
        .count())
}

fn part_2(input: &str) -> Result<usize> {
    Ok(parse_input_into_reports(input)?
        .into_iter()
        .filter(|report| is_safe_with_tolerance(report))
        .count())
}

fn parse_input_into_reports(input: &str) -> Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            Ok(line
                .split_whitespace()
                .map(|level| level.parse())
                .collect::<Result<Vec<_>, _>>()?)
        })
        .collect()
}

fn is_safe(report: &[u32]) -> bool {
    all_increasing_or_decreasing(report) && all_acceptable_gap(report)
}

fn all_increasing_or_decreasing(report: &[u32]) -> bool {
    report
        .windows(2)
        .map(|window| match window[0].cmp(&window[1]) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        })
        .sum::<i32>()
        .unsigned_abs() as usize
        == report.len() - 1
}

fn all_acceptable_gap(report: &[u32]) -> bool {
    report
        .windows(2)
        .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])))
}

fn is_safe_with_tolerance(report: &[u32]) -> bool {
    if is_safe(report) {
        return true;
    }

    for index in 0..report.len() {
        let mut report = Vec::from(report);
        report.remove(index);

        if is_safe(&report) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 2);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 4);

        Ok(())
    }
}
