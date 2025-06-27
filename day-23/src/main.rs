use std::collections::{HashMap, HashSet};

use anyhow::{Result, anyhow};
use itertools::Itertools;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-23.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let connections = parse_input_into_connections(input)?;

    Ok(connections
        .iter()
        .filter(|(computer, _)| computer.starts_with("t"))
        .flat_map(|(t_computer, neighbours)| {
            neighbours
                .iter()
                .tuple_combinations()
                .filter_map(|(&computer_1, &computer_2)| {
                    if connections[computer_1].contains(&computer_2) {
                        // Get rid of overlaps between triplets with 2 or more "t" computers.
                        let mut triplet = [*t_computer, computer_1, computer_2];
                        triplet.sort();

                        Some(triplet)
                    } else {
                        None
                    }
                })
        })
        .unique()
        .count())
}

fn part_2(input: &str) -> Result<String> {
    let connections = parse_input_into_connections(input)?;

    let mut max_clique = HashSet::new();
    bron_kerborsh(
        HashSet::new(),
        connections.keys().copied().collect::<HashSet<_>>(),
        HashSet::new(),
        &mut max_clique,
        &connections,
    );

    Ok(max_clique.into_iter().sorted().join(","))
}

fn parse_input_into_connections(input: &str) -> Result<HashMap<&str, HashSet<&str>>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let Some((computer_1, computer_2)) = line.split_once("-") else {
            return Err(anyhow!(
                "Cannot split input into computers 1 and 2: {}",
                line
            ));
        };

        connections
            .entry(computer_1)
            .or_default()
            .insert(computer_2);
        connections
            .entry(computer_2)
            .or_default()
            .insert(computer_1);
    }

    Ok(connections)
}

fn bron_kerborsh<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() && r.len() > max_clique.len() {
        *max_clique = r;
        return;
    }

    if r.len() + p.len() <= max_clique.len() {
        return;
    }

    for v in p.clone() {
        let mut next_r = r.clone();
        next_r.insert(v);
        let next_p = p.intersection(&connections[v]).copied().collect();
        let next_x = x.intersection(&connections[v]).copied().collect();
        bron_kerborsh(next_r, next_p, next_x, max_clique, connections);

        p.remove(v);
        x.insert(v);
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
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
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 7);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, "co,de,ka,ta");

        Ok(())
    }
}
