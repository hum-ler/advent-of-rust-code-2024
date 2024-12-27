use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs::read_to_string,
};

use anyhow::{anyhow, Result};

const EXAMPLE_INPUT: &str = r"
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

const INPUT_FILE: &str = "inputs/day-24.txt";

pub fn run_example_1() -> Result<u64> {
    part_1(EXAMPLE_INPUT.trim())
}

pub fn run_part_1() -> Result<u64> {
    part_1(read_to_string(INPUT_FILE)?.trim())
}

pub fn run_part_2() -> Result<String> {
    part_2(read_to_string(INPUT_FILE)?.trim())
}

fn part_1(input: &str) -> Result<u64> {
    let (mut resolved_signals, mut unresolved_signals) = parse_input_to_signals(input)?;
    let z_signals = find_signals_starting_with_letter("z", &unresolved_signals);

    let mut number = bits_to_number(&z_signals, &resolved_signals)?;
    while number.is_none() {
        resolve(&mut unresolved_signals, &mut resolved_signals);

        number = bits_to_number(&z_signals, &resolved_signals)?;
    }

    if let Some(number) = number {
        Ok(number)
    } else {
        Err(anyhow!("Cannot unwrap combined z number"))
    }
}

fn part_2(input: &str) -> Result<String> {
    // We should be looking at a 45-bit ripple-carry adder.
    //
    // At the very least, we should expect to find the following operation:
    // - for the lsb:
    //   - x00 ^ y00 -> z00
    //   - x00 & y00 -> c01
    // - for higher bits:
    //   - x<n> ^ y<n> -> i<n>
    //   - x<n> & y<n> -> j<n>
    //   - i<n> & c<n> -> k<n>
    //   - i<n> ^ c<n> -> z<n>
    //   - j<n> | k<n> -> c<n + 1>
    // - for the msb, the last statement becomes:
    //   - j44 | k44 -> z45
    //
    // For 45 bits we should expect 2 + 44 * 5 = 222 operations, which matches the input.

    let (mut resolved_signals, mut unresolved_signals) = parse_input_to_signals(input)?;

    // let mut _signals: HashSet<Operation> =
    //     HashSet::from_iter(unresolved_signals.clone().into_values());
    // let _substitutions = _remove_correct_signals(&mut _signals);

    // From here, there are 3 "-> z"s that are not XORs, hence must be incorrect:
    // - z15
    // - z05
    // - z20
    // There are also correspondingly 3 XORs that do not involve x and y:
    // - fvm XOR mvv
    // - bhw XOR sth
    // - gcs XOR hdc

    // Hence the swaps:
    // - z15 <-> htp
    // - z05 <-> dkr
    // - z20 <-> hhh

    // z45 is equivalent to c45, which clears the following:
    // - x44 AND y44 -> kbb
    // - gqg AND pfh -> khw
    // - kbb OR khw -> z45

    // Checking z36, we need to swap: rhv <-> ggk.

    // Combining everything: dkr,ggk,hhh,htp,rhv,z05,z15,z20.

    let swaps = [
        ("z15", "htp"),
        ("z05", "dkr"),
        ("z20", "hhh"),
        ("rhv", "ggk"),
    ];
    swaps.iter().for_each(|(first, second)| {
        let operation_1 = unresolved_signals[first].to_owned();
        let operation_2 = unresolved_signals[second].to_owned();

        unresolved_signals.insert(first, operation_2);
        unresolved_signals.insert(second, operation_1);
    });

    let x_signals = find_signals_starting_with_letter("x", &resolved_signals);
    let y_signals = find_signals_starting_with_letter("y", &resolved_signals);
    let z_signals = find_signals_starting_with_letter("z", &unresolved_signals);

    // Resolve all the signals.
    let mut z = bits_to_number(&z_signals, &resolved_signals)?;
    while z.is_none() {
        resolve(&mut unresolved_signals, &mut resolved_signals);

        z = bits_to_number(&z_signals, &resolved_signals)?;
    }

    let Some(x) = bits_to_number(&x_signals, &resolved_signals)? else {
        return Err(anyhow!("Cannot derive number x"));
    };
    let Some(y) = bits_to_number(&y_signals, &resolved_signals)? else {
        return Err(anyhow!("Cannot derive number y"));
    };
    let Some(z) = z else {
        return Err(anyhow!("Cannot unwrap number z"));
    };

    // Quick verification.
    if x + y != z {
        return Err(anyhow!("Cannot implement adder: {} + {} != {}", x, y, z));
    }

    let mut swaps = swaps
        .into_iter()
        .flat_map(|(first, second)| [first, second])
        .collect::<Vec<_>>();
    swaps.sort();

    Ok(swaps.join(","))
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Operation<'a> {
    input_1: &'a str,
    input_2: &'a str,
    operator: &'a str,
    output: &'a str,
}

impl<'a> TryFrom<&'a str> for Operation<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> std::result::Result<Self, Self::Error> {
        let [input_1, operator, input_2, arrow, signal] = value.split(" ").collect::<Vec<_>>()[..]
        else {
            return Err(anyhow!("Cannot split value: {}", value));
        };

        assert_eq!(arrow, "->");

        Ok(Self::new(input_1, input_2, operator, signal))
    }
}

impl Display for Operation<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.input_1 < self.input_2 {
            f.write_fmt(format_args!(
                "{} {} {} -> {}",
                self.input_1, self.operator, self.input_2, self.output,
            ))
        } else {
            f.write_fmt(format_args!(
                "{} {} {} -> {}",
                self.input_2, self.operator, self.input_1, self.output,
            ))
        }
    }
}

impl<'a> Operation<'a> {
    fn new(input_1: &'a str, input_2: &'a str, operator: &'a str, output: &'a str) -> Self {
        Self {
            input_1,
            input_2,
            operator,
            output,
        }
    }

    /// Matches this [Operation] against the given arguments.
    ///
    /// [operator] and [output] (if given) must match exactly.
    ///
    /// [operand_1] and [operand_2] will match either [operand_1] == [input_1] and
    /// [operand_2] == [input_2] as a pair, or the other way around ([operand_1] == [input_2] and
    /// [operand_2] == [input_1]).
    ///
    /// You may also just set one of [operand_1] or [operand_2] to None in order to match either
    /// [input_1] or [input_2].
    fn _matches(
        &self,
        operand_1: Option<&str>,
        operand_2: Option<&str>,
        operator: Option<&str>,
        output: Option<&str>,
    ) -> bool {
        // Handle operator and output first.
        let operator = match operator {
            Some(operator) => operator,
            None => self.operator,
        };
        let output = match output {
            Some(output) => output,
            None => self.output,
        };
        if self.operator != operator || self.output != output {
            return false;
        }

        // Handle operands. Assume operator and output matches.
        match (operand_1, operand_2) {
            (Some(operand_1), Some(operand_2)) => {
                (operand_1 == self.input_1 && operand_2 == self.input_2)
                    || (operand_1 == self.input_2 && operand_2 == self.input_1)
            }
            (Some(operand), None) | (None, Some(operand)) => {
                operand == self.input_1 || operand == self.input_2
            }
            (None, None) => true,
        }
    }
}

type ResolvedSignals<'a> = HashMap<&'a str, u64>;
type UnresolvedSignals<'a> = HashMap<&'a str, Operation<'a>>;

/// Parses [input] into resolved values ([ResolvedSignals]) and unresolved [Operation]s
/// ([UnresolvedSignals]).
fn parse_input_to_signals(input: &str) -> Result<(ResolvedSignals, UnresolvedSignals)> {
    let mut resolved_signals = HashMap::new();
    let mut unresolved_signals = HashMap::new();

    let [resolved_section, unresolved_section] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        return Err(anyhow!("Cannot split input into sections"));
    };

    resolved_section.split("\n").try_for_each(|line| {
        let [signal, value] = line.split(": ").collect::<Vec<_>>()[..] else {
            return Err(anyhow!("Cannot split resolved line: {}", line));
        };

        resolved_signals.insert(signal, value.parse::<u64>()?);

        Ok(())
    })?;

    unresolved_section.split("\n").try_for_each(|line| {
        let operation = Operation::try_from(line)?;

        unresolved_signals.insert(operation.output, operation);

        Ok::<_, anyhow::Error>(())
    })?;

    Ok((resolved_signals, unresolved_signals))
}

/// Assembles the bits (keyed by the elements of [signal_keys] into [resolved_signals]), and parses
/// the number represented by the bits.
///
/// [signal_keys] must be in the correct order (from most significant bit to least significant bit).
///
/// Returns `Ok(None)` if not all signals are resolved yet.
fn bits_to_number(
    signal_keys: &Vec<&str>,
    resolved_signals: &ResolvedSignals,
) -> Result<Option<u64>> {
    let bit_string = signal_keys
        .iter()
        .try_fold(String::default(), |acc, signal| {
            if !resolved_signals.contains_key(signal) {
                return Err(anyhow!("{} not resolved, short-circuiting...", signal));
            }

            Ok(acc + resolved_signals[signal].to_string().as_str())
        });

    if let Ok(bit_string) = bit_string {
        let value = u64::from_str_radix(bit_string.as_str(), 2);

        if let Ok(value) = value {
            return Ok(Some(value));
        } else {
            return Err(anyhow!(
                "Cannot parse number from bit_string: {} {:?}",
                bit_string,
                value.err(),
            ));
        }
    }

    Ok(None)
}

/// Attempt to resolve the values of signal inside [unresolved_signals].
///
/// If successfully resolved, the signal will be removed from [unresolved_signals] and added to
/// [resolved_signals].
fn resolve<'a>(
    unresolved_signals: &mut UnresolvedSignals<'a>,
    resolved_signals: &mut ResolvedSignals<'a>,
) {
    let unresolved_keys = unresolved_signals.keys().copied().collect::<Vec<_>>();

    unresolved_keys.iter().for_each(|key| {
        let operation = &unresolved_signals[key];

        if resolved_signals.contains_key(operation.input_1)
            && resolved_signals.contains_key(operation.input_2)
        {
            match operation.operator {
                "AND" => {
                    resolved_signals.insert(
                        key,
                        resolved_signals[operation.input_1] & resolved_signals[operation.input_2],
                    );
                }
                "OR" => {
                    resolved_signals.insert(
                        key,
                        resolved_signals[operation.input_1] | resolved_signals[operation.input_2],
                    );
                }
                "XOR" => {
                    resolved_signals.insert(
                        key,
                        resolved_signals[operation.input_1] ^ resolved_signals[operation.input_2],
                    );
                }
                _ => (),
            }

            unresolved_signals.remove(key);
        }
    });
}

/// Retrieves the list of keys in [hash_map] that start with [letter].
///
/// The returned list is sorted in the order of most significant bit to least significant bit.
fn find_signals_starting_with_letter<'a, T>(
    letter: &str,
    hash_map: &HashMap<&'a str, T>,
) -> Vec<&'a str> {
    assert_eq!(letter.len(), 1);

    let mut signals = hash_map
        .keys()
        .filter(|key| key.starts_with(letter))
        .copied()
        .collect::<Vec<_>>();

    signals.sort();
    signals.reverse();

    signals
}

/// Removes all the "expected" signals from [signals], leaving behind the rest that requires closer
/// scrutiny.
///
/// Returns the map of deciphered signals to encoded signals.
fn _remove_correct_signals(signals: &mut HashSet<Operation>) -> HashMap<String, String> {
    let mut substitutions: HashMap<String, String> = HashMap::new();

    // Handle the lsb first.
    for operation in signals.clone().iter() {
        if operation._matches(Some("x00"), Some("y00"), Some("XOR"), Some("z00")) {
            signals.remove(operation);
        } else if operation._matches(Some("x00"), Some("y00"), Some("AND"), None) {
            signals.remove(operation);
            substitutions.insert("c01".to_owned(), operation.output.to_owned());
        }
    }

    // Match x<n> ^ y<n> -> i<n>, and then i<n> ^ c<n> -> z<n>.
    for n in 1..45 {
        let x = format!("x{:02}", n);
        let y = format!("y{:02}", n);
        let z = format!("z{:02}", n);

        let signals_clone = signals.clone();

        // x<n> ^ y<n> -> i<n>
        let candidate_1 = signals_clone
            .iter()
            .find(|operation| operation._matches(Some(&x), Some(&y), Some("XOR"), None));
        let Some(candidate_1) = candidate_1 else {
            continue;
        };

        // i<n> ^ c<n> -> z<n>
        let candidate_2 = signals_clone.iter().find(|operation| {
            operation._matches(Some(candidate_1.output), None, Some("XOR"), Some(&z))
        });
        let Some(candidate_2) = candidate_2 else {
            continue;
        };

        let i = format!("i{:02}", n);
        let c = format!("c{:02}", n);
        let c_sub = if candidate_2.input_1 == candidate_1.output {
            candidate_2.input_2
        } else {
            candidate_2.input_1
        };

        substitutions.insert(i, candidate_1.output.to_owned());
        substitutions.insert(c, c_sub.to_owned());
        signals.remove(candidate_1);
        signals.remove(candidate_2);
    }

    // Match x<n> & y<n> -> j<n> and i<n> & c<n> -> k<n>, then verify j<n> | k<n> -> c<n + 1>.
    for n in 1..45 {
        let x = format!("x{:02}", n);
        let y = format!("y{:02}", n);
        let i = format!("i{:02}", n);
        let c = format!("c{:02}", n);
        let next_c = format!("c{:02}", n + 1);

        if !substitutions.contains_key(&i) {
            continue;
        }
        let i_sub = &substitutions[&i];
        if !substitutions.contains_key(&c) {
            continue;
        }
        let c_sub = &substitutions[&c];

        if !substitutions.contains_key(&next_c) {
            continue;
        }
        let next_c_sub = &substitutions[&next_c];

        let signals_clone = signals.clone();

        // x<n> & y<n> -> j<n>
        let candidate_1 = signals_clone
            .iter()
            .find(|operation| operation._matches(Some(&x), Some(&y), Some("AND"), None));
        let Some(candidate_1) = candidate_1 else {
            continue;
        };

        // i<n> & c<n> -> k<n>
        let candidate_2 = signals_clone
            .iter()
            .find(|operation| operation._matches(Some(i_sub), Some(c_sub), Some("AND"), None));
        let Some(candidate_2) = candidate_2 else {
            continue;
        };

        // j<n> | k<n> -> c<n + 1>
        let candidate_3 = signals_clone.iter().find(|operation| {
            operation._matches(
                Some(candidate_1.output),
                Some(candidate_2.output),
                Some("OR"),
                Some(next_c_sub),
            )
        });
        let Some(candidate_3) = candidate_3 else {
            continue;
        };

        let j = format!("j{:02}", n);
        let k = format!("k{:02}", n);

        substitutions.insert(j, candidate_1.output.to_owned());
        substitutions.insert(k, candidate_2.output.to_owned());
        signals.remove(candidate_1);
        signals.remove(candidate_2);
        signals.remove(candidate_3);
    }

    substitutions
}
