use std::{collections::HashMap, fs::read_to_string, sync::RwLock};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

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
    // The counting can be done on each stone individually and then adding up.
    //
    // Try brute-forcing the answer in 2 parts:
    // 1. The first 40 rounds can be done like part 1 within reasonable time (half a minute?).
    // 2. The remaining 35 rounds we do per-stone, caching the count for unique stone values.

    let mut stones = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    // Run 40 rounds first.
    for _ in 0..40 {
        stones = stones.par_iter().flat_map_iter(transform).collect();
    }

    // After 40 rounds:
    //
    // Number of stones: 112414503
    // Number of unique stones: 1649

    // Generate the lookup.
    let unique_stones = stones.iter().unique().copied().collect::<Vec<_>>();
    let cache: RwLock<HashMap<u64, usize>> = RwLock::new(HashMap::new());
    unique_stones.into_par_iter().try_for_each(|stone| {
        let mut stones = vec![stone];

        // Run 35 rounds.
        for _ in 0..35 {
            stones = stones.par_iter().flat_map_iter(transform).collect();
        }

        cache
            .write()
            .map_err(|e| anyhow!("Cannot get write lock on cache: {}", e))?
            .insert(stone, stones.len());

        Ok::<_, anyhow::Error>(())
    })?;

    // Look up the count from the cache.
    let cache = cache
        .read()
        .map_err(|e| anyhow!("Cannot get read lock on cache: {}", e))?;
    Ok(stones.par_iter().map(|stone| cache[stone].to_owned()).sum())
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
