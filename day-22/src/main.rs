use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-22.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    Ok(input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(secret_2000)
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let secrets = input
        .lines()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    max_bananas_purchasable(&secrets)
}

fn secret_2000(mut secret: u64) -> u64 {
    for _ in 0..2000 {
        secret = next_secret(secret);
    }

    secret
}

fn next_secret(mut secret: u64) -> u64 {
    secret = mix_and_prune(secret, secret * 64);
    secret = mix_and_prune(secret, secret / 32);
    secret = mix_and_prune(secret, secret * 2048);

    secret
}

fn mix_and_prune(secret: u64, number: u64) -> u64 {
    (secret ^ number) % 16777216
}

fn max_bananas_purchasable(secrets: &[u64]) -> Result<u64> {
    let mut bananas_for_sequence = HashMap::new();
    for secret in secrets {
        add_bananas_for_sequence(*secret, &mut bananas_for_sequence);
    }

    bananas_for_sequence
        .into_values()
        .max()
        .ok_or(anyhow!("Cannot find max bananas purchasable"))
}

fn add_bananas_for_sequence(secret: u64, bananas_for_sequence: &mut HashMap<[i64; 4], u64>) {
    // For each monkey, only the first time the sequence appears can count.
    let mut sequences_added = HashSet::new();

    (0..2000)
        .scan(secret, |state, _| {
            // Calculate bananas and diff from prev secret.

            let prev_bananas = *state % 10;

            *state = next_secret(*state);

            let bananas = *state % 10;
            let diff = bananas as i64 - prev_bananas as i64;

            Some((bananas, diff))
        })
        .collect::<Vec<_>>()
        .windows(4)
        .for_each(|window| {
            let sequence = [window[0].1, window[1].1, window[2].1, window[3].1];

            if sequences_added.insert(sequence) {
                *bananas_for_sequence.entry(sequence).or_default() += window[3].0;
            }
        });
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
1
10
100
2024
";

        assert_eq!(part_1(trim_newlines(example))?, 37327623);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let example = r"
1
2
3
2024
";

        assert_eq!(part_2(trim_newlines(example))?, 23);

        Ok(())
    }
}
