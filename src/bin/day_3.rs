use anyhow::Result;
use regex::Regex;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-3.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    mul(&input)
}

fn part_2(input: String) -> Result<u32> {
    mul_with_do_and_dont(&input)
}

/// Sums up all the `mul()`s.
fn mul(input: &str) -> Result<u32> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?
        .captures_iter(input)
        .map(|capture| {
            let [left, right] = capture.extract().1;

            Ok(left.parse::<u32>()? * right.parse::<u32>()?)
        })
        .sum()
}

/// Sums up all the `mul()`s, taking `do()`s and `don't()`s into account.
///
/// All the `don't()`s are stripped out from `input` before `mul()`s are calculated.
fn mul_with_do_and_dont(input: &str) -> Result<u32> {
    mul(&strip_donts(input)?)
}

/// Removes all `don't()`s until we hit a `do()` or the end of line.
fn strip_donts(input: &str) -> Result<String> {
    // The `s` flag allows `.` to match `\n`.
    let input = Regex::new(r"(?s)don't\(\).*?do\(\)")?.replace_all(input, "");
    let input = Regex::new(r"(?s)don't\(\).*")?.replace(input.as_ref(), "");

    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

        assert_eq!(part_1(input)?, 161);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

        assert_eq!(part_2(input)?, 48);

        Ok(())
    }
}
