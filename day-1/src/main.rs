use std::collections::HashMap;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-1.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let (mut left_list, mut right_list) = parse_input_into_separate_lists(input)?;
    left_list.sort();
    right_list.sort();

    Ok(left_list
        .into_iter()
        .zip(right_list)
        .map(|(left_value, right_value)| left_value.abs_diff(right_value))
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let (left_list, right_list) = parse_input_into_separate_lists(input)?;

    let mut number_counts: HashMap<u32, u32> = HashMap::new();
    right_list
        .into_iter()
        .for_each(|right_value| *number_counts.entry(right_value).or_default() += 1);

    Ok(left_list
        .into_iter()
        .map(|left_value| left_value * *number_counts.entry(left_value).or_default())
        .sum())
}

fn parse_input_into_separate_lists(input: &str) -> Result<(Vec<u32>, Vec<u32>)> {
    Ok(input
        .lines()
        .map(|line| {
            let Some((left_value, right_value)) = line.split_once("   ") else {
                return Err(anyhow!(
                    "Cannot split input into left and right values: {}",
                    line
                ));
            };

            Ok((left_value.parse::<u32>()?, right_value.parse::<u32>()?))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .unzip())
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 11);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 31);

        Ok(())
    }
}
