use std::{collections::HashMap, fs::read_to_string};

use anyhow::Result;

const EXAMPLE_INPUT: &str = "125 17";

const INPUT_FILE: &str = "inputs/day-11.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(EXAMPLE_INPUT)
}

pub fn run_part_1() -> Result<usize> {
    part_1(read_to_string(INPUT_FILE)?.trim())
}

pub fn run_part_2() -> Result<usize> {
    part_2(read_to_string(INPUT_FILE)?.trim())
}

fn part_1(input: &str) -> Result<usize> {
    let blink_count = 25;

    let mut stones = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    for _ in 0..blink_count {
        stones = stones.iter().flat_map(transform).collect();
    }

    Ok(stones.len())
}

fn part_2(input: &str) -> Result<usize> {
    // The counting can be done on each stone individually and then summing up.
    //
    // Try brute-forcing the answer in 2 phases:
    //
    // 1. The first 40 rounds can be done like part 1 within reasonable time (half a minute?).
    // 2. The remaining 35 rounds we do per-stone, caching the count for unique stone values.

    let mut stones = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    // Phase 1 (40 rounds)
    for _ in 0..40 {
        stones = stones.iter().flat_map(transform).collect();
    }

    // After 40 rounds:
    // Number of stones: 112414503
    // Number of unique stones: 1649

    // Phase 2 (35 rounds, one stone at a time)

    let mut count = 0usize;
    let mut cache: HashMap<u64, usize> = HashMap::new();

    for stone in stones {
        cache.entry(stone).or_insert_with(|| {
            let mut stones = vec![stone];

            for _ in 0..35 {
                stones = stones.iter().flat_map(transform).collect();
            }

            stones.len()
        });

        count += cache[&stone];
    }

    Ok(count)
}

/// Transforms a stone based on puzzle rules.
fn transform(stone: &u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        x if *x > 9 && x.ilog10() % 2 == 1 => {
            let x = *x;
            let divisor = 10u64.pow((x.ilog10() + 1) / 2);

            vec![x / divisor, x % divisor]
        }
        x => vec![x * 2024],
    }
}
