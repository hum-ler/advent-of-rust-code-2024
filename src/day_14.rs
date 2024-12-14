use std::str::FromStr;

use anyhow::{anyhow, Ok, Result};
use regex::Regex;

use crate::{file_to_lines, string_to_lines};

const EXAMPLE_INPUT: &str = r"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

const INPUT_FILE: &str = "inputs/day-14.txt";

const EXAMPLE_GRID_SIZE: Vector2 = Vector2 { x: 11, y: 7 };

const INPUT_GRID_SIZE: Vector2 = Vector2 { x: 101, y: 103 };

const TIME_ELAPSED: i32 = 100;

const _VERTICAL_LINE_LENGTH: i32 = 20;

pub fn run_example_1() -> Result<usize> {
    part_1(&string_to_lines(EXAMPLE_INPUT), &EXAMPLE_GRID_SIZE)
}

pub fn run_part_1() -> Result<usize> {
    part_1(&file_to_lines(INPUT_FILE)?, &INPUT_GRID_SIZE)
}

pub fn run_part_2() -> Result<i32> {
    part_2(&file_to_lines(INPUT_FILE)?, &INPUT_GRID_SIZE)
}

fn part_1(lines: &[String], grid_size: &Vector2) -> Result<usize> {
    let robots = parse_lines_to_robots(lines)?;

    let final_positions = robots
        .iter()
        .map(|r| r.traverse(TIME_ELAPSED, grid_size))
        .collect::<Vec<_>>();

    let populations = find_quadrant_populations(&final_positions, grid_size);

    Ok(populations.0 * populations.1 * populations.2 * populations.3)
}

fn part_2(lines: &[String], grid_size: &Vector2) -> Result<i32> {
    // This is a stupid puzzle. What is the shape of the tree? Where is it
    // positioned? Is it solid or hollow? How big is it? Must each and every
    // robot form part of tree?
    //
    // Based on hints from the subreddit, the damn tree is a solid arrangement.
    // We can search for a vertical or a diagonal linear arrangement of robots.
    //
    // Alternatively, assuming the tree falls entirely into 1 quadrant, we can
    // use part 1 to check if one of the quadrants has way more robots that the
    // other quadrants.

    let robots = parse_lines_to_robots(lines)?;
    let half_of_robots = robots.len() / 2;

    let mut time = 0i32;

    loop {
        time += 1;

        let positions = robots
            .iter()
            .map(|r| r.traverse(time, grid_size))
            .collect::<Vec<_>>();

        let populations = find_quadrant_populations(&positions, grid_size);
        if populations.0 > half_of_robots
            || populations.1 > half_of_robots
            || populations.2 > half_of_robots
            || populations.3 > half_of_robots
        {
            print_grid(&positions, grid_size);
            break;
        }

        // if _find_vertical_line(&positions, _VERTICAL_LINE_LENGTH) {
        //     print_grid(&positions, grid_size);
        //     break;
        // }
    }

    Ok(time)
}

#[derive(PartialEq)]
struct Vector2 {
    x: i32,
    y: i32,
}

struct Robot {
    p: Vector2,
    v: Vector2,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(captures) =
            Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)")?.captures(s)
        else {
            return Err(anyhow!("Cannot parse regex: {}", s));
        };

        let px = captures["px"].parse()?;
        let py = captures["py"].parse()?;
        let vx = captures["vx"].parse()?;
        let vy = captures["vy"].parse()?;

        Ok(Self {
            p: Vector2 { x: px, y: py },
            v: Vector2 { x: vx, y: vy },
        })
    }
}

impl Robot {
    /// Moves the robot [duration] times.
    pub fn traverse(&self, duration: i32, grid_size: &Vector2) -> Vector2 {
        let mut x = (self.p.x + duration * self.v.x) % grid_size.x;
        if x.is_negative() {
            x += grid_size.x;
        }

        let mut y = (self.p.y + duration * self.v.y) % grid_size.y;
        if y.is_negative() {
            y += grid_size.y;
        }

        Vector2 { x, y }
    }
}

fn parse_lines_to_robots(lines: &[String]) -> Result<Vec<Robot>> {
    lines
        .iter()
        .map(String::as_str)
        .map(Robot::from_str)
        .collect()
}

fn find_quadrant_populations(
    positions: &[Vector2],
    grid_size: &Vector2,
) -> (usize, usize, usize, usize) {
    assert!(grid_size.x > 0);
    assert_eq!(grid_size.x % 2, 1);
    assert!(grid_size.y > 0);
    assert_eq!(grid_size.y % 2, 1);

    // "Median" as in divider, not the statistic.
    let median_x = (grid_size.x - 1) / 2;
    let median_y = (grid_size.y - 1) / 2;

    positions.iter().fold((0, 0, 0, 0), |acc, pos| {
        if pos.x < median_x && pos.y < median_y {
            (acc.0 + 1, acc.1, acc.2, acc.3)
        } else if pos.x > median_x && pos.y < median_y {
            (acc.0, acc.1 + 1, acc.2, acc.3)
        } else if pos.x < median_x && pos.y > median_y {
            (acc.0, acc.1, acc.2 + 1, acc.3)
        } else if pos.x > median_x && pos.y > median_y {
            (acc.0, acc.1, acc.2, acc.3 + 1)
        } else {
            acc
        }
    })
}

fn print_grid(positions: &[Vector2], grid_size: &Vector2) {
    for y in 0..grid_size.y {
        for x in 0..grid_size.x {
            if positions.contains(&Vector2 { x, y }) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

/// Finds a vertical line of at least [length] formed by items in [positions].
fn _find_vertical_line(positions: &[Vector2], length: i32) -> bool {
    'outer: for pos in positions {
        for i in 0..length {
            if !positions.contains(&Vector2 {
                x: pos.x + i,
                y: pos.y,
            }) {
                // Move on to the next pos because this one failed.
                continue 'outer;
            }
        }

        // Successfully looped through length iterations.
        return true;
    }

    // Exhausted all pos.
    false
}
