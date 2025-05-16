use std::collections::HashMap;

use anyhow::{Result, anyhow};

use button_sequences::BUTTON_SEQUENCES;
use cli::{Part, get_part};

mod button_sequences;

fn main() {
    match get_part("input/day-21.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            let numeric_part = code
                .strip_suffix("A")
                .ok_or(anyhow!("Invalid code: {}", code))?
                .parse::<u64>()?;
            let button_presses = code_button_presses(code, 3, &mut cache);

            Ok(button_presses * numeric_part)
        })
        .sum()
}

fn part_2(input: &str) -> Result<u64> {
    let mut cache = HashMap::new();
    input
        .lines()
        .map(|code| {
            let numeric_part = code
                .strip_suffix("A")
                .ok_or(anyhow!("Invalid code: {}", code))?
                .parse::<u64>()?;
            let button_presses = code_button_presses(code, 26, &mut cache);

            Ok(button_presses * numeric_part)
        })
        .sum()
}

fn code_button_presses(
    code: &str,
    directional_keypads: u8,
    cache: &mut HashMap<(u8, u8, u8), u64>,
) -> u64 {
    let mut code_sequence = vec![b'A'];
    code_sequence.extend(code.bytes());
    code_sequence
        .windows(2)
        .map(|window| count_button_presses(window[0], window[1], directional_keypads - 1, cache))
        .sum()
}

fn count_button_presses(
    start_pos: u8,
    end_pos: u8,
    directional_keypads: u8,
    cache: &mut HashMap<(u8, u8, u8), u64>,
) -> u64 {
    if cache.contains_key(&(start_pos, end_pos, directional_keypads)) {
        return cache[&(start_pos, end_pos, directional_keypads)];
    }

    let button_sequence = &BUTTON_SEQUENCES[&(start_pos, end_pos)];

    if directional_keypads == 0 {
        return *cache
            .entry((start_pos, end_pos, directional_keypads))
            .or_insert(button_sequence.len() as u64 - 1);
    }

    let redirected_presses = button_sequence
        .windows(2)
        .map(|window| count_button_presses(window[0], window[1], directional_keypads - 1, cache))
        .sum();
    *cache
        .entry((start_pos, end_pos, directional_keypads))
        .or_insert(redirected_presses)
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
029A
980A
179A
456A
379A
";

        assert_eq!(part_1(trim_newlines(example))?, 126384);

        Ok(())
    }
}
