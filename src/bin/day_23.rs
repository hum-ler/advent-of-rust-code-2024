use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-23.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let mut t_triplets: HashSet<[String; 3]> = HashSet::new();

    let connections = parse_input_to_connections(input)?;

    let t_computers = connections
        .keys()
        .filter(|computer| computer.starts_with("t"))
        .collect::<Vec<_>>();

    for t_computer in t_computers {
        get_triplets(t_computer, &connections)
            .into_iter()
            .for_each(|mut triplet| {
                triplet.sort();
                t_triplets.insert(triplet);
            });
    }

    Ok(t_triplets.len())
}

fn part_2(input: String) -> Result<String> {
    // Each computer is connected to 13 others. Work downwards until we find the largest complete
    // graph.
    part_2_with_prune(input, 12)
}

fn part_2_with_prune(input: String, prune: usize) -> Result<String> {
    // We are looking for the largest complete subgraph.

    let connections = parse_input_to_connections(input)?;

    let mut largest_complete_graph = connections.keys().fold(Vec::default(), |acc, computer| {
        let mut graph = find_largest_complete_graph(&connections[computer], &connections, prune);

        if graph.len() + 1 > acc.len() {
            graph.push(computer.to_owned());
            graph
        } else {
            acc
        }
    });
    largest_complete_graph.sort();

    Ok(largest_complete_graph.join(","))
}

fn parse_input_to_connections(input: String) -> Result<HashMap<String, Vec<String>>> {
    let re = Regex::new(r"^(?<first>\w\w)-(?<second>\w\w)$")?;

    let mut connections = HashMap::new();

    input.split_terminator("\n").try_for_each(|line| {
        let captures = re
            .captures(line)
            .ok_or(anyhow!("Cannot parse line: {}", line))?;

        let first_computer = &captures["first"];
        let second_computer = &captures["second"];

        connections
            .entry(first_computer.to_owned())
            .and_modify(|computers: &mut Vec<String>| computers.push(second_computer.to_owned()))
            .or_insert(vec![second_computer.to_owned()]);

        connections
            .entry(second_computer.to_owned())
            .and_modify(|computers| computers.push(first_computer.to_owned()))
            .or_insert(vec![first_computer.to_owned()]);

        Ok::<_, anyhow::Error>(())
    })?;

    Ok(connections)
}

fn get_triplets(computer: &String, connections: &HashMap<String, Vec<String>>) -> Vec<[String; 3]> {
    assert!(connections.contains_key(computer));

    let mut triplets = Vec::default();

    connections[computer]
        .iter()
        .tuple_combinations()
        .for_each(|(first, second)| {
            if are_connected(first, second, connections) {
                triplets.push([computer.to_owned(), first.to_owned(), second.to_owned()]);
            }
        });

    triplets
}

fn are_connected(
    first: &String,
    second: &String,
    connections: &HashMap<String, Vec<String>>,
) -> bool {
    assert!(connections.contains_key(first));
    assert!(connections.contains_key(second));

    connections[first].contains(second)
}

/// Finds the largest complete graph within the computers represented by nodes.
///
/// Stops searching once the length of nodes falls below the value of prune, in which case, an empty
/// [Vec] is returned.
fn find_largest_complete_graph(
    nodes: &[String],
    connections: &HashMap<String, Vec<String>>,
    prune: usize,
) -> Vec<String> {
    if is_complete_graph(nodes, connections) {
        return nodes.to_owned();
    }

    if nodes.len() <= prune {
        return Vec::default();
    }

    nodes
        .iter()
        .enumerate()
        .fold(Vec::default(), |acc, (index, _)| {
            let mut nodes = nodes.to_owned();
            nodes.remove(index);

            let graph = find_largest_complete_graph(&nodes, connections, prune);
            if graph.len() > acc.len() {
                graph
            } else {
                acc
            }
        })
}

fn is_complete_graph(nodes: &[String], connections: &HashMap<String, Vec<String>>) -> bool {
    assert!(!nodes.is_empty());

    if nodes.len() == 1 {
        return true;
    }

    nodes
        .iter()
        .tuple_combinations()
        .all(|(first, second)| are_connected(first, second, connections))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 7);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        // We know the password is 4 computers long, so ignore anything shorter than that.
        assert_eq!(
            part_2_with_prune(EXAMPLE_INPUT.trim().to_string(), 3)?,
            "co,de,ka,ta".to_string()
        );

        Ok(())
    }
}
