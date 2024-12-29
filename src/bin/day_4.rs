use anyhow::Result;

const INPUT_FILE: &str = "inputs/day-4.txt";

fn main() {
    match advent_of_rust_code_2024::get_part(INPUT_FILE) {
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
    column_count: usize,
}

impl From<String> for WordSearch {
    fn from(value: String) -> Self {
        let grid = value
            .split_terminator("\n")
            .map(|s| s.to_string().into_bytes())
            .collect::<Vec<Vec<_>>>();
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
