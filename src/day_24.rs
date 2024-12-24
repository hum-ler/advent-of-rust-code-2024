use std::{collections::HashMap, fs::read_to_string};

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

pub fn _run_part_2() -> Result<String> {
    _part_2(read_to_string(INPUT_FILE)?.trim())
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

fn _part_2(_input: &str) -> Result<String> {
    // We should be looking at a 45-bit ripple-carry adder.
    //
    // At the very least, we should expect to find the following operation:
    // - for the lsb:
    //   - x00 ^ y00 -> z00
    //   - x00 & y00 -> c_out00
    // - for higher bits:
    //   - x<n> ^ y<n> -> temp_1<n>
    //   - x<n> & y<n> -> temp_2<n>
    //   - temp_2<n> & c_out<n - 1> -> temp_3<n>
    //   - temp_1<n> ^ c_out<n - 1> -> z<n>
    //   - temp_2<n> | temp_3<n> -> c_out<n>
    //
    // For 45 bits we should expect 2 + 44 * 5 = 222 operations, which matches the input.

    todo!()
}

struct Operation<'a> {
    input_1: &'a str,
    input_2: &'a str,
    operator: &'a str,
}

impl PartialEq for Operation<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator
            && ((self.input_1 == other.input_1 && self.input_2 == other.input_2)
                || (self.input_1 == other.input_2 && self.input_2 == other.input_1))
    }
}

impl<'a> Operation<'a> {
    pub fn new(input_1: &'a str, input_2: &'a str, operator: &'a str) -> Self {
        Self {
            input_1,
            input_2,
            operator,
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
        let [input_1, operator, input_2, arrow, signal] = line.split(" ").collect::<Vec<_>>()[..]
        else {
            return Err(anyhow!("Cannot split unresolved line: {}", line));
        };

        assert_eq!(arrow, "->");

        unresolved_signals.insert(signal, Operation::new(input_1, input_2, operator));

        Ok(())
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
