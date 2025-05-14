use std::{collections::HashSet, str::FromStr};

use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-14.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u32> {
    safety_factor(input, (101, 103))
}

fn part_2(input: &str) -> Result<u32> {
    let robots = input
        .lines()
        .map(Robot::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mut time = 1;
    loop {
        let pattern = robots
            .iter()
            .map(|robot| robot.pos(time, (101, 103)))
            .collect::<HashSet<_>>();

        if contains_corner(&pattern) {
            break;
        }

        time += 1;
    }

    Ok(time)
}

type GridSize = (usize, usize);

fn safety_factor(robots: &str, grid_size: GridSize) -> Result<u32> {
    let robots = robots
        .lines()
        .map(Robot::from_str)
        .collect::<Result<Vec<_>>>()?;

    let mid_x = grid_size.0 / 2;
    let mid_y = grid_size.1 / 2;

    let low_x = 0..mid_x;
    let high_x = mid_x + 1..grid_size.0;
    let low_y = 0..mid_y;
    let high_y = mid_y + 1..grid_size.1;

    Ok(robots
        .into_iter()
        .map(|robot| robot.pos(100, grid_size))
        .fold([0; 4], |acc, pos| {
            // acc: [top_left, top_right, bottom_left, bottom_right]

            match (pos.0 as usize, pos.1 as usize) {
                (x, y) if low_x.contains(&x) && low_y.contains(&y) => {
                    [acc[0] + 1, acc[1], acc[2], acc[3]]
                }
                (x, y) if high_x.contains(&x) && low_y.contains(&y) => {
                    [acc[0], acc[1] + 1, acc[2], acc[3]]
                }
                (x, y) if low_x.contains(&x) && high_y.contains(&y) => {
                    [acc[0], acc[1], acc[2] + 1, acc[3]]
                }
                (x, y) if high_x.contains(&x) && high_y.contains(&y) => {
                    [acc[0], acc[1], acc[2], acc[3] + 1]
                }
                _ => acc,
            }
        })
        .into_iter()
        .product())
}

type Vector2 = (i32, i32);

struct Robot {
    initial_pos: Vector2,
    velocity: Vector2,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some((pos, velocity)) = s.strip_prefix("p=").and_then(|s| s.split_once(" v=")) else {
            return Err(anyhow!("Cannot split input into pos and velocity: {}", s));
        };

        let Some((x, y)) = pos.split_once(",") else {
            return Err(anyhow!("Cannot split input into x and y: {}", pos));
        };
        let pos = (x.parse()?, y.parse()?);

        let Some((x, y)) = velocity.split_once(",") else {
            return Err(anyhow!("Cannot split input into x and y: {}", velocity));
        };
        let velocity = (x.parse()?, y.parse()?);

        Ok(Self {
            initial_pos: pos,
            velocity,
        })
    }
}

impl Robot {
    fn pos(&self, time: u32, grid_size: GridSize) -> Vector2 {
        let mut x = (self.initial_pos.0 + time as i32 * self.velocity.0) % grid_size.0 as i32;
        if x < 0 {
            x += grid_size.0 as i32;
        }
        let mut y = (self.initial_pos.1 + time as i32 * self.velocity.1) % grid_size.1 as i32;
        if y < 0 {
            y += grid_size.1 as i32;
        }

        (x, y)
    }
}

/// Finds a top-left corner formed by 5-unit-long straight lines.
fn contains_corner(pattern: &HashSet<Vector2>) -> bool {
    pattern.iter().any(|&(x, y)| {
        [
            (x + 1, y),
            (x + 2, y),
            (x + 3, y),
            (x + 4, y),
            (x, y + 1),
            (x, y + 2),
            (x, y + 3),
            (x, y + 4),
        ]
        .into_iter()
        .all(|pos| pattern.contains(&pos))
    })
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    #[test]
    fn example_1() -> Result<()> {
        let example = r"
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

        assert_eq!(safety_factor(trim_newlines(example), (11, 7))?, 12);

        Ok(())
    }
}
