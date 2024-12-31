use std::collections::HashSet;

use anyhow::{anyhow, Result};

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-5.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    let (updates, rules) = parse_input(&input)?;

    updates
        .iter()
        .filter(|update| in_right_order(update, &rules))
        .map(|update| middle_page_number_by_update_len(update))
        .sum()
}

fn part_2(input: String) -> Result<u32> {
    let (updates, rules) = parse_input(&input)?;

    updates
        .iter()
        .filter(|update| !in_right_order(update, &rules))
        .map(|update| middle_page_number_by_power(update, &rules))
        .sum()
}

/// Converts input into an "updates" list and a "rules" set.
fn parse_input(input: &str) -> Result<(Vec<Vec<&str>>, HashSet<&str>)> {
    let input = input.split_terminator("\n\n").collect::<Vec<_>>();
    let [rules_input, updates_input] = input[..] else {
        return Err(anyhow!("Cannot split input into rules and updates"));
    };

    let rules = HashSet::from_iter(rules_input.split_terminator("\n"));

    let updates = updates_input
        .split_terminator("\n")
        .map(|update| update.split_terminator(",").collect())
        .collect();

    Ok((updates, rules))
}

/// Checks whether update is in right order i.e. each subsequent page has decreasing "power".
fn in_right_order(update: &[&str], rules: &HashSet<&str>) -> bool {
    update
        .iter()
        .rev()
        .enumerate()
        .all(|(index, page)| get_power(page, update, rules) == index)
}

fn middle_page_number_by_update_len(update: &[&str]) -> Result<u32> {
    let middle_index = update.len() / 2;

    let Some(middle_page) = update.get(middle_index) else {
        return Err(anyhow!(
            "Cannot retrieve middle page number from {:?}",
            update
        ));
    };

    middle_page
        .parse()
        .map_err(|e| anyhow!("Cannot parse middle page number: {}", e))
}

fn middle_page_number_by_power(update: &[&str], rules: &HashSet<&str>) -> Result<u32> {
    let middle_power = update.len() / 2;

    let Some(middle_page) = update
        .iter()
        .find(|page| get_power(page, update, rules) == middle_power)
    else {
        return Err(anyhow!(
            "Cannot get element with middle power from {:?}",
            update
        ));
    };

    middle_page
        .parse::<u32>()
        .map_err(|e| anyhow!("Cannot parse middle page number: {}", e))
}

/// Gets the "power" value of page_number within the group of pages inside update.
///
/// The "power" value is the number of rules that put that number in front of the other numbers.
/// E.g. for this input of a correct update:
///
/// A|B
/// A|C
/// B|C
///
/// A,B,C
///
/// The power of A is 2. The power of B is 1, The power of C is 0.
/// Note that the power series follows the reversed index order of the update.
fn get_power(page_number: &str, update: &[&str], rules: &HashSet<&str>) -> usize {
    update
        .iter()
        .filter(|other| {
            let search_term = [page_number, other].join("|");

            rules.contains(search_term.as_str())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 143);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 123);

        Ok(())
    }
}
