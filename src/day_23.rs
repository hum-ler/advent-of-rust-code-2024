use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use crate::{file_to_lines, string_to_lines};

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

const INPUT_FILE: &str = "inputs/day-23.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<usize> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<String> {
    // We know the password is 4 computers long, so ignore anything shorter than that.
    part_2(&string_to_lines(EXAMPLE_INPUT), 3)
}

pub fn run_part_2() -> Result<String> {
    // Each computer is connected to 13 others. Work downwards until we find the largest complete
    // graph.
    part_2(&file_to_lines(INPUT_FILE)?, 12)
}

fn part_1(lines: &[String]) -> Result<usize> {
    let mut t_triplets: HashSet<[String; 3]> = HashSet::new();

    let connections = parse_lines_to_connections(lines)?;

    let t_computers = connections
        .keys()
        .filter(|c| c.starts_with("t"))
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

fn part_2(lines: &[String], prune: usize) -> Result<String> {
    // We are looking for the largest complete subgraph.

    let connections = parse_lines_to_connections(lines)?;

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

fn parse_lines_to_connections(lines: &[String]) -> Result<HashMap<String, Vec<String>>> {
    let re = Regex::new(r"^(?<first>\w\w)-(?<second>\w\w)$")?;

    let mut connections = HashMap::new();

    lines.iter().try_for_each(|line| {
        let captures = re
            .captures(line)
            .ok_or(anyhow!("Cannot parse line: {}", line))?;

        let first_computer = &captures["first"];
        let second_computer = &captures["second"];

        connections
            .entry(first_computer.to_owned())
            .and_modify(|v: &mut Vec<String>| v.push(second_computer.to_owned()))
            .or_insert(vec![second_computer.to_owned()]);

        connections
            .entry(second_computer.to_owned())
            .and_modify(|v| v.push(first_computer.to_owned()))
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

/// Finds the largest complete graph within the computers represented by [nodes].
///
/// Use [prune] to stop searching once the length of [nodes] falls below [prune].
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
