use std::collections::HashMap;

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-11.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let mut stones = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    for _ in 0..25 {
        stones = blink_stones(stones);
    }

    Ok(stones.len())
}

fn part_2(input: &str) -> Result<u64> {
    // Each stone expands independently of each other, so we can just blink them separately and then
    // sum up the results.

    let stones = input
        .split_whitespace()
        .map(str::parse::<u64>)
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();
    Ok(stones
        .into_iter()
        .map(|stone| count_stones_after_blinks(stone, 75, &mut cache))
        .sum())
}

fn blink(stone: u64) -> Vec<u64> {
    match stone {
        0 => vec![1],
        d if d.ilog10() % 2 == 1 => {
            let half_digits = d.ilog10().div_ceil(2);

            vec![d / 10u64.pow(half_digits), d % 10u64.pow(half_digits)]
        }
        _ => vec![stone * 2024],
    }
}

fn blink_stones(stones: Vec<u64>) -> Vec<u64> {
    stones.into_iter().flat_map(blink).collect()
}

fn count_stones_after_blinks(stone: u64, blinks: u8, cache: &mut HashMap<(u64, u8), u64>) -> u64 {
    if cache.contains_key(&(stone, blinks)) {
        return cache[&(stone, blinks)];
    }

    if blinks == 0 {
        return *cache.entry((stone, 0)).or_insert(1);
    }

    let count = blink(stone)
        .into_iter()
        .map(|stone| count_stones_after_blinks(stone, blinks - 1, cache))
        .sum();
    *cache.entry((stone, blinks)).or_insert(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1("125 17")?, 55312);

        Ok(())
    }
}
