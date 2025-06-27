use std::collections::{HashMap, HashSet};

use anyhow::Result;
use itertools::Itertools;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-8.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = parse_input_into_grid(input);

    Ok(grid
        .antennae
        .into_values()
        .flat_map(|antennae| antinodes(&antennae, grid.size))
        .collect::<HashSet<_>>()
        .len())
}

fn part_2(input: &str) -> Result<usize> {
    let grid = parse_input_into_grid(input);

    Ok(grid
        .antennae
        .into_values()
        .flat_map(|antennae| antinodes_with_harmonics(&antennae, grid.size))
        .collect::<HashSet<_>>()
        .len())
}

/// (row, col)
type Coord = (usize, usize);

struct Grid {
    antennae: HashMap<u8, Vec<Coord>>,
    size: usize,
}

fn parse_input_into_grid(input: &str) -> Grid {
    let lines = input.lines().collect::<Vec<_>>();

    let mut antennae: HashMap<u8, Vec<Coord>> = HashMap::new();
    let size = lines.len();

    for (row, line) in lines.into_iter().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            if byte == b'.' {
                continue;
            }

            antennae.entry(byte).or_default().push((row, col));
        }
    }

    Grid { antennae, size }
}

fn antinodes(antennae: &[Coord], grid_size: usize) -> Vec<Coord> {
    antennae
        .iter()
        .tuple_combinations()
        .flat_map(|(coord_1, coord_2)| {
            // Virtual order:
            //   antinode 0: (row_0, col_0),
            //   antenna 1:  (row_1, col_1),
            //   antenna 2:  (row_2, col_2),
            //   antinode 3: (row_3, col_3),

            let (row_1, col_1) = (coord_1.0 as isize, coord_1.1 as isize);
            let (row_2, col_2) = (coord_2.0 as isize, coord_2.1 as isize);

            let row_0 = row_1 + (row_1 - row_2);
            let col_0 = col_1 + (col_1 - col_2);
            let row_3 = row_2 + (row_2 - row_1);
            let col_3 = col_2 + (col_2 - col_1);

            let mut antinodes = Vec::new();
            if (0..grid_size as isize).contains(&row_0) && (0..grid_size as isize).contains(&col_0)
            {
                antinodes.push((row_0 as usize, col_0 as usize));
            }
            if (0..grid_size as isize).contains(&row_3) && (0..grid_size as isize).contains(&col_3)
            {
                antinodes.push((row_3 as usize, col_3 as usize));
            }

            antinodes
        })
        .collect()
}

fn antinodes_with_harmonics(antennae: &[Coord], grid_size: usize) -> Vec<Coord> {
    antennae
        .iter()
        .tuple_combinations()
        .flat_map(|(coord_1, coord_2)| {
            let mut antinodes = vec![*coord_1, *coord_2];

            // Virtual order:
            //   antinodes 0 and before: (row_0, col_0),
            //   antenna 1:              (row_1, col_1),
            //   antenna 2:              (row_2, col_2),
            //   antinodes 3 and after:  (row_3, col_3),

            let (row_1, col_1) = (coord_1.0 as isize, coord_1.1 as isize);
            let (row_2, col_2) = (coord_2.0 as isize, coord_2.1 as isize);

            let mut row_0 = row_1 + (row_1 - row_2);
            let mut col_0 = col_1 + (col_1 - col_2);
            while (0..grid_size as isize).contains(&row_0)
                && (0..grid_size as isize).contains(&col_0)
            {
                antinodes.push((row_0 as usize, col_0 as usize));

                row_0 += row_1 - row_2;
                col_0 += col_1 - col_2;
            }

            let mut row_3 = row_2 + (row_2 - row_1);
            let mut col_3 = col_2 + (col_2 - col_1);
            while (0..grid_size as isize).contains(&row_3)
                && (0..grid_size as isize).contains(&col_3)
            {
                antinodes.push((row_3 as usize, col_3 as usize));

                row_3 += row_2 - row_1;
                col_3 += col_2 - col_1;
            }

            antinodes
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE: &str = r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE))?, 14);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE))?, 34);

        Ok(())
    }
}
