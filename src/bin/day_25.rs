use std::str::FromStr;

use anyhow::{anyhow, Result};

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-25.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (locks, keys) = parse_input_into_locks_and_keys(input)?;

    Ok(locks
        .iter()
        .map(|lock| keys.iter().filter(|key| key.fits_in(lock)).count())
        .sum())
}

fn part_2(_input: String) -> Result<()> {
    Err(anyhow!("No part 2"))
}

struct Lock {
    pins: [u8; 5],
}

impl FromStr for Lock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if !s.starts_with("#") {
            return Err(anyhow!("Not lock pattern: {}", s));
        }

        let pins = s
            .split_terminator("\n")
            .skip(1)
            .take(5)
            .fold([0, 0, 0, 0, 0], |acc, line| {
                let mut acc = acc;

                line.as_bytes().iter().enumerate().for_each(|(col, byte)| {
                    if *byte == b'#' {
                        acc[col] += 1;
                    }
                });

                acc
            });

        Ok(Lock { pins })
    }
}

struct Key {
    heights: [u8; 5],
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if !s.ends_with("#") {
            return Err(anyhow!("Not key pattern: {}", s));
        }

        let heights =
            s.split_terminator("\n")
                .skip(1)
                .take(5)
                .fold([0, 0, 0, 0, 0], |acc, line| {
                    let mut acc = acc;

                    line.as_bytes().iter().enumerate().for_each(|(col, byte)| {
                        if *byte == b'#' {
                            acc[col] += 1;
                        }
                    });

                    acc
                });

        Ok(Key { heights })
    }
}

impl Key {
    /// Checks if this [Key] fits into the given [lock].
    fn fits_in(&self, lock: &Lock) -> bool {
        self.heights
            .iter()
            .zip(lock.pins.iter())
            .all(|(height, pin)| height + pin <= 5)
    }
}

fn parse_input_into_locks_and_keys(input: String) -> Result<(Vec<Lock>, Vec<Key>)> {
    let mut locks = Vec::default();
    let mut keys = Vec::default();

    let segments = input.split_terminator("\n\n").collect::<Vec<_>>();

    segments.iter().try_for_each(|segment| match segment {
        s if s.starts_with("#") => {
            locks.push(Lock::from_str(s)?);

            Ok(())
        }
        s if s.ends_with("#") => {
            keys.push(Key::from_str(s)?);

            Ok(())
        }
        _ => Err(anyhow!("Cannot parse segment: {}", segment)),
    })?;

    Ok((locks, keys))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 3);

        Ok(())
    }
}
