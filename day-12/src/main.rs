use std::{collections::HashSet, str::FromStr};

use anyhow::Result;

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-12.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{error:?}"),
    }
}

fn part_1(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    let regions = grid.into_regions();

    Ok(regions
        .into_iter()
        .map(|region| region.area() * region.parameter())
        .sum())
}

fn part_2(input: &str) -> Result<usize> {
    let grid = Grid::from_str(input)?;
    let regions = grid.into_regions();

    Ok(regions
        .into_iter()
        .map(|region| region.area() * region.sides())
        .sum())
}

struct Grid {
    plants: Vec<Vec<u8>>,
    size: usize,
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let size = lines.len();
        let plants = lines
            .into_iter()
            .map(|line| line.bytes().collect())
            .collect();

        Ok(Self { plants, size })
    }
}

impl Grid {
    fn into_regions(mut self) -> Vec<Region> {
        let mut regions = Vec::new();

        for row in 0..self.size {
            for col in 0..self.size {
                let plant = self.plants[row][col];
                if plant == 0 {
                    continue;
                }

                let plots = Self::flood_region(plant, (row, col), &mut self);
                regions.push(Region(plots));
            }
        }

        regions
    }

    fn flood_region(plant: u8, coord: Coord, grid: &mut Grid) -> HashSet<Coord> {
        let (row, col) = coord;

        if grid.plants[row][col] != plant {
            return HashSet::new();
        }

        grid.plants[row][col] = 0;

        let mut neighbours = Vec::new();
        if row > 0 {
            neighbours.push((row - 1, col));
        }
        if col < grid.size - 1 {
            neighbours.push((row, col + 1));
        }
        if row < grid.size - 1 {
            neighbours.push((row + 1, col));
        }
        if col > 0 {
            neighbours.push((row, col - 1));
        }

        let mut flooded_coords = neighbours
            .into_iter()
            .flat_map(|neighbour| Self::flood_region(plant, neighbour, grid))
            .collect::<HashSet<_>>();
        flooded_coords.insert((row, col));

        flooded_coords
    }
}

/// (row, col)
type Coord = (usize, usize);

struct Region(HashSet<Coord>);

impl Region {
    fn area(&self) -> usize {
        self.0.len()
    }

    fn parameter(&self) -> usize {
        self.0
            .iter()
            .map(|&(row, col)| {
                let mut fences = 4;

                if row > 0 && self.0.contains(&(row - 1, col)) {
                    fences -= 1;
                }
                if self.0.contains(&(row, col + 1)) {
                    fences -= 1;
                }
                if self.0.contains(&(row + 1, col)) {
                    fences -= 1;
                }
                if col > 0 && self.0.contains(&(row, col - 1)) {
                    fences -= 1;
                }

                fences
            })
            .sum()
    }

    fn sides(&self) -> usize {
        // The number of sides is equals to the number of corners. So look for:
        //   ox Oo oO xo x  Ox xO  x
        //   Oo ox xo oO Ox x   x xO
        // where O is the plot in question, o is a neighbour in the region, x is outside the region.

        self.0
            .iter()
            .map(|&(row, col)| {
                let mut corners = 0;

                let top = row > 0 && self.0.contains(&(row - 1, col));
                let top_right = row > 0 && self.0.contains(&(row - 1, col + 1));
                let right = self.0.contains(&(row, col + 1));
                let bottom_right = self.0.contains(&(row + 1, col + 1));
                let bottom = self.0.contains(&(row + 1, col));
                let bottom_left = col > 0 && self.0.contains(&(row + 1, col - 1));
                let left = col > 0 && self.0.contains(&(row, col - 1));
                let top_left = row > 0 && col > 0 && self.0.contains(&(row - 1, col - 1));

                if top && !top_right && right {
                    corners += 1;
                }
                if right && !bottom_right && bottom {
                    corners += 1;
                }
                if bottom && !bottom_left && left {
                    corners += 1;
                }
                if left && !top_left && top {
                    corners += 1;
                }
                if !top && !right {
                    corners += 1;
                }
                if !right && !bottom {
                    corners += 1;
                }
                if !bottom && !left {
                    corners += 1;
                }
                if !left && !top {
                    corners += 1;
                }

                corners
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use cli::trim_newlines;

    use super::*;

    const EXAMPLE_A: &str = r"
AAAA
BBCD
BBCC
EEEC
";
    const EXAMPLE_B: &str = r"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    const EXAMPLE_C: &str = r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    fn example_1a() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_A))?, 140);

        Ok(())
    }

    #[test]
    fn example_1b() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_B))?, 772);

        Ok(())
    }

    #[test]
    fn example_1c() -> Result<()> {
        assert_eq!(part_1(trim_newlines(EXAMPLE_C))?, 1930);

        Ok(())
    }

    #[test]
    fn example_2a() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_A))?, 80);

        Ok(())
    }

    #[test]
    fn example_2b() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_B))?, 436);

        Ok(())
    }

    #[test]
    fn example_2c() -> Result<()> {
        assert_eq!(part_2(trim_newlines(EXAMPLE_C))?, 1206);

        Ok(())
    }

    #[test]
    fn example_2d() -> Result<()> {
        let example = r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

        assert_eq!(part_2(trim_newlines(example))?, 236);

        Ok(())
    }

    #[test]
    fn example_2e() -> Result<()> {
        let example = r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

        assert_eq!(part_2(trim_newlines(example))?, 368);

        Ok(())
    }
}
