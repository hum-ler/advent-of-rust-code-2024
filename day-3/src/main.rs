use anyhow::Result;
use regex::Regex;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-3.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?
        .captures_iter(input)
        .map(|capture| {
            let (_, [x, y]) = capture.extract();

            Ok(x.parse::<u32>()? * y.parse::<u32>()?)
        })
        .sum()
}

fn part_2(input: &str) -> Result<u32> {
    part_1(&remove_disabled_sections(input)?)
}

/// Strips out sections marked by `don't()` and `do()`.
///
/// Does not handle any dangling `don't()` at the end.
fn remove_disabled_sections(input: &str) -> Result<String> {
    Ok(String::from(
        Regex::new(r"(?s)don't\(\).*?do\(\)")?.replace_all(input, ""),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))")?,
            161
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")?,
            48
        );

        Ok(())
    }
}
