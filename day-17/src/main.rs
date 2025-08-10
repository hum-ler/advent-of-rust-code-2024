use std::{collections::VecDeque, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-17.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<String> {
    let mut computer = Computer::from_str(input)?;

    computer.run()?;

    Ok(computer
        .output
        .into_iter()
        .fold(String::new(), |acc, value| {
            if acc.is_empty() {
                value.to_string()
            } else {
                format!("{},{}", acc, value)
            }
        }))
}

fn part_2(input: &str) -> Result<u64> {
    step_through_program_values(input)
}

#[cfg(test)]
fn match_program(input: &str) -> Result<u32> {
    let computer = Computer::from_str(input)?;

    let mut a = 0;
    loop {
        let mut computer = computer.clone();
        computer.a = a;

        match computer.run_with_program_output_matching() {
            Ok(()) => {
                if computer.output == computer.program {
                    break;
                }
            }
            Err(error) => {
                if error.to_string() != "Program-output mismatch" {
                    return Err(error);
                }
            }
        }

        a += 1;
    }

    Ok(a)
}

fn step_through_program_values(input: &str) -> Result<u64> {
    // Brute-force won't work.
    //
    // Program is 2,4,1,3,7,5,0,3,1,5,4,4,5,5,3,0. (Sorry!) Let's break up the program into ops
    // (opcode + operand):
    //   (1) 2,4 => B = A % 8 i.e. B is 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7
    //   (2) 1,3 => B = B ^ 3 i.e. B is 3 | 2 | 1 | 0 | 7 | 6 | 5 | 4
    //   (3) 7,5 => C = A / 2^B (or A >> B)
    //   (4) 0,3 => A = A / 2^3 (or A >> 3)
    //   (5) 1,5 => B = B ^ 5 i.e. B is 6 | 7 | 4 | 5 | 2 | 3 | 0 | 1
    //   (6) 4,4 => B = B ^ C
    //   (7) 5,5 => output B % 8 (or B & 7)
    //   (8) 3,0 => jump back to (1) until A == 0
    //
    // Note that (1) and (3) sets the values of B and C i.e. there is no retention from the prev
    // cycle other than the value inside A.
    //
    // A gets right-shifted by 3 in (4) every cycle.
    //
    // To terminate with A == 0, A must be 0..8 at the beginning of the cycle. If we plug in values
    // manually and check it out, we find that, in fact, A can only be 6 at the start of the cycle
    // i.e. for the shortest program of simply "0", to get the output "0", A must be 6. We can build
    // on this to work out the possible A's by recursion or iteration.

    let mut step_output = VecDeque::new();
    step_output.push_back(6);

    Computer::from_str(input)?
        .program
        .into_iter()
        .rev()
        .skip(1) // skip the first step that we found manually
        .for_each(|target| {
            // At each step, we handle one value (the target) from the tail of program, and keep the
            // numbers that can survive the evaluation and result in the target value.

            let mut prev_step_output = step_output.clone();
            step_output.clear();

            while let Some(from_prev_step) = prev_step_output.pop_front() {
                for i in 0..8 {
                    let possible_number = (from_prev_step << 3) + i; // inverse of (4)

                    let a = possible_number;
                    let mut b = a % 8;
                    b ^= 3;
                    let c = a >> b;
                    // a >>= 3; // no point
                    b ^= 5;
                    b ^= c;

                    if b & 7 == target as u64 {
                        step_output.push_back(possible_number);
                    }
                }
            }
        });

    step_output
        .front() // smallest number is at the front because of order of evaluation
        .copied()
        .ok_or(anyhow!("Cannot find solution for register A"))
}

#[derive(Clone)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        if lines.len() != 5 {
            return Err(anyhow!("Invalid input: {}", s));
        }

        let a = lines[0]
            .strip_prefix("Register A: ")
            .ok_or(anyhow!("Invalid register A: {}", lines[0]))?
            .parse()?;

        let b = lines[1]
            .strip_prefix("Register B: ")
            .ok_or(anyhow!("Invalid register B: {}", lines[1]))?
            .parse()?;

        let c = lines[2]
            .strip_prefix("Register C: ")
            .ok_or(anyhow!("Invalid register C: {}", lines[2]))?
            .parse()?;

        let program = lines[4]
            .strip_prefix("Program: ")
            .ok_or(anyhow!("Invalid program: {}", lines[4]))?
            .split_terminator(",")
            .map(str::parse::<u8>)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::new(),
        })
    }
}

impl Computer {
    fn run(&mut self) -> Result<()> {
        while self.ip < self.program.len() {
            self.step(false)?
        }

        Ok(())
    }

    fn step(&mut self, check_program_output: bool) -> Result<()> {
        if self.ip == self.program.len() - 1 {
            return Err(anyhow!("Cannot get operand"));
        }
        let operand = self.program[self.ip + 1];

        match self.program[self.ip] {
            0 => self.a /= 2u32.pow(self.combo_operand_value(operand)?),
            1 => self.b ^= operand as u32,
            2 => self.b = self.combo_operand_value(operand)? % 8,
            3 => {
                if self.a != 0 {
                    self.ip = operand as usize;

                    return Ok(());
                }
            }
            4 => self.b ^= self.c,
            5 => {
                self.output
                    .push((self.combo_operand_value(operand)? % 8) as u8);

                if check_program_output
                    && (self.output.len() > self.program.len()
                        || !self
                            .output
                            .iter()
                            .enumerate()
                            .all(|(index, value)| self.program[index] == *value))
                {
                    return Err(anyhow!("Program-output mismatch"));
                }
            }
            6 => self.b = self.a / 2u32.pow(self.combo_operand_value(operand)?),
            7 => self.c = self.a / 2u32.pow(self.combo_operand_value(operand)?),
            _ => return Err(anyhow!("Invalid opcode: {}", self.program[self.ip])),
        }

        self.ip += 2;

        Ok(())
    }

    fn combo_operand_value(&self, combo_operand: u8) -> Result<u32> {
        match combo_operand {
            0..=3 => Ok(combo_operand as u32),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(anyhow!("Invalid combo operand: {}", combo_operand)),
        }
    }

    #[cfg(test)]
    fn run_with_program_output_matching(&mut self) -> Result<()> {
        while self.ip < self.program.len() {
            self.step(true)?
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

        assert_eq!(part_1(trim_newlines(example))?, "4,6,3,5,6,3,5,2,1,0");

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        let example = r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

        assert_eq!(match_program(trim_newlines(example))?, 117440);

        Ok(())
    }
}
