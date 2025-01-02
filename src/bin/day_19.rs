use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-19.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (patterns, designs) = parse_input(&input)?;

    Ok(designs
        .iter()
        .filter(|design| is_possible_design(design, &patterns))
        .count())
}

fn part_2(input: String) -> Result<usize> {
    let (patterns, designs) = parse_input(&input)?;

    let mut combinations_cache: HashMap<String, usize> = HashMap::new();

    Ok(designs
        .iter()
        .map(|design| {
            count_possible_combinations(design.to_string(), &patterns, &mut combinations_cache)
        })
        .sum())
}

fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let input = input.trim().split_terminator("\n\n").collect::<Vec<_>>();
    if input.len() != 2 {
        return Err(anyhow!("Cannot parse input"));
    }

    let towels = input[0].split_terminator(", ").collect::<Vec<_>>();
    let designs = input[1].split_terminator("\n").collect::<Vec<_>>();

    Ok((towels, designs))
}

fn is_possible_design(design: &str, patterns: &[&str]) -> bool {
    let mut branches: Vec<String> = Vec::default();

    for pattern in patterns {
        let pattern = *pattern;

        if design == pattern {
            return true;
        }

        if design.starts_with(pattern) {
            branches.push(design.replacen(pattern, "", 1));
        }
    }

    branches
        .into_iter()
        .any(|shorter_design| is_possible_design(&shorter_design, patterns))
}

/// Counts all possible combinations that can be used to create a design.
///
/// Using owned types as it is easier to manage the lifetime of cached objects.
fn count_possible_combinations(
    design: String,
    patterns: &[&str],
    combinations_cache: &mut HashMap<String, usize>,
) -> usize {
    if combinations_cache.contains_key(&design) {
        return combinations_cache[&design];
    }

    let mut branches: Vec<String> = Vec::default();

    // Even if we find an exact match, there can still be other combinations using shorter patterns.
    let mut exact_pattern_found = false;

    for pattern in patterns {
        let pattern = *pattern;

        if design == pattern {
            exact_pattern_found = true;
        }

        if design.starts_with(pattern) {
            branches.push(design.replacen(pattern, "", 1));
        }
    }

    let total_count = exact_pattern_found as usize
        + branches
            .into_iter()
            .map(|shorter_design| {
                count_possible_combinations(shorter_design, patterns, combinations_cache)
            })
            .sum::<usize>();

    *combinations_cache.entry(design).or_insert(total_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 6);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 16);

        Ok(())
    }
}
