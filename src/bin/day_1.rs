use std::collections::HashMap;

use anyhow::Result;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-1.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    let (mut left, mut right) = parse_input_into_lists(input)?;

    left.sort();
    right.sort();

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum())
}

fn part_2(input: String) -> Result<u32> {
    let (left, right) = parse_input_into_lists(input)?;

    let mut cache: HashMap<u32, u32> = HashMap::new();

    left.iter()
        .map(|n| {
            if !cache.contains_key(n) {
                let count = right.iter().filter(|m| *m == n).count() as u32;

                cache.insert(*n, n * count);
            }

            Ok(cache[n])
        })
        .sum()
}

/// Separates the input into left and right lists.
fn parse_input_into_lists(input: String) -> Result<(Vec<u32>, Vec<u32>)> {
    Ok(input
        .split_terminator("\n")
        .flat_map(|line| line.split_whitespace())
        .map(|n| n.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?
        .chunks(2)
        .map(|pair| (pair[0], pair[1]))
        .unzip())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 11);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 31);

        Ok(())
    }
}
