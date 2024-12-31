use std::{collections::HashMap, sync::LazyLock};

use anyhow::{anyhow, Result};

static NUMERIC_PAD_MOVEMENTS: LazyLock<HashMap<(u8, u8), &str>> =
    LazyLock::new(init_numeric_pad_movements);

static DIRECTIONAL_PAD_MOVEMENTS: LazyLock<HashMap<(u8, u8), &str>> =
    LazyLock::new(init_directional_pad_movements);

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-21.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(codes: String) -> Result<usize> {
    let first_redirection =
        keypad_redirection(&DIRECTIONAL_PAD_MOVEMENTS, NUMERIC_PAD_MOVEMENTS.clone());

    let second_redirection = keypad_redirection(&DIRECTIONAL_PAD_MOVEMENTS, first_redirection);

    codes
        .split_terminator("\n")
        .map(|code| {
            let code = format!("{}{}", "A", code);

            let sequence = code
                .as_bytes()
                .windows(2)
                .map(|bytes| second_redirection[&(bytes[0], bytes[1])].to_owned())
                .collect::<Vec<String>>()
                .join("");

            let split_code = code.split_terminator("A").collect::<Vec<_>>();

            let Some(code_number) = split_code.get(1) else {
                return Err(anyhow!("Cannot retrieve code_number from code: {}", code));
            };

            Ok(sequence.len() * code_number.parse::<usize>()?)
        })
        .sum()
}

fn part_2(codes: String) -> Result<usize> {
    let numeric_pad_movements = _init_numeric_pad_movements_for_input();

    let mut repeated_redirection =
        keypad_redirection(&DIRECTIONAL_PAD_MOVEMENTS, numeric_pad_movements);

    for _ in 0..24 {
        repeated_redirection = keypad_redirection(&DIRECTIONAL_PAD_MOVEMENTS, repeated_redirection);
    }

    codes
        .split_terminator("\n")
        .map(|code| {
            let code = format!("{}{}", "A", code);

            let sequence = code
                .as_bytes()
                .windows(2)
                .map(|bytes| repeated_redirection[&(bytes[0], bytes[1])].to_owned())
                .collect::<Vec<String>>()
                .join("");

            let split_code = code.split_terminator("A").collect::<Vec<_>>();

            let Some(code_number) = split_code.get(1) else {
                return Err(anyhow!("Cannot retrieve code_number from code: {}", code));
            };

            Ok(sequence.len() * code_number.parse::<usize>()?)
        })
        .sum()
}

fn init_numeric_pad_movements() -> HashMap<(u8, u8), &'static str> {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    //
    // Notes:
    // 1. 'A' at the end of sequence is common.
    // 2. Sequence from x to y can be mirrored to get sequence from y to x.
    // 3. Favour consecutive presses of the same button.
    // 4. Favour '<', then '^' or 'v', then '>'.

    HashMap::from([
        ((b'A', b'A'), "A"),
        ((b'A', b'0'), "<A"),
        ((b'A', b'1'), "^<<A"),
        ((b'A', b'2'), "<^A"),
        ((b'A', b'3'), "^A"),
        ((b'A', b'4'), "^^<<A"),
        ((b'A', b'5'), "<^^A"),
        ((b'A', b'6'), "^^A"),
        ((b'A', b'7'), "^^^<<A"),
        ((b'A', b'8'), "<^^^A"),
        ((b'A', b'9'), "^^^A"),
        ((b'0', b'A'), ">A"),
        ((b'0', b'0'), "A"),
        ((b'0', b'1'), "^<A"),
        ((b'0', b'2'), "^A"),
        ((b'0', b'3'), "^>A"),
        ((b'0', b'4'), "^^<A"),
        ((b'0', b'5'), "^^A"),
        ((b'0', b'6'), "^^>A"),
        ((b'0', b'7'), "^^^<A"),
        ((b'0', b'8'), "^^^A"),
        ((b'0', b'9'), "^^^>A"),
        ((b'1', b'A'), ">>vA"),
        ((b'1', b'0'), ">vA"),
        ((b'1', b'1'), "A"),
        ((b'1', b'2'), ">A"),
        ((b'1', b'3'), ">>A"),
        ((b'1', b'4'), "^A"),
        ((b'1', b'5'), "^>A"),
        ((b'1', b'6'), "^>>A"),
        ((b'1', b'7'), "^^A"),
        ((b'1', b'8'), "^^>A"),
        ((b'1', b'9'), "^^>>A"),
        ((b'2', b'A'), "v>A"),
        ((b'2', b'0'), "vA"),
        ((b'2', b'1'), "<A"),
        ((b'2', b'2'), "A"),
        ((b'2', b'3'), ">A"),
        ((b'2', b'4'), "<^A"),
        ((b'2', b'5'), "^A"),
        ((b'2', b'6'), "^>A"),
        ((b'2', b'7'), "<^^A"),
        ((b'2', b'8'), "^^A"),
        ((b'2', b'9'), "^^>A"),
        ((b'3', b'A'), "vA"),
        ((b'3', b'0'), "<vA"),
        ((b'3', b'1'), "<<A"),
        ((b'3', b'2'), "<A"),
        ((b'3', b'3'), "A"),
        ((b'3', b'4'), "<<^A"),
        ((b'3', b'5'), "<^A"),
        ((b'3', b'6'), "^A"),
        ((b'3', b'7'), "<<^^A"),
        ((b'3', b'8'), "<^^A"),
        ((b'3', b'9'), "^^A"),
        ((b'4', b'A'), ">>vvA"),
        ((b'4', b'0'), ">vvA"),
        ((b'4', b'1'), "vA"),
        ((b'4', b'2'), "v>A"),
        ((b'4', b'3'), "v>>A"),
        ((b'4', b'4'), "A"),
        ((b'4', b'5'), ">A"),
        ((b'4', b'6'), ">>A"),
        ((b'4', b'7'), "^A"),
        ((b'4', b'8'), "^>A"),
        ((b'4', b'9'), "^>>A"),
        ((b'5', b'A'), "vv>A"),
        ((b'5', b'0'), "vvA"),
        ((b'5', b'1'), "<vA"),
        ((b'5', b'2'), "vA"),
        ((b'5', b'3'), "v>A"),
        ((b'5', b'4'), "<A"),
        ((b'5', b'5'), "A"),
        ((b'5', b'6'), ">A"),
        ((b'5', b'7'), "<^A"),
        ((b'5', b'8'), "^A"),
        ((b'5', b'9'), "^>A"),
        ((b'6', b'A'), "vvA"),
        ((b'6', b'0'), "<vvA"),
        ((b'6', b'1'), "<<vA"),
        ((b'6', b'2'), "<vA"),
        ((b'6', b'3'), "vA"),
        ((b'6', b'4'), "<<A"),
        ((b'6', b'5'), "<A"),
        ((b'6', b'6'), "A"),
        ((b'6', b'7'), "<<^A"),
        ((b'6', b'8'), "<^A"),
        ((b'6', b'9'), "^A"),
        ((b'7', b'A'), ">>vvvA"),
        ((b'7', b'0'), ">vvvA"),
        ((b'7', b'1'), "vvA"),
        ((b'7', b'2'), "vv>A"),
        ((b'7', b'3'), "vv>>A"),
        ((b'7', b'4'), "vA"),
        ((b'7', b'5'), "v>A"),
        ((b'7', b'6'), "v>>A"),
        ((b'7', b'7'), "A"),
        ((b'7', b'8'), ">A"),
        ((b'7', b'9'), ">>A"),
        ((b'8', b'A'), "vvv>A"),
        ((b'8', b'0'), "vvvA"),
        ((b'8', b'1'), "<vvA"),
        ((b'8', b'2'), "vvA"),
        ((b'8', b'3'), "vv>A"),
        ((b'8', b'4'), "<vA"),
        ((b'8', b'5'), "vA"),
        ((b'8', b'6'), "v>A"),
        ((b'8', b'7'), "<A"),
        ((b'8', b'8'), "A"),
        ((b'8', b'9'), ">A"),
        ((b'9', b'A'), "vvvA"),
        ((b'9', b'0'), "<vvvA"),
        ((b'9', b'1'), "<<vvA"),
        ((b'9', b'2'), "<vvA"),
        ((b'9', b'3'), "vvA"),
        ((b'9', b'4'), "<<vA"),
        ((b'9', b'5'), "<vA"),
        ((b'9', b'6'), "vA"),
        ((b'9', b'7'), "<<A"),
        ((b'9', b'8'), "<A"),
        ((b'9', b'9'), "A"),
    ])
}

fn init_directional_pad_movements() -> HashMap<(u8, u8), &'static str> {
    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    //
    // Notes:
    // 1. 'A' at the end of sequence is common.
    // 2. Sequence from x to y can be mirrored to get sequence from y to x.
    // 3. Favour consecutive presses of the same button.
    // 4. Favour '<', then '^' or 'v', then '>'.

    HashMap::from([
        ((b'A', b'A'), "A"),
        ((b'A', b'^'), "<A"),
        ((b'A', b'>'), "vA"),
        ((b'A', b'v'), "<vA"),
        ((b'A', b'<'), "v<<A"),
        ((b'^', b'A'), ">A"),
        ((b'^', b'^'), "A"),
        ((b'^', b'>'), "v>A"),
        ((b'^', b'v'), "vA"),
        ((b'^', b'<'), "v<A"),
        ((b'>', b'A'), "^A"),
        ((b'>', b'^'), "<^A"),
        ((b'>', b'>'), "A"),
        ((b'>', b'v'), "<A"),
        ((b'>', b'<'), "<<A"),
        ((b'v', b'A'), "^>A"),
        ((b'v', b'^'), "^A"),
        ((b'v', b'>'), ">A"),
        ((b'v', b'v'), "A"),
        ((b'v', b'<'), "<A"),
        ((b'<', b'A'), ">>^A"),
        ((b'<', b'^'), ">^A"),
        ((b'<', b'>'), ">>A"),
        ((b'<', b'v'), ">A"),
        ((b'<', b'<'), "A"),
    ])
}

fn _init_numeric_pad_movements_for_input() -> HashMap<(u8, u8), &'static str> {
    // Include only pairs that are used in input.
    HashMap::from([
        ((b'A', b'2'), "<^A"),
        ((b'A', b'3'), "^A"),
        ((b'A', b'4'), "^^<<A"),
        ((b'A', b'5'), "<^^A"),
        ((b'0', b'8'), "^^^A"),
        ((b'2', b'4'), "<^A"),
        ((b'3', b'A'), "vA"),
        ((b'3', b'8'), "<^^A"),
        ((b'4', b'5'), ">A"),
        ((b'4', b'6'), ">>A"),
        ((b'5', b'0'), "vvA"),
        ((b'5', b'9'), "^>A"),
        ((b'6', b'A'), "vvA"),
        ((b'8', b'A'), "vvv>A"),
        ((b'8', b'6'), "v>A"),
        ((b'9', b'A'), "vvvA"),
        ((b'9', b'3'), "vvA"),
    ])
}

/// Substitutes the mappings in target by breaking down and replacing each movement with the
/// sequence from interface.
///
/// Consumes target.
fn keypad_redirection<T>(
    interface: &HashMap<(u8, u8), &str>,
    target: HashMap<(u8, u8), T>,
) -> HashMap<(u8, u8), String>
where
    T: Into<String>,
{
    let mut movements = HashMap::new();

    for (key, value) in target.into_iter() {
        let value = format!("{}{}", "A", value.into());

        let sequence = value
            .as_bytes()
            .windows(2)
            .map(|bv| interface[&(bv[0], bv[1])])
            .collect::<Vec<_>>()
            .join("");

        movements.insert(key, sequence);
    }

    movements
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
029A
980A
179A
456A
379A
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 126384);

        Ok(())
    }
}
