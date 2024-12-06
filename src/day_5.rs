use std::{collections::HashSet, fs::read_to_string};

use anyhow::{anyhow, Result};

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

const INPUT_FILE: &str = "inputs/day-5.txt";

pub fn run_example_1() -> Result<u32> {
    part_1(EXAMPLE_INPUT)
}

pub fn run_part_1() -> Result<u32> {
    part_1(&read_to_string(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<u32> {
    part_2(EXAMPLE_INPUT)
}

pub fn run_part_2() -> Result<u32> {
    part_2(&read_to_string(INPUT_FILE)?)
}

fn part_1(input: &str) -> Result<u32> {
    let (updates, rules) = parse_input(input)?;

    updates
        .iter()
        .filter(|u| in_right_order(u, &rules))
        .map(|u| middle_page_number_by_update_len(u))
        .sum()
}

fn part_2(input: &str) -> Result<u32> {
    let (updates, rules) = parse_input(input)?;

    updates
        .iter()
        .filter(|u| !in_right_order(u, &rules))
        .map(|u| middle_page_number_by_power(u, &rules))
        .sum()
}

/// Converts [input] into an "updates" list and a "rules" set.
fn parse_input(input: &str) -> Result<(Vec<Vec<&str>>, HashSet<&str>)> {
    let input = input.trim().split("\n\n").collect::<Vec<&str>>();
    let [rules_input, updates_input, ..] = input.as_slice() else {
        return Err(anyhow!("Cannot split input into rules and updates"));
    };

    let rules = HashSet::from_iter(rules_input.split("\n"));

    let updates = updates_input
        .split("\n")
        .map(|u| u.split(",").collect())
        .collect::<Vec<_>>();

    Ok((updates, rules))
}

/// Check whether [update] is in right order i.e. each subsequent page has decreasing "power".
fn in_right_order(update: &[&str], rules: &HashSet<&str>) -> bool {
    for i in 0..update.len() {
        if get_power(update[i], update, rules) != update.len() - i - 1 {
            return false;
        }
    }

    true
}

fn middle_page_number_by_update_len(update: &[&str]) -> Result<u32> {
    let middle_index = update.len() / 2;

    if let Some(middle_str) = update.get(middle_index) {
        middle_str
            .parse::<u32>()
            .map_err(|e| anyhow!("Cannot parse middle page number: {}", e))
    } else {
        Err(anyhow!(
            "Cannot retrieve middle page number from {:?}",
            update
        ))
    }
}

fn middle_page_number_by_power(update: &[&str], rules: &HashSet<&str>) -> Result<u32> {
    let middle_power = update.len() / 2;

    if let Some(middle_str) = update
        .iter()
        .find(|u| get_power(u, update, rules) == middle_power)
    {
        middle_str
            .parse::<u32>()
            .map_err(|e| anyhow!("Cannot parse middle page number: {}", e))
    } else {
        Err(anyhow!(
            "Cannot get element with middle power from {:?}",
            update
        ))
    }
}

/// Gets the "power" value of [page_number] within the group of pages inside [update].
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
