use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};

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

const _INPUT_FILE: &str = "inputs/day-19.txt";

pub fn run_example_1() -> Result<usize> {
    example_1(EXAMPLE_INPUT.trim())
}

pub fn _run_part_1() -> Result<usize> {
    _part_1(read_to_string(_INPUT_FILE)?.trim())
}

pub fn _run_example_2() -> Result<usize> {
    _part_2(EXAMPLE_INPUT.trim())
}

pub fn _run_part_2() -> Result<usize> {
    _part_2(read_to_string(_INPUT_FILE)?.trim())
}

fn example_1(input: &str) -> Result<usize> {
    // Is this a variant of coin change?

    // 1. Take the towels list and reduce it to unique patterns.
    //   "r, wr, b, g, bwu, rb, gb, br"  becomes  "r, wr, b, g, bwu"
    // 2. Since 'u' is only found in "bwu", any design with 'u' in it that cannot use "bwu" are out.
    // 3. Do the same for 'w' with "wr".
    // 4. The rest of the designs are possible since 'r', 'b', 'g' are single-color.

    let (_, designs) = parse_input(input)?;

    let mut possible_designs = 0usize;

    for design in designs {
        let design = design.replace("bwu", "");

        if design.contains("u") {
            continue;
        }

        let design = design.replace("wr", "");

        if design.contains("w") {
            continue;
        }

        possible_designs += 1;
    }

    Ok(possible_designs)
}

fn _part_1(input: &str) -> Result<usize> {
    // Is this a variant of coin change?

    // 1. Take the towels list and reduce it to unique patterns that includes 'u' ('r', 'g', 'w',
    //    'b' single-colors are available).
    // 2. For each design, find elements from 1. to substitute the 'u's (the hard part).
    // 3. The rest of the designs are possible since we have 'r', 'g', 'w', 'b'.

    let (patterns, designs) = parse_input(input)?;

    let mut patterns = _reduce_input_patterns(&patterns);
    patterns.sort_by_key(|p| p.len());

    let mut possible_designs = 0usize;

    let mut retries: Vec<&str> = Vec::default();

    for design in &designs {
        let mut reduction = String::from(*design);
        for pattern in &patterns {
            reduction = reduction.replace(pattern, " ");
        }

        if reduction.contains("u") {
            retries.push(design);
        } else {
            possible_designs += 1;
        }
    }

    // Just snuffle the patterns randomly and retry. With enough rounds, the answer should present
    // itself.
    for _ in 0..100 {
        patterns.shuffle(&mut thread_rng());

        for design in &retries.clone() {
            let mut reduction = String::from(*design);
            for pattern in &patterns {
                reduction = reduction.replace(pattern, " ");
            }

            if !reduction.contains("u") {
                retries.remove(retries.iter().position(|r| r == design).unwrap());
                possible_designs += 1;
            }
        }
    }

    Ok(possible_designs)
}

fn _part_2(_input: &str) -> Result<usize> {
    todo!()
}

fn parse_input(input: &str) -> Result<(Vec<&str>, Vec<&str>)> {
    let input = input.trim().split("\n\n").collect::<Vec<_>>();
    if input.len() != 2 {
        return Err(anyhow!("Cannot parse input"));
    }

    let towels = input[0].split(", ").collect::<Vec<_>>();
    let designs = input[1].split("\n").collect::<Vec<_>>();

    Ok((towels, designs))
}

/// Reduce patterns to the minimal set whose elements cannot be broken down into smaller components.
fn _reduce_input_patterns<'a>(patterns: &'a [&'a str]) -> Vec<&'a str> {
    let mut patterns = patterns
        .iter()
        .filter(|p| p.contains("u"))
        .collect::<Vec<_>>();
    patterns.sort_by_key(|p| p.len());

    // After sorting, the shortest sequences are "ug", "ru", "bu", "wu", "ur", "uw", "ub".
    let mut unique_sequences = vec!["ug", "ru", "bu", "wu", "ur", "uw", "ub"];
    let patterns = &patterns[unique_sequences.len()..patterns.len()];

    // By inspection, the remaining patterns have 1 to 4 'u's in them, so let's separate them first.
    let mut patterns_by_u_count: Vec<Vec<&str>> = vec![Vec::default(); 4];
    patterns.iter().for_each(|p| {
        let index = p.as_bytes().iter().filter(|b| **b == b'u').count() - 1;

        patterns_by_u_count[index].push(*p);
    });

    // Those with only 1 'u' is simple.
    for pattern in patterns_by_u_count[0].clone() {
        let mut reduction = pattern.to_owned();

        for sequence in unique_sequences.clone() {
            reduction = reduction.replace(sequence, "");
        }

        if reduction.contains("u") {
            unique_sequences.push(pattern);
        }
    }

    // For 2 'u's, the number of elements are small enough that we can manually inspect the them.
    unique_sequences.extend_from_slice(&[
        "wuu", "ubu", "ruu", "uug", "uuw", "uru", "uwu", "guu", "ugu", "buu", "uur", "ubgu",
        "uubbr", "uubbww",
    ]);

    // 3 'u's.
    unique_sequences.extend_from_slice(&["uuu", "ubuu"]);

    // 4 'u's.
    unique_sequences.extend_from_slice(&["uuuu"]);

    unique_sequences
}
