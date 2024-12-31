use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-22.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u64> {
    input
        .split_terminator("\n")
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(generate_secret_seq)
        .map(|seq| {
            seq.last()
                .copied()
                .ok_or(anyhow!("Invalid secret sequence generated"))
        })
        .sum()
}

fn part_2(input: String) -> Result<i64> {
    input
        .split_terminator("\n")
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(generate_secret_seq)
        .map(convert_secret_to_banana_seq)
        .map(map_delta_seq_to_bananas)
        .fold(HashMap::new(), |mut acc, seq_map| {
            // Sum up the banana amounts for each last-4-banana-delta.

            seq_map.iter().for_each(|(delta_seq, bananas)| {
                acc.entry(delta_seq.to_owned())
                    .and_modify(|sum| *sum += *bananas)
                    .or_insert(*bananas);
            });

            acc
        })
        .into_values()
        .max()
        .ok_or(anyhow!("Cannot find highest price"))
}

fn mix(secret: u64, intermediate_result: u64) -> u64 {
    intermediate_result ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn step_1(secret: u64) -> u64 {
    prune(mix(secret, secret * 64))
}

fn step_2(secret: u64) -> u64 {
    prune(mix(secret, secret / 32))
}

fn step_3(secret: u64) -> u64 {
    prune(mix(secret, secret * 2048))
}

fn next_secret(secret: u64) -> u64 {
    step_3(step_2(step_1(secret)))
}

fn generate_secret_seq(secret_seed: u64) -> Vec<u64> {
    let mut secret = secret_seed;
    let mut secret_seq = vec![secret];

    for _ in 0..2000 {
        secret = next_secret(secret);
        secret_seq.push(secret);
    }

    secret_seq
}

fn convert_secret_to_banana_seq(secret_seq: Vec<u64>) -> Vec<i64> {
    secret_seq
        .iter()
        .map(|secret| (secret % 10) as i64)
        .collect()
}

/// Converts the sequence of bananas to the mapping of (last 4 banana-delta) to (banana amount).
///
/// E.g. "-1,-1,0,2" => 6
fn map_delta_seq_to_bananas(banana_seq: Vec<i64>) -> HashMap<String, i64> {
    let mut delta_seq_to_price_map = HashMap::new();

    for i in 0..banana_seq.len() {
        if i < 4 {
            continue;
        }

        let delta_seq = format!(
            "{},{},{},{}",
            banana_seq[i - 3] - banana_seq[i - 4],
            banana_seq[i - 2] - banana_seq[i - 3],
            banana_seq[i - 1] - banana_seq[i - 2],
            banana_seq[i] - banana_seq[i - 1]
        );

        // Only the first occurrence matter.
        delta_seq_to_price_map
            .entry(delta_seq)
            .or_insert(banana_seq[i]);
    }

    delta_seq_to_price_map
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_INPUT: &str = r"
1
10
100
2024
";

    const EXAMPLE_2_INPUT: &str = r"
1
2
3
2024
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_1_INPUT.trim().to_string())?, 37327623);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_2_INPUT.trim().to_string())?, 23);

        Ok(())
    }
}
