use std::collections::HashMap;

use anyhow::{anyhow, Result};

const INPUT_FILE: &str = "inputs/day-22.txt";

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u64> {
    input
        .split_terminator("\n")
        .map(|l| l.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(generate_secret_sequence)
        .map(|seq| {
            seq.last()
                .copied()
                .ok_or(anyhow!("Invalid secret sequence generated"))
        })
        .sum()
}

fn part_2(input: String) -> Result<i64> {
    let change_sequence_maps = input
        .split_terminator("\n")
        .map(|l| l.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(generate_secret_sequence)
        .map(convert_secret_to_banana_sequence)
        .map(map_change_sequence)
        .collect::<Vec<_>>();

    // Sum up the banana amounts for each last-4-banana-delta.
    let mut combined_change_sequence_map: HashMap<String, i64> = HashMap::new();
    change_sequence_maps.iter().for_each(|h| {
        h.iter().for_each(|(change_sequence, bananas)| {
            combined_change_sequence_map
                .entry(change_sequence.to_owned())
                .and_modify(|acc| *acc += *bananas)
                .or_insert(*bananas);
        })
    });

    combined_change_sequence_map
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

fn generate_secret_sequence(secret_seed: u64) -> Vec<u64> {
    let mut secret = secret_seed;
    let mut secret_sequence = vec![secret];

    for _ in 0..2000 {
        secret = next_secret(secret);
        secret_sequence.push(secret);
    }

    secret_sequence
}

fn convert_secret_to_banana_sequence(secret_sequence: Vec<u64>) -> Vec<i64> {
    secret_sequence.iter().map(|s| (s % 10) as i64).collect()
}

/// Converts the sequence of bananas to the mapping of (last 4 banana-delta) to (banana amount).
///
/// E.g. "-1,-1,0,2" => 6
fn map_change_sequence(banana_sequence: Vec<i64>) -> HashMap<String, i64> {
    let mut change_sequence_to_price = HashMap::new();

    for i in 0..banana_sequence.len() {
        if i < 4 {
            continue;
        }

        let change_sequence = format!(
            "{},{},{},{}",
            banana_sequence[i - 3] - banana_sequence[i - 4],
            banana_sequence[i - 2] - banana_sequence[i - 3],
            banana_sequence[i - 1] - banana_sequence[i - 2],
            banana_sequence[i] - banana_sequence[i - 1]
        );

        // Only the first occurrence matter.
        change_sequence_to_price
            .entry(change_sequence)
            .or_insert(banana_sequence[i]);
    }

    change_sequence_to_price
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
