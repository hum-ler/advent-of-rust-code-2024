use std::collections::HashSet;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-5.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let (rules, updates) = parse_input_into_rules_and_updates(input)?;

    Ok(updates
        .into_iter()
        .filter(|update| in_order(update, &rules))
        .map(|update| middle_page_number(&update))
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let (rules, updates) = parse_input_into_rules_and_updates(input)?;

    Ok(updates
        .into_iter()
        .filter(|update| !in_order(update, &rules))
        .map(|update| rearrange(update, &rules))
        .map(|update| middle_page_number(&update))
        .sum())
}

type Rule = (u32, u32);
type Update = Vec<u32>;

fn parse_input_into_rules_and_updates(input: &str) -> Result<(HashSet<Rule>, Vec<Update>)> {
    let Some((rules, update)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into rules and updates: {}",
            input
        ));
    };

    let rules = rules
        .lines()
        .map(|line| {
            let Some((before, after)) = line.split_once("|") else {
                return Err(anyhow!(
                    "Cannot split input into before and after: {}",
                    line
                ));
            };

            Ok((before.parse()?, after.parse()?))
        })
        .collect::<Result<HashSet<_>>>()?;

    let updates = update
        .lines()
        .map(|line| {
            line.split_terminator(",")
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok((rules, updates))
}

fn in_order(update: &Update, rules: &HashSet<Rule>) -> bool {
    update.iter().enumerate().all(|(index, page)| {
        update[index + 1..]
            .iter()
            .all(|page_after| rules.contains(&(*page, *page_after)))
    })
}

fn middle_page_number(update: &Update) -> u32 {
    update[update.len() / 2]
}

fn rearrange(update: Update, rules: &HashSet<Rule>) -> Update {
    let mut rearranged = update.clone();
    rearranged.sort_by_key(|page| {
        update
            .iter()
            .filter(|&other_page| rules.contains(&(*page, *other_page)))
            .count()
    });
    rearranged.reverse();

    rearranged
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 143);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 123);

        Ok(())
    }
}
