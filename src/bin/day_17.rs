use std::{collections::VecDeque, str::FromStr};

use anyhow::{anyhow, Result};
use regex::Regex;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-17.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<String> {
    let mut computer = Computer::from_str(&input)?;

    run(&mut computer)
}

fn _part_2_example(input: String) -> Result<u64> {
    let computer = Computer::from_str(&input)?;

    let mut a = 0;

    loop {
        let mut computer = computer.clone();

        computer.a = a;
        _run_with_termination_check(&mut computer)?;

        if computer.program == computer.output {
            return Ok(a);
        }

        a += 1;
    }
}

fn part_2(input: String) -> Result<u64> {
    // Brute-force solution does not work.
    //
    // Program is 2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0.
    //
    // [1] 2,4 => B = A % 8 i.e. B is 0..8
    // [2] 1,3 => B = B ^ 3 i.e. B is 3, 2, 1, 0, 7, 6, 5, 4
    // [3] 7,5 => C = A >> B
    // [4] 0,3 => A = A >> 3
    // [5] 1,5 => B = B ^ 5 i.e. B is 6, 7, 4, 5, 2, 3, 0, 1
    // [6] 4,4 => B = B ^ C
    // [7] 5,5 => prints B % 8
    // [8] 3,0 => jumps back to start if A != 0 (the termination condition).
    //
    // Note that [1] and [3] sets the values of B and C i.e. there is no retention from the previous
    // cycle other than the value for A.
    //
    // A gets shifted by 3 every cycle.
    //
    // To terminate with last print == 0, A must be 0..8 at start of the cycle. In fact we can plug
    // in the values and find that A can only be 6.

    // Create a queue to check all number iteractively rather than recursively.
    let mut output_queue: VecDeque<u64> = VecDeque::new();
    output_queue.push_back(6);

    Computer::from_str(&input)?
        .program
        .into_iter()
        .rev()
        .skip(1) // we seeded 6 into the queue
        .for_each(|program_value| {
            // Clone the input, and then clear output_queue for this iteraction.
            let mut input_queue = output_queue.clone();
            output_queue = VecDeque::default();

            while let Some(input) = input_queue.pop_front() {
                // Check all 8 possibilities.
                for i in 0..8u64 {
                    let possible_a = (input << 3) + i;

                    let a = possible_a;
                    let mut b = a % 8;
                    b ^= 3;
                    let c = a >> b;

                    // No need to evaluate [4], incorporated into to_check.
                    // a >>= 3;

                    b ^= 5;
                    b ^= c;

                    if b % 8 == program_value {
                        output_queue.push_back(possible_a);
                    }
                }
            }
        });

    output_queue
        .front() // smallest number is pushed first
        .copied()
        .ok_or(anyhow!("Cannot find a solution"))
}

#[derive(Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    program: Vec<u64>,
    output: Vec<u64>,
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(captures) = Regex::new(r"(?s)Register A: (?<a>\d+).*Register B: (?<b>\d+).*Register C: (?<c>\d+).*Program: (?<program>\d(,\d)*)")?.captures(s) else {
            return Err(anyhow!("Cannot parse regex: {}", s));
        };

        let a = captures["a"].parse()?;
        let b = captures["b"].parse()?;
        let c = captures["c"].parse()?;
        let program = captures["program"]
            .split_terminator(",")
            .map(str::parse::<u64>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::default(),
        })
    }
}

fn run(computer: &mut Computer) -> Result<String> {
    while computer.program.get(computer.ip).is_some() {
        step(computer)?;
    }

    Ok(computer
        .output
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(","))
}

fn step(computer: &mut Computer) -> Result<bool> {
    let Some(opcode) = computer.program.get(computer.ip) else {
        return Err(anyhow!("Cannot retrieve opcode: ip out of bound"));
    };
    let Some(operand) = computer.program.get(computer.ip + 1) else {
        return Err(anyhow!("Cannot retrieve operand: ip out of bound"));
    };

    match opcode {
        0 => adv(computer, *operand),
        1 => bxl(computer, *operand),
        2 => bst(computer, *operand),
        3 => jnz(computer, *operand),
        4 => bxc(computer, *operand),
        5 => out(computer, *operand),
        6 => bdv(computer, *operand),
        7 => cdv(computer, *operand),
        x => Err(anyhow!("Invalid opcode: {x}")),
    }
}

fn adv(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.a /= 2u64.pow(operand as u32);

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn bxl(computer: &mut Computer, operand: u64) -> Result<bool> {
    computer.b ^= operand;

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn bst(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.b = operand % 8;

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn jnz(computer: &mut Computer, operand: u64) -> Result<bool> {
    if computer.a > 0 {
        computer.ip = operand as usize;
    } else {
        computer.ip += 2;
    };

    // Do not terminate early.
    Ok(false)
}

fn bxc(computer: &mut Computer, _operand: u64) -> Result<bool> {
    computer.b ^= computer.c;

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn out(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.output.push(operand % 8);

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn bdv(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.b = computer.a / 2u64.pow(operand as u32);

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn cdv(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.c = computer.a / 2u64.pow(operand as u32);

    computer.ip += 2;

    // Do not terminate early.
    Ok(false)
}

fn combo_operand(operand: u64, computer: &Computer) -> Result<u64> {
    match operand {
        o if (0..=3).contains(&o) => Ok(o),
        4 => Ok(computer.a),
        5 => Ok(computer.b),
        6 => Ok(computer.c),
        x => Err(anyhow!("Invalid operand: {x}")),
    }
}

/// Compares [output] with [program].
///
/// Return true if [output] is prefix of [program].
fn _output_follows_program(output: &[u64], program: &[u64]) -> bool {
    if output.is_empty() {
        return true;
    }
    if output.len() > program.len() {
        return false;
    }

    output == &program[0..output.len()]
}

/// Runs the program with check for early termination.
///
/// Stops running once output does not match program.
fn _run_with_termination_check(computer: &mut Computer) -> Result<()> {
    while computer.program.get(computer.ip).is_some() {
        if let Ok(true) = _step_with_termination_check(computer) {
            break;
        }
    }

    Ok(())
}

/// Executes an operation with check for early termination.
///
/// Returns true (i.e. should terminate) if output does not match program.
fn _step_with_termination_check(computer: &mut Computer) -> Result<bool> {
    let Some(opcode) = computer.program.get(computer.ip) else {
        return Err(anyhow!("Cannot retrieve opcode: ip out of bound"));
    };
    let Some(operand) = computer.program.get(computer.ip + 1) else {
        return Err(anyhow!("Cannot retrieve operand: ip out of bound"));
    };

    match opcode {
        0 => adv(computer, *operand),
        1 => bxl(computer, *operand),
        2 => bst(computer, *operand),
        3 => jnz(computer, *operand),
        4 => bxc(computer, *operand),
        5 => _out_with_termination_check(computer, *operand),
        6 => bdv(computer, *operand),
        7 => cdv(computer, *operand),
        x => Err(anyhow!("Invalid opcode: {x}")),
    }
}

/// out operation with check for early termination.
///
/// Returns true (i.e. should terminate) if output does not match program.
fn _out_with_termination_check(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.output.push(operand % 8);

    computer.ip += 2;

    Ok(!_output_follows_program(
        &computer.output,
        &computer.program,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1_INPUT: &str = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2_INPUT: &str = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(
            part_1(EXAMPLE_1_INPUT.trim().to_string())?,
            "4,6,3,5,6,3,5,2,1,0".to_string()
        );

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(_part_2_example(EXAMPLE_2_INPUT.trim().to_string())?, 117440);

        Ok(())
    }
}
