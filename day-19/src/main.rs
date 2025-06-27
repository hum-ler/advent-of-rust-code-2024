use std::collections::HashMap;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-19.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (patterns, designs) = parse_input_into_patterns_and_designs(input)?;

    let mut cache = HashMap::new();
    Ok(designs
        .into_iter()
        .filter(|design| is_possible_design(design, &patterns, &mut cache))
        .count())
}

fn part_2(input: &str) -> Result<u64> {
    let (patterns, designs) = parse_input_into_patterns_and_designs(input)?;

    let mut cache = HashMap::new();
    Ok(designs
        .into_iter()
        .map(|design| pattern_combinations(design, &patterns, &mut cache))
        .sum())
}

fn parse_input_into_patterns_and_designs(input: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let Some((patterns, designs)) = input.split_once("\n\n") else {
        return Err(anyhow!("Cannot split input into patterns and designs"));
    };

    let patterns = patterns.split_terminator(", ").collect();
    let designs = designs.lines().collect();

    Ok((patterns, designs))
}

fn is_possible_design<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, bool>,
) -> bool {
    if cache.contains_key(design) {
        return cache[design];
    }

    if design.is_empty() {
        return *cache.entry(design).or_insert(true);
    }

    let possible_sub_designs = patterns.iter().any(|pattern| {
        design
            .strip_prefix(pattern)
            .is_some_and(|design| is_possible_design(design, patterns, cache))
    });
    *cache.entry(design).or_insert(possible_sub_designs)
}

fn pattern_combinations<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if cache.contains_key(design) {
        return cache[design];
    }

    if design.is_empty() {
        return *cache.entry(design).or_insert(1);
    }

    let sub_design_combinations = patterns
        .iter()
        .map(|pattern| {
            design
                .strip_prefix(pattern)
                .map_or(0, |design| pattern_combinations(design, patterns, cache))
        })
        .sum();
    *cache.entry(design).or_insert(sub_design_combinations)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 6);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 16);

        Ok(())
    }
}
