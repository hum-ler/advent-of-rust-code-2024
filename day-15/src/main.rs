use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-15.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let (mut grid, moves) = parse_input_into_grid_and_pushes(input)?;

    for push in moves {
        match push {
            b'^' => grid.push_up(),
            b'>' => grid.push_right(),
            b'v' => grid.push_down(),
            b'<' => grid.push_left(),
            _ => return Err(anyhow!("Invalid push: {}", push)),
        }
    }

    Ok(grid.box_gps_sum())
}

fn part_2(input: &str) -> Result<usize> {
    let (mut grid, moves) = parse_input_into_grid_and_pushes(input)?;
    grid = grid.into_wide_grid();

    for push in moves {
        match push {
            b'^' => grid.push_up(),
            b'>' => grid.push_right(),
            b'v' => grid.push_down(),
            b'<' => grid.push_left(),
            _ => return Err(anyhow!("Invalid push: {}", push)),
        }
    }

    Ok(grid.box_gps_sum())
}

/// (row, col)
type Coord = (usize, usize);

struct Grid {
    layout: Vec<Vec<u8>>,
    robot: Coord,
    is_wide: bool,
}

impl Grid {
    fn push_up(&mut self) {
        if self.is_wide {
            return self.push_up_for_wide();
        }

        let (mut row, col) = (self.robot.0 - 1, self.robot.1);
        loop {
            match self.layout[row][col] {
                b'#' => return,
                b'.' => {
                    for row in row..self.robot.0 {
                        (self.layout[row][col], self.layout[row + 1][col]) =
                            (self.layout[row + 1][col], self.layout[row][col]);
                    }
                    self.robot = (self.robot.0 - 1, col);

                    return;
                }
                _ => (),
            }

            row -= 1;
        }
    }

    fn push_right(&mut self) {
        let (row, mut col) = (self.robot.0, self.robot.1 + 1);
        loop {
            match self.layout[row][col] {
                b'#' => return,
                b'.' => {
                    for col in (self.robot.1..col).rev() {
                        (self.layout[row][col], self.layout[row][col + 1]) =
                            (self.layout[row][col + 1], self.layout[row][col]);
                    }
                    self.robot = (row, self.robot.1 + 1);

                    return;
                }
                _ => (),
            }

            col += 1;
        }
    }

    fn push_down(&mut self) {
        if self.is_wide {
            return self.push_down_for_wide_grid();
        }

        let (mut row, col) = (self.robot.0 + 1, self.robot.1);
        loop {
            match self.layout[row][col] {
                b'#' => return,
                b'.' => {
                    for row in (self.robot.0..row).rev() {
                        (self.layout[row][col], self.layout[row + 1][col]) =
                            (self.layout[row + 1][col], self.layout[row][col]);
                    }
                    self.robot = (self.robot.0 + 1, col);

                    return;
                }
                _ => (),
            }

            row += 1;
        }
    }

    fn push_left(&mut self) {
        let (row, mut col) = (self.robot.0, self.robot.1 - 1);
        loop {
            match self.layout[row][col] {
                b'#' => return,
                b'.' => {
                    for col in col..self.robot.1 {
                        (self.layout[row][col], self.layout[row][col + 1]) =
                            (self.layout[row][col + 1], self.layout[row][col]);
                    }
                    self.robot = (row, self.robot.1 - 1);

                    return;
                }
                _ => (),
            }

            col -= 1;
        }
    }

    fn box_gps_sum(&self) -> usize {
        self.layout
            .iter()
            .enumerate()
            .flat_map(|(row, bytes)| {
                bytes.iter().enumerate().filter_map(move |(col, byte)| {
                    match (self.is_wide, *byte) {
                        (false, b'O') | (true, b'[') => Some(row * 100 + col),
                        _ => None,
                    }
                })
            })
            .sum()
    }

    fn into_wide_grid(self) -> Self {
        if self.is_wide {
            return self;
        }

        let layout = self
            .layout
            .into_iter()
            .map(|bytes| {
                bytes
                    .into_iter()
                    .flat_map(|byte| match byte {
                        b'O' => [b'[', b']'],
                        b'@' => [b'@', b'.'],
                        _ => [byte, byte],
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let row = layout
            .iter()
            .position(|row| row.contains(&b'@'))
            .unwrap_or(0);
        let col = layout[row]
            .iter()
            .position(|byte| *byte == b'@')
            .unwrap_or(0);
        let robot = (row, col);

        Self {
            layout,
            robot,
            is_wide: true,
        }
    }

    fn push_up_for_wide(&mut self) {
        let (row, col) = self.robot;

        if self.can_push_up_from_coord((row, col)) {
            self.push_up_from_coord((row, col));
            self.robot = (row - 1, col);
        }
    }

    fn can_push_up_from_coord(&self, coord: Coord) -> bool {
        let (row, col) = (coord.0, coord.1);

        match self.layout[row - 1][col] {
            b'.' => true,
            b'[' => {
                self.can_push_up_from_coord((row - 1, col))
                    && self.can_push_up_from_coord((row - 1, col + 1))
            }
            b']' => {
                self.can_push_up_from_coord((row - 1, col))
                    && self.can_push_up_from_coord((row - 1, col - 1))
            }
            _ => false,
        }
    }

    fn push_up_from_coord(&mut self, coord: Coord) {
        let (row, col) = (coord.0, coord.1);

        match self.layout[row - 1][col] {
            b'[' => {
                self.push_up_from_coord((row - 1, col));
                self.push_up_from_coord((row - 1, col + 1));
            }
            b']' => {
                self.push_up_from_coord((row - 1, col));
                self.push_up_from_coord((row - 1, col - 1));
            }
            _ => (),
        }

        (self.layout[row][col], self.layout[row - 1][col]) =
            (self.layout[row - 1][col], self.layout[row][col]);
    }

    fn push_down_for_wide_grid(&mut self) {
        let (row, col) = self.robot;

        if self.can_push_down_from_coord((row, col)) {
            self.push_down_from_coord((row, col));
            self.robot = (row + 1, col);
        }
    }

    fn can_push_down_from_coord(&self, coord: Coord) -> bool {
        let (row, col) = (coord.0, coord.1);

        match self.layout[row + 1][col] {
            b'.' => true,
            b'[' => {
                self.can_push_down_from_coord((row + 1, col))
                    && self.can_push_down_from_coord((row + 1, col + 1))
            }
            b']' => {
                self.can_push_down_from_coord((row + 1, col))
                    && self.can_push_down_from_coord((row + 1, col - 1))
            }
            _ => false,
        }
    }

    fn push_down_from_coord(&mut self, coord: Coord) {
        let (row, col) = (coord.0, coord.1);

        match self.layout[row + 1][col] {
            b'[' => {
                self.push_down_from_coord((row + 1, col));
                self.push_down_from_coord((row + 1, col + 1));
            }
            b']' => {
                self.push_down_from_coord((row + 1, col));
                self.push_down_from_coord((row + 1, col - 1));
            }
            _ => (),
        }

        (self.layout[row][col], self.layout[row + 1][col]) =
            (self.layout[row + 1][col], self.layout[row][col]);
    }
}

fn parse_input_into_grid_and_pushes(input: &str) -> Result<(Grid, Vec<u8>)> {
    let Some((grid, pushes)) = input.split_once("\n\n") else {
        return Err(anyhow!(
            "Cannot split input into grid and pushes: {}",
            input
        ));
    };

    let mut robot = (0, 0);
    let layout = grid
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, byte)| {
                    if byte == b'@' {
                        robot = (row, col);
                    }

                    byte
                })
                .collect()
        })
        .collect();

    let pushes = pushes.replace("\n", "").bytes().collect();

    Ok((
        Grid {
            layout,
            robot,
            is_wide: false,
        },
        pushes,
    ))
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn example_1a() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 10092);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        let example = r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

        assert_eq!(part_1(trim_newlines(example))?, 2028);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 9021);

        Ok(())
    }
}
