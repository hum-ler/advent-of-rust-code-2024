use std::{fs::read_to_string, str::FromStr};

use anyhow::{anyhow, Result};
use regex::Regex;

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

const INPUT_FILE: &str = "inputs/day-17.txt";

pub fn run_example_1() -> Result<String> {
    part_1(EXAMPLE_1_INPUT.trim())
}

pub fn run_part_1() -> Result<String> {
    part_1(read_to_string(INPUT_FILE)?.trim())
}

pub fn run_example_2() -> Result<u64> {
    part_2(EXAMPLE_2_INPUT.trim())
}

pub fn _run_part_2() -> Result<u64> {
    part_2(read_to_string(INPUT_FILE)?.trim())
}

fn part_1(input: &str) -> Result<String> {
    let mut computer = Computer::from_str(input)?;

    run(&mut computer)
}

fn part_2(input: &str) -> Result<u64> {
    // Brute-force solution does not work.

    let computer = Computer::from_str(input)?;

    let mut a = 0;

    loop {
        let mut computer = computer.clone();

        computer.a = a;
        run_with_termination_check(&mut computer)?;

        if computer.program == computer.output {
            return Ok(a);
        }

        a += 1;
    }
}

#[derive(Debug, Clone)]
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
            .split(",")
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
fn output_follows_program(output: &[u64], program: &[u64]) -> bool {
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
fn run_with_termination_check(computer: &mut Computer) -> Result<()> {
    while computer.program.get(computer.ip).is_some() {
        if let Ok(true) = step_with_termination_check(computer) {
            break;
        }
    }

    Ok(())
}

/// Executes an operation with check for early termination.
///
/// Returns true (i.e. should terminate) if output does not match program.
fn step_with_termination_check(computer: &mut Computer) -> Result<bool> {
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
        5 => out_with_termination_check(computer, *operand),
        6 => bdv(computer, *operand),
        7 => cdv(computer, *operand),
        x => Err(anyhow!("Invalid opcode: {x}")),
    }
}

/// out operation with check for early termination.
///
/// Returns true (i.e. should terminate) if output does not match program.
fn out_with_termination_check(computer: &mut Computer, operand: u64) -> Result<bool> {
    let operand = combo_operand(operand, computer)?;

    computer.output.push(operand % 8);

    computer.ip += 2;

    Ok(!output_follows_program(&computer.output, &computer.program))
}
