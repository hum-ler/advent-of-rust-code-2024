use anyhow::Result;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-4.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<u32> {
    Ok(WordSearch::from(input).tally_part_1())
}

fn part_2(input: String) -> Result<u32> {
    Ok(WordSearch::from(input).tally_part_2())
}

struct WordSearch {
    grid: Vec<Vec<u8>>,
    row_count: usize,
    col_count: usize,
}

impl From<String> for WordSearch {
    fn from(value: String) -> Self {
        let grid = value
            .split_terminator("\n")
            .map(|line| line.to_string().into_bytes())
            .collect::<Vec<Vec<_>>>();

        let row_count = grid.len();
        let col_count = grid.first().map_or(0, Vec::len);

        Self {
            grid,
            row_count,
            col_count,
        }
    }
}

impl WordSearch {
    pub fn tally_part_1(&self) -> u32 {
        let mut score = 0;

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                score += self.get_part_1_score_at_coord(row, col);
            }
        }

        score
    }

    fn get_part_1_score_at_coord(&self, row: usize, col: usize) -> u32 {
        if self.grid[row][col] != b'X' {
            return 0;
        }

        self.get_part_1_n_score_at_coord(row, col)
            + self.get_part_1_ne_score_at_coord(row, col)
            + self.get_part_1_e_score_at_coord(row, col)
            + self.get_part_1_se_score_at_coord(row, col)
            + self.get_part_1_s_score_at_coord(row, col)
            + self.get_part_1_sw_score_at_coord(row, col)
            + self.get_part_1_w_score_at_coord(row, col)
            + self.get_part_1_nw_score_at_coord(row, col)
    }

    // S
    // A
    // M
    // X
    fn get_part_1_n_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row < 3 {
            return 0;
        }

        if self.grid[row - 1][col] == b'M'
            && self.grid[row - 2][col] == b'A'
            && self.grid[row - 3][col] == b'S'
        {
            return 1;
        }

        0
    }

    //    S
    //   A
    //  M
    // X
    fn get_part_1_ne_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row < 3 || col >= self.col_count - 3 {
            return 0;
        }

        if self.grid[row - 1][col + 1] == b'M'
            && self.grid[row - 2][col + 2] == b'A'
            && self.grid[row - 3][col + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // XMAS
    fn get_part_1_e_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if col >= self.col_count - 3 {
            return 0;
        }

        if self.grid[row][col + 1] == b'M'
            && self.grid[row][col + 2] == b'A'
            && self.grid[row][col + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // X
    //  M
    //   A
    //    S
    fn get_part_1_se_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row >= self.row_count - 3 || col >= self.col_count - 3 {
            return 0;
        }

        if self.grid[row + 1][col + 1] == b'M'
            && self.grid[row + 2][col + 2] == b'A'
            && self.grid[row + 3][col + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // X
    // M
    // A
    // S
    fn get_part_1_s_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row >= self.row_count - 3 {
            return 0;
        }

        if self.grid[row + 1][col] == b'M'
            && self.grid[row + 2][col] == b'A'
            && self.grid[row + 3][col] == b'S'
        {
            return 1;
        }

        0
    }

    //    X
    //   M
    //  A
    // S
    fn get_part_1_sw_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row >= self.row_count - 3 || col < 3 {
            return 0;
        }

        if self.grid[row + 1][col - 1] == b'M'
            && self.grid[row + 2][col - 2] == b'A'
            && self.grid[row + 3][col - 3] == b'S'
        {
            return 1;
        }

        0
    }

    // SAMX
    fn get_part_1_w_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if col < 3 {
            return 0;
        }

        if self.grid[row][col - 1] == b'M'
            && self.grid[row][col - 2] == b'A'
            && self.grid[row][col - 3] == b'S'
        {
            return 1;
        }

        0
    }

    // S
    //  A
    //   M
    //    X
    fn get_part_1_nw_score_at_coord(&self, row: usize, col: usize) -> u32 {
        assert_eq!(self.grid[row][col], b'X');

        if row < 3 || col < 3 {
            return 0;
        }

        if self.grid[row - 1][col - 1] == b'M'
            && self.grid[row - 2][col - 2] == b'A'
            && self.grid[row - 3][col - 3] == b'S'
        {
            return 1;
        }

        0
    }

    pub fn tally_part_2(&self) -> u32 {
        let mut score = 0;

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                if self.check_part_2_at_coord(row, col) {
                    score += 1;
                }
            }
        }

        score
    }

    fn check_part_2_at_coord(&self, row: usize, col: usize) -> bool {
        if row == 0 || row == self.row_count - 1 || col == 0 || col == self.col_count - 1 {
            return false;
        }

        if self.grid[row][col] != b'A' {
            return false;
        }

        self.check_part_2_slash_at_coord(row, col)
            && self.check_part_2_backslash_at_coord(row, col)
    }

    //   S      M
    //  A  or  A
    // M      S
    fn check_part_2_slash_at_coord(&self, row: usize, col: usize) -> bool {
        assert_eq!(self.grid[row][col], b'A');
        assert_ne!(row, 0);
        assert_ne!(row, self.row_count - 1);
        assert_ne!(col, 0);
        assert_ne!(col, self.row_count - 1);

        (self.grid[row - 1][col + 1] == b'M' && self.grid[row + 1][col - 1] == b'S')
            || (self.grid[row - 1][col + 1] == b'S' && self.grid[row + 1][col - 1] == b'M')
    }

    // M      S
    //  A  or  A
    //   S      M
    fn check_part_2_backslash_at_coord(&self, row: usize, col: usize) -> bool {
        assert_eq!(self.grid[row][col], b'A');
        assert_ne!(row, 0);
        assert_ne!(row, self.row_count - 1);
        assert_ne!(col, 0);
        assert_ne!(col, self.row_count - 1);

        (self.grid[row - 1][col - 1] == b'M' && self.grid[row + 1][col + 1] == b'S')
            || (self.grid[row - 1][col - 1] == b'S' && self.grid[row + 1][col + 1] == b'M')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
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
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 18);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 9);

        Ok(())
    }
}
