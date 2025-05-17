use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-25.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(_)) => println!("No part 2"),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (keys, locks) = parse_input_into_keys_and_locks(input)?;

    Ok(keys
        .iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| (0..5).all(|index| lock[index] + key[index] <= 5))
                .count()
        })
        .sum())
}

type Heights = [u8; 5];
type Keys = Vec<Heights>;
type Locks = Vec<Heights>;

fn parse_input_into_keys_and_locks(input: &str) -> Result<(Keys, Locks)> {
    let schematics = input.split_terminator("\n\n").collect::<Vec<_>>();

    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for schematic in schematics {
        parse_schematic_into_key_or_lock(schematic, &mut keys, &mut locks)?;
    }

    Ok((keys, locks))
}

fn parse_schematic_into_key_or_lock(
    schematic: &str,
    keys: &mut Keys,
    locks: &mut Locks,
) -> Result<()> {
    let lines = schematic.lines().collect::<Vec<_>>();
    if lines.len() != 7 || !(schematic.starts_with("#") || schematic.starts_with(".")) {
        return Err(anyhow!("Invalid schematic: {}", schematic));
    }

    let mut heights: Heights = Default::default();
    for line in lines.into_iter().skip(1).take(5) {
        if line.len() != 5 {
            return Err(anyhow!("Invalid line: {}", line));
        }

        for (index, byte) in line.bytes().enumerate() {
            if byte == b'#' {
                heights[index] += 1;
            }
        }
    }

    if schematic.starts_with("#") {
        locks.push(heights);
    } else {
        keys.push(heights);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
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

        assert_eq!(part_1(trim_newlines(example))?, 3);

        Ok(())
    }
}
