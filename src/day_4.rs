use anyhow::Result;

use crate::{file_to_lines, string_to_lines};

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

const INPUT_FILE: &str = "inputs/day-4.txt";

pub fn run_example_1() -> Result<u32> {
    part_1(string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<u32> {
    part_1(file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<u32> {
    part_2(string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<u32> {
    part_2(file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: Vec<String>) -> Result<u32> {
    Ok(WordSearch::from(lines).tally_part_1())
}

fn part_2(lines: Vec<String>) -> Result<u32> {
    Ok(WordSearch::from(lines).tally_part_2())
}

struct WordSearch {
    grid: Vec<Vec<u8>>,
    row_count: usize,
    column_count: usize,
}

impl From<Vec<String>> for WordSearch {
    fn from(value: Vec<String>) -> Self {
        let grid = value
            .iter()
            .map(|s| s.to_owned().into_bytes())
            .collect::<Vec<Vec<u8>>>();
        let row_count = grid.len();
        let column_count = grid[0].len();

        Self {
            grid,
            row_count,
            column_count,
        }
    }
}

impl WordSearch {
    pub fn tally_part_1(&self) -> u32 {
        let mut score = 0;

        for row in 0..self.row_count {
            for column in 0..self.column_count {
                score += self.get_part_1_score_at_coord(row, column);
            }
        }

        score
    }

    fn get_part_1_score_at_coord(&self, row: usize, column: usize) -> u32 {
        if self.grid[row][column] != b'X' {
            return 0;
        }

        self.get_part_1_n_score_at_coord(row, column)
            + self.get_part_1_ne_score_at_coord(row, column)
            + self.get_part_1_e_score_at_coord(row, column)
            + self.get_part_1_se_score_at_coord(row, column)
            + self.get_part_1_s_score_at_coord(row, column)
            + self.get_part_1_sw_score_at_coord(row, column)
            + self.get_part_1_w_score_at_coord(row, column)
            + self.get_part_1_nw_score_at_coord(row, column)
    }

    // S
    // A
    // M
    // X
    fn get_part_1_n_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row < 3 {
            return 0;
        }

        if self.grid[row - 1][column] == b'M'
            && self.grid[row - 2][column] == b'A'
            && self.grid[row - 3][column] == b'S'
        {
            return 1;
        }

        0
    }

    //    S
    //   A
    //  M
    // X
    fn get_part_1_ne_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row < 3 || column >= self.column_count - 3 {
            return 0;
        }

        if self.grid[row - 1][column + 1] == b'M'
            && self.grid[row - 2][column + 2] == b'A'
            && self.grid[row - 3][column + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // XMAS
    fn get_part_1_e_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if column >= self.column_count - 3 {
            return 0;
        }

        if self.grid[row][column + 1] == b'M'
            && self.grid[row][column + 2] == b'A'
            && self.grid[row][column + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // X
    //  M
    //   A
    //    S
    fn get_part_1_se_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row >= self.row_count - 3 || column >= self.column_count - 3 {
            return 0;
        }

        if self.grid[row + 1][column + 1] == b'M'
            && self.grid[row + 2][column + 2] == b'A'
            && self.grid[row + 3][column + 3] == b'S'
        {
            return 1;
        }

        0
    }

    // X
    // M
    // A
    // S
    fn get_part_1_s_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row >= self.row_count - 3 {
            return 0;
        }

        if self.grid[row + 1][column] == b'M'
            && self.grid[row + 2][column] == b'A'
            && self.grid[row + 3][column] == b'S'
        {
            return 1;
        }

        0
    }

    //    X
    //   M
    //  A
    // S
    fn get_part_1_sw_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row >= self.row_count - 3 || column < 3 {
            return 0;
        }

        if self.grid[row + 1][column - 1] == b'M'
            && self.grid[row + 2][column - 2] == b'A'
            && self.grid[row + 3][column - 3] == b'S'
        {
            return 1;
        }

        0
    }

    // SAMX
    fn get_part_1_w_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if column < 3 {
            return 0;
        }

        if self.grid[row][column - 1] == b'M'
            && self.grid[row][column - 2] == b'A'
            && self.grid[row][column - 3] == b'S'
        {
            return 1;
        }

        0
    }

    // S
    //  A
    //   M
    //    X
    fn get_part_1_nw_score_at_coord(&self, row: usize, column: usize) -> u32 {
        assert_eq!(self.grid[row][column], b'X');

        if row < 3 || column < 3 {
            return 0;
        }

        if self.grid[row - 1][column - 1] == b'M'
            && self.grid[row - 2][column - 2] == b'A'
            && self.grid[row - 3][column - 3] == b'S'
        {
            return 1;
        }

        0
    }

    pub fn tally_part_2(&self) -> u32 {
        let mut score = 0;

        for row in 0..self.row_count {
            for column in 0..self.column_count {
                if self.check_part_2_at_coord(row, column) {
                    score += 1;
                }
            }
        }

        score
    }

    fn check_part_2_at_coord(&self, row: usize, column: usize) -> bool {
        if row == 0 || row == self.row_count - 1 || column == 0 || column == self.column_count - 1 {
            return false;
        }

        if self.grid[row][column] != b'A' {
            return false;
        }

        self.check_part_2_slash_at_coord(row, column)
            && self.check_part_2_backslash_at_coord(row, column)
    }

    //   S      M
    //  A  or  A
    // M      S
    fn check_part_2_slash_at_coord(&self, row: usize, column: usize) -> bool {
        assert_eq!(self.grid[row][column], b'A');
        assert_ne!(row, 0);
        assert_ne!(row, self.row_count - 1);
        assert_ne!(column, 0);
        assert_ne!(column, self.row_count - 1);

        (self.grid[row - 1][column + 1] == b'M' && self.grid[row + 1][column - 1] == b'S')
            || (self.grid[row - 1][column + 1] == b'S' && self.grid[row + 1][column - 1] == b'M')
    }

    // M      S
    //  A  or  A
    //   S      M
    fn check_part_2_backslash_at_coord(&self, row: usize, column: usize) -> bool {
        assert_eq!(self.grid[row][column], b'A');
        assert_ne!(row, 0);
        assert_ne!(row, self.row_count - 1);
        assert_ne!(column, 0);
        assert_ne!(column, self.row_count - 1);

        (self.grid[row - 1][column - 1] == b'M' && self.grid[row + 1][column + 1] == b'S')
            || (self.grid[row - 1][column - 1] == b'S' && self.grid[row + 1][column + 1] == b'M')
    }
}
