use std::collections::HashMap;

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-24.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let (values, logic_gates) = parse_input_into_values_and_logic_gates(input)?;
    let mut values = extend_values(values, &logic_gates);

    // Just loop until all "z"s are resolved.
    while !values
        .iter()
        .all(|(wire, value)| !wire.starts_with("z") || value.is_some())
    {
        for logic_gate in &logic_gates {
            logic_gate.evaluate(&mut values);
        }
    }

    values_to_number("z", &values)
}

fn part_2(input: &str) -> Result<String> {
    // We are looking at a 45-bit ripple-carry adder.
    //
    // At the very least, we should expect to find the following operations:
    //   - For lsb:
    //     - x00 ^ y00 -> z00
    //     - x00 & y00 -> c01
    //   - For higher bits:
    //     - x<n> ^ y<n> -> i<n>
    //     - x<n> & y<n> -> j<n>
    //     - i<n> & c<n> -> k<n>
    //     - i<n> ^ c<n> -> z<n>
    //     - j<n> | k<n> -> c<n + 1>
    //   - For msb, the last statement becomes:
    //     - j44 | k44 -> z45
    //
    // For 45 bits we should expect 2 + 44 * 5 = 222 operations, which fits the input.
    //
    // Use some pattern matching (Excel works!) to filter out all the correct, expected signals.
    // From here, there are 3 "-> z"s that are not XORs, hence must be incorrect:
    //   - z15
    //   - z05
    //   - z20
    // There are also correspondingly 3 XORs that do not involve x and y:
    //   - fvm XOR mvv
    //   - bhw XOR sth
    //   - gcs XOR hdc
    //
    // Hence the swaps:
    //   - z15 <-> htp
    //   - z05 <-> dkr
    //   - z20 <-> hhh
    //
    // z45 is equivalent to c45, which clears the following:
    //   - x44 AND y44 -> kbb
    //   - gqg AND pfh -> khw
    //   - kbb OR khw -> z45
    //
    // Checking z36, we need to swap: rhv <-> ggk.

    // Perform quick verification.

    let replacements = HashMap::from([
        ("z15", "htp"),
        ("htp", "z15"),
        ("z05", "dkr"),
        ("dkr", "z05"),
        ("z20", "hhh"),
        ("hhh", "z20"),
        ("rhv", "ggk"),
        ("ggk", "rhv"),
    ]);

    let (values, mut logic_gates) = parse_input_into_values_and_logic_gates(input)?;
    let mut values = extend_values(values, &logic_gates);

    logic_gates
        .iter_mut()
        .for_each(|logic_gate| match logic_gate {
            LogicGate::And { output, .. }
            | LogicGate::Or { output, .. }
            | LogicGate::Xor { output, .. } => {
                if replacements.contains_key(output) {
                    *output = replacements[output];
                }
            }
        });

    while !values
        .iter()
        .all(|(wire, value)| !wire.starts_with("z") || value.is_some())
    {
        for logic_gate in &logic_gates {
            logic_gate.evaluate(&mut values);
        }
    }

    let x = values_to_number("x", &values)?;
    let y = values_to_number("y", &values)?;
    let z = values_to_number("z", &values)?;
    if x + y != z {
        return Err(anyhow!("Cannot implement adder: {} + {} != {}", x, y, z));
    }

    let mut replacements = replacements.into_keys().collect::<Vec<_>>();
    replacements.sort();

    Ok(replacements.join(","))
}

#[derive(Clone, Copy)]
enum LogicGate<'a> {
    And {
        input_1: &'a str,
        input_2: &'a str,
        output: &'a str,
    },
    Or {
        input_1: &'a str,
        input_2: &'a str,
        output: &'a str,
    },
    Xor {
        input_1: &'a str,
        input_2: &'a str,
        output: &'a str,
    },
}

impl<'a> LogicGate<'a> {
    fn from_str(s: &'a str) -> Result<Self> {
        let tokens = s.split_whitespace().collect::<Vec<_>>();
        if tokens.len() != 5 {
            return Err(anyhow!("Invalid input: {}", s));
        }

        match tokens[1] {
            "AND" => Ok(Self::And {
                input_1: tokens[0],
                input_2: tokens[2],
                output: tokens[4],
            }),
            "OR" => Ok(Self::Or {
                input_1: tokens[0],
                input_2: tokens[2],
                output: tokens[4],
            }),
            "XOR" => Ok(Self::Xor {
                input_1: tokens[0],
                input_2: tokens[2],
                output: tokens[4],
            }),
            _ => Err(anyhow!("Invalid op: {}", tokens[1])),
        }
    }

    fn evaluate(&self, values: &mut HashMap<&'a str, Option<bool>>) {
        match self {
            LogicGate::And {
                input_1,
                input_2,
                output,
            } => {
                if let (Some(input_1), Some(input_2), None) =
                    (values[input_1], values[input_2], values[output])
                {
                    values
                        .entry(output)
                        .and_modify(|value| *value = Some(input_1 && input_2));
                }
            }
            LogicGate::Or {
                input_1,
                input_2,
                output,
            } => {
                if let (Some(input_1), Some(input_2), None) =
                    (values[input_1], values[input_2], values[output])
                {
                    values
                        .entry(output)
                        .and_modify(|value| *value = Some(input_1 || input_2));
                }
            }
            LogicGate::Xor {
                input_1,
                input_2,
                output,
            } => {
                if let (Some(input_1), Some(input_2), None) =
                    (values[input_1], values[input_2], values[output])
                {
                    values
                        .entry(output)
                        .and_modify(|value| *value = Some(input_1 ^ input_2));
                }
            }
        }
    }
}

fn parse_input_into_values_and_logic_gates(
    input: &str,
) -> Result<(HashMap<&str, bool>, Vec<LogicGate>)> {
    let Some((values, logic_gates)) = input.split_once("\n\n") else {
        return Err(anyhow!("Cannot split into values and logic gates"));
    };

    let values = values
        .lines()
        .map(|line| {
            let Some((wire, value)) = line.split_once(": ") else {
                return Err(anyhow!("Cannot split input into wire and value: {}", line));
            };

            let value = match value {
                "0" => false,
                "1" => true,
                _ => return Err(anyhow!("Invalid value: {}", value)),
            };

            Ok((wire, value))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let logic_gates = logic_gates
        .lines()
        .map(LogicGate::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok((values, logic_gates))
}

/// Extends values by creating entries for all wires, and wrapping with [Option].
fn extend_values<'a>(
    values: HashMap<&'a str, bool>,
    logic_gates: &[LogicGate<'a>],
) -> HashMap<&'a str, Option<bool>> {
    let mut values = values
        .into_iter()
        .map(|(wire, value)| (wire, Some(value)))
        .collect::<HashMap<_, _>>();

    for logic_gate in logic_gates {
        match *logic_gate {
            LogicGate::And {
                input_1,
                input_2,
                output,
            }
            | LogicGate::Or {
                input_1,
                input_2,
                output,
            }
            | LogicGate::Xor {
                input_1,
                input_2,
                output,
            } => {
                values.entry(input_1).or_default();
                values.entry(input_2).or_default();
                values.entry(output).or_default();
            }
        }
    }

    values
}

/// Combines the bits represented by wires with wire_prefix into a [u64].
fn values_to_number(wire_prefix: &str, values: &HashMap<&str, Option<bool>>) -> Result<u64> {
    let mut bits = values
        .iter()
        .filter(|(wire, _)| wire.starts_with(wire_prefix))
        .map(|(wire, value)| match value {
            Some(true) => Ok((wire, 1)),
            Some(false) => Ok((wire, 0)),
            _ => Err(anyhow!("Attempting to use unresolved value: {}", wire)),
        })
        .collect::<Result<Vec<_>>>()?;
    bits.sort_by_key(|z| z.0);
    bits.reverse();

    Ok(bits
        .into_iter()
        .fold(0, |acc, (_, bit)| (acc << 1) + bit as u64))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1a() -> Result<()> {
        let example = r"
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

        assert_eq!(part_1(trim_newlines(example))?, 4);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let example = r"
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

        assert_eq!(part_1(trim_newlines(example))?, 2024);

        Ok(())
    }
}
