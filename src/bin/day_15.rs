use anyhow::{anyhow, Result};

const INPUT_FILE: &str = "inputs/day-15.txt";

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let (mut grid, mut robot_pos, instructions) = parse_input(input)?;

    for instruction in instructions {
        robot_pos = traverse(robot_pos, instruction, &mut grid)?;
    }

    Ok(sum_gps_coordinates(&grid))
}

fn part_2(input: String) -> Result<usize> {
    let (grid, robot_pos, instructions) = parse_input(input)?;

    let mut grid = expand_grid(&grid);
    let mut robot_pos = expand_robot_pos(&robot_pos);

    for instruction in instructions {
        robot_pos = traverse(robot_pos, instruction, &mut grid)?;
    }

    Ok(sum_gps_coordinates(&grid))
}

type Coord = (usize, usize);

fn parse_input(input: String) -> Result<(Vec<Vec<u8>>, Coord, Vec<u8>)> {
    let input = input.split_terminator("\n").map(str::to_string).collect::<Vec<_>>();
    let mut sections = input.split(|s| s.is_empty());

    let Some(grid_section) = sections.next() else {
        return Err(anyhow!("Cannot find grid input"));
    };
    let Some(instructions_section) = sections.next() else {
        return Err(anyhow!("Cannot find instruction input"));
    };

    let (grid, robot_pos) = parse_lines_to_grid(grid_section)?;
    let instructions = parse_lines_to_instructions(instructions_section);

    Ok((grid, robot_pos, instructions))
}

/// Parses lines into the grid, and returns the starting location of the robot.
fn parse_lines_to_grid(lines: &[String]) -> Result<(Vec<Vec<u8>>, Coord)> {
    let mut grid = lines
        .iter()
        .map(|l| l.as_bytes().to_owned())
        .collect::<Vec<Vec<_>>>();

    let Some(row) = grid.iter().position(|r| r.contains(&b'@')) else {
        return Err(anyhow!("Cannot locate robot row"));
    };
    let Some(col) = grid[row].iter().position(|c| c == &b'@') else {
        return Err(anyhow!("Cannot locate robot col"));
    };

    // Clear the original robot position.
    grid[row][col] = b'.';

    Ok((grid, (row, col)))
}

/// Parses lines into the sequence of robot movements.
fn parse_lines_to_instructions(lines: &[String]) -> Vec<u8> {
    lines.join("").replace("\n", "").as_bytes().to_owned()
}

/// Moves the robot by one [instruction].
///
/// Returns the new position of the robot.
///
/// The [grid] is updated to reflect the move.
fn traverse(robot_pos: Coord, instruction: u8, grid: &mut [Vec<u8>]) -> Result<Coord> {
    match instruction {
        b'^' => push_up(robot_pos, grid),
        b'>' => push_right(robot_pos, grid),
        b'v' => push_down(robot_pos, grid),
        b'<' => push_left(robot_pos, grid),
        x => Err(anyhow!("Unexpected instruction: {}", x)),
    }
}

/// Pushes robot or box at [pos] upwards.
///
/// Returns the final position of the robot or box.
fn push_up(pos: Coord, grid: &mut [Vec<u8>]) -> Result<Coord> {
    assert!(pos.0 > 0);

    let (row, col) = pos;
    let on_top = grid[row - 1][col];

    if on_top == b'#' {
        return Ok(pos);
    }

    if on_top == b'O' {
        push_up((row - 1, col), grid)?;
    }

    if on_top == b'[' && can_push_up((row, col), grid)? {
        push_up((row - 1, col), grid)?;
        push_up((row - 1, col + 1), grid)?;
    }

    if on_top == b']' && can_push_up((row, col), grid)? {
        push_up((row - 1, col), grid)?;
        push_up((row - 1, col - 1), grid)?;
    }

    if grid[row - 1][col] == b'.' {
        // Swap.
        grid[row - 1][col] = grid[row][col];
        grid[row][col] = b'.';

        return Ok((row - 1, col));
    }

    Ok(pos)
}

/// Pushes robot or box at [pos] to the right.
///
/// Returns the final position of the robot or box.
fn push_right(pos: Coord, grid: &mut [Vec<u8>]) -> Result<Coord> {
    let (row, col) = pos;
    let to_the_right = grid[row][col + 1];

    if to_the_right == b'#' {
        return Ok(pos);
    }

    if to_the_right == b'O' || to_the_right == b'[' || to_the_right == b']' {
        push_right((row, col + 1), grid)?;
    }

    if grid[row][col + 1] == b'.' {
        // Swap.
        grid[row][col + 1] = grid[row][col];
        grid[row][col] = b'.';

        return Ok((row, col + 1));
    }

    Ok(pos)
}

/// Pushes robot or box at [pos] downwards.
///
/// Returns the final position of the robot or box.
fn push_down(pos: Coord, grid: &mut [Vec<u8>]) -> Result<Coord> {
    let (row, col) = pos;
    let below = grid[row + 1][col];

    if below == b'#' {
        return Ok(pos);
    }

    if below == b'O' {
        push_down((row + 1, col), grid)?;
    }

    if below == b'[' && can_push_down((row, col), grid)? {
        push_down((row + 1, col), grid)?;
        push_down((row + 1, col + 1), grid)?;
    }

    if below == b']' && can_push_down((row, col), grid)? {
        push_down((row + 1, col), grid)?;
        push_down((row + 1, col - 1), grid)?;
    }

    if grid[row + 1][col] == b'.' {
        // Swap.
        grid[row + 1][col] = grid[row][col];
        grid[row][col] = b'.';

        return Ok((row + 1, col));
    }

    Ok(pos)
}

/// Pushes robot or box at [pos] to the left.
///
/// Returns the final position of the robot or box.
fn push_left(pos: Coord, grid: &mut [Vec<u8>]) -> Result<Coord> {
    assert!(pos.1 > 0);

    let (row, col) = pos;
    let to_the_left = grid[row][col - 1];

    if to_the_left == b'#' {
        return Ok(pos);
    }

    if to_the_left == b'O' || to_the_left == b'[' || to_the_left == b']' {
        push_left((row, col - 1), grid)?;
    }

    if grid[row][col - 1] == b'.' {
        // Swap.
        grid[row][col - 1] = grid[row][col];
        grid[row][col] = b'.';

        return Ok((row, col - 1));
    }

    Ok(pos)
}

/// Sums up the GPS coordinates of all the boxes in [grid].
fn sum_gps_coordinates(grid: &[Vec<u8>]) -> usize {
    let mut sum = 0;

    for (row, l) in grid.iter().enumerate() {
        for (col, c) in l.iter().enumerate() {
            if *c == b'O' || *c == b'[' {
                sum += 100 * row + col;
            }
        }
    }

    sum
}

/// Expands grid for part 2.
fn expand_grid(grid: &[Vec<u8>]) -> Vec<Vec<u8>> {
    grid.iter()
        .map(|r| {
            r.iter()
                .flat_map(|c| match c {
                    b'O' => vec![b'[', b']'],
                    x => vec![*x, *x],
                })
                .collect()
        })
        .collect()
}

/// Updates the robot position for part 2.
fn expand_robot_pos(robot_pos: &Coord) -> Coord {
    (robot_pos.0, robot_pos.1 * 2)
}

/// Checks whether it is possible to push the box or robot at [pos] upwards.
fn can_push_up(pos: Coord, grid: &[Vec<u8>]) -> Result<bool> {
    assert!(pos.0 > 0);

    let (row, col) = pos;

    match grid[row - 1][col] {
        b'#' => Ok(false),
        b'.' => Ok(true),
        b'[' => Ok(can_push_up((row - 1, col), grid)? && can_push_up((row - 1, col + 1), grid)?),
        b']' => Ok(can_push_up((row - 1, col), grid)? && can_push_up((row - 1, col - 1), grid)?),
        x => Err(anyhow!("Unexpected grid value: {}", x)),
    }
}

/// Checks whether it is possible to push the box or robot at [pos] upwards.
fn can_push_down(pos: Coord, grid: &[Vec<u8>]) -> Result<bool> {
    let (row, col) = pos;

    match grid[row + 1][col] {
        b'#' => Ok(false),
        b'.' => Ok(true),
        b'[' => {
            Ok(can_push_down((row + 1, col), grid)? && can_push_down((row + 1, col + 1), grid)?)
        }
        b']' => {
            Ok(can_push_down((row + 1, col), grid)? && can_push_down((row + 1, col - 1), grid)?)
        }
        x => Err(anyhow!("Unexpected grid value: {}", x)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
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
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 10092);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 9021);

        Ok(())
    }
}
