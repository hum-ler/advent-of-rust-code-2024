use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-4.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<u32> {
    let grid = parse_input_into_grid(input);

    Ok(count_xmas(&grid))
}

fn part_2(input: &str) -> Result<u32> {
    let grid = parse_input_into_grid(input);

    Ok(count_x_mas(&grid))
}

fn parse_input_into_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn count_xmas(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid.len() {
            if grid[row][col] != b'X' {
                continue;
            }

            if row >= 3
                && grid[row - 1][col] == b'M'
                && grid[row - 2][col] == b'A'
                && grid[row - 3][col] == b'S'
            {
                count += 1;
            }
            if row >= 3
                && col < grid.len() - 3
                && grid[row - 1][col + 1] == b'M'
                && grid[row - 2][col + 2] == b'A'
                && grid[row - 3][col + 3] == b'S'
            {
                count += 1;
            }
            if col < grid.len() - 3
                && grid[row][col + 1] == b'M'
                && grid[row][col + 2] == b'A'
                && grid[row][col + 3] == b'S'
            {
                count += 1;
            }
            if row < grid.len() - 3
                && col < grid.len() - 3
                && grid[row + 1][col + 1] == b'M'
                && grid[row + 2][col + 2] == b'A'
                && grid[row + 3][col + 3] == b'S'
            {
                count += 1;
            }
            if row < grid.len() - 3
                && grid[row + 1][col] == b'M'
                && grid[row + 2][col] == b'A'
                && grid[row + 3][col] == b'S'
            {
                count += 1;
            }
            if row < grid.len() - 3
                && col >= 3
                && grid[row + 1][col - 1] == b'M'
                && grid[row + 2][col - 2] == b'A'
                && grid[row + 3][col - 3] == b'S'
            {
                count += 1;
            }
            if col >= 3
                && grid[row][col - 1] == b'M'
                && grid[row][col - 2] == b'A'
                && grid[row][col - 3] == b'S'
            {
                count += 1;
            }
            if row >= 3
                && col >= 3
                && grid[row - 1][col - 1] == b'M'
                && grid[row - 2][col - 2] == b'A'
                && grid[row - 3][col - 3] == b'S'
            {
                count += 1;
            }
        }
    }

    count
}

fn count_x_mas(grid: &[Vec<u8>]) -> u32 {
    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid.len() {
            if grid[row][col] != b'A' {
                continue;
            }

            if (1..grid.len() - 1).contains(&row)
                && (1..grid.len() - 1).contains(&col)
                && ((grid[row - 1][col - 1] == b'M' && grid[row + 1][col + 1] == b'S')
                    || (grid[row - 1][col - 1] == b'S' && grid[row + 1][col + 1] == b'M'))
                && ((grid[row - 1][col + 1] == b'M' && grid[row + 1][col - 1] == b'S')
                    || (grid[row - 1][col + 1] == b'S' && grid[row + 1][col - 1] == b'M'))
            {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 18);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 9);

        Ok(())
    }
}
