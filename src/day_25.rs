use std::{fs::read_to_string, str::FromStr};

use anyhow::{anyhow, Result};

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

const INPUT_FILE: &str = "inputs/day-25.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(EXAMPLE_INPUT.trim())
}

pub fn run_part_1() -> Result<usize> {
    part_1(read_to_string(INPUT_FILE)?.trim())
}

fn part_1(input: &str) -> Result<usize> {
    let (locks, keys) = parse_input_into_locks_and_keys(input)?;

    Ok(locks
        .iter()
        .map(|lock| keys.iter().filter(|key| key.fits_in(lock)).count())
        .sum())
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

                line.as_bytes().iter().enumerate().for_each(|(col, b)| {
                    if *b == b'#' {
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

                    line.as_bytes().iter().enumerate().for_each(|(col, b)| {
                        if *b == b'#' {
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

fn parse_input_into_locks_and_keys(input: &str) -> Result<(Vec<Lock>, Vec<Key>)> {
    let mut locks = Vec::default();
    let mut keys = Vec::default();

    let segments = input.split_terminator("\n\n").collect::<Vec<_>>();

    segments.iter().try_for_each(|s| match s {
        s if s.starts_with("#") => {
            locks.push(Lock::from_str(s)?);

            Ok(())
        }
        s if s.ends_with("#") => {
            keys.push(Key::from_str(s)?);

            Ok(())
        }
        _ => Err(anyhow!("Cannot parse segment: {}", s)),
    })?;

    Ok((locks, keys))
}
