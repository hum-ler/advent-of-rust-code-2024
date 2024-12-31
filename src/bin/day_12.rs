use std::collections::HashMap;

use anyhow::Result;

fn main() {
    match advent_of_rust_code_2024::get_part("inputs/day-12.txt") {
        Ok(advent_of_rust_code_2024::Part::Part1(input)) => println!("{:?}", part_1(input)),
        Ok(advent_of_rust_code_2024::Part::Part2(input)) => println!("{:?}", part_2(input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: String) -> Result<usize> {
    let grid = parse_input_to_grid(input);

    let regions = find_regions(&grid);

    Ok(regions
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum())
}

fn part_2(input: String) -> Result<usize> {
    let grid = parse_input_to_grid(input);

    let regions = find_regions(&grid);

    Ok(regions
        .iter()
        .map(|region| region.area() * region.sides())
        .sum())
}

/// Represents the border around a plot.
///
/// A direction is true if it is not touching another same-region plot in that direction.
#[derive(Clone, Copy)]
struct Border {
    pub north: bool,
    pub east: bool,
    pub south: bool,
    pub west: bool,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            north: true,
            east: true,
            south: true,
            west: true,
        }
    }
}

impl Border {
    /// Counts the number of directions that are true i.e. that require fencing.
    pub fn size(&self) -> usize {
        self.north as usize + self.east as usize + self.south as usize + self.west as usize
    }
}

type Coord = (usize, usize);

#[derive(Clone, Copy)]
struct Plot {
    coord: Coord,
    border: Border,
}

impl From<Coord> for Plot {
    fn from(value: Coord) -> Self {
        Self {
            coord: value,
            border: Border::default(),
        }
    }
}

struct Region {
    plots: Vec<Plot>,
}

impl Region {
    pub fn new(coords: &[Coord]) -> Self {
        Self {
            plots: Self::coords_to_plots(coords),
        }
    }

    pub fn area(&self) -> usize {
        self.plots.len()
    }

    pub fn perimeter(&self) -> usize {
        self.plots.iter().map(|plot| plot.border.size()).sum()
    }

    pub fn sides(&self) -> usize {
        self.outer_corners() + self.inner_corners()
    }

    fn coords_to_plots(coords: &[Coord]) -> Vec<Plot> {
        let plots = coords.iter().copied().map(Plot::from).collect::<Vec<_>>();

        Self::set_plot_borders(&plots)
    }

    /// Sets the correct border for each [Plot].
    ///
    /// Assumes that each border is in its default state.
    fn set_plot_borders(plots: &[Plot]) -> Vec<Plot> {
        // Create a HashMap for quick lookup.
        let mut plot_map: HashMap<Coord, Plot> = HashMap::new();
        for plot in plots {
            plot_map.insert(plot.coord, *plot);
        }

        // By default, borders are in the true state.
        //
        // If 2 plots are touching each other, remove the border sides between them.
        for plot in plots {
            // N
            if plot.coord.0 > 0 && plot_map.contains_key(&(plot.coord.0 - 1, plot.coord.1)) {
                plot_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.north = false);
                plot_map
                    .entry((plot.coord.0 - 1, plot.coord.1))
                    .and_modify(|p| p.border.south = false);
            }

            // E
            if plot_map.contains_key(&(plot.coord.0, plot.coord.1 + 1)) {
                plot_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.east = false);
                plot_map
                    .entry((plot.coord.0, plot.coord.1 + 1))
                    .and_modify(|p| p.border.west = false);
            }

            // S
            if plot_map.contains_key(&(plot.coord.0 + 1, plot.coord.1)) {
                plot_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.south = false);
                plot_map
                    .entry((plot.coord.0 + 1, plot.coord.1))
                    .and_modify(|p| p.border.north = false);
            }

            // W
            if plot.coord.1 > 0 && plot_map.contains_key(&(plot.coord.0, plot.coord.1 - 1)) {
                plot_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.west = false);
                plot_map
                    .entry((plot.coord.0, plot.coord.1 - 1))
                    .and_modify(|p| p.border.east = false);
            }
        }

        plot_map.values().copied().collect()
    }

    /// Counts the number of 90 degree corners in each plot border.
    ///
    /// L-shape = 1 corner
    /// U-shape = 2 corners
    /// Square = 4 corners
    fn outer_corners(&self) -> usize {
        self.plots
            .iter()
            .map(|plot| match plot.border.size() {
                4 => 4,
                3 => 2,
                2 if plot.border.north && plot.border.east => 1,
                2 if plot.border.east && plot.border.south => 1,
                2 if plot.border.south && plot.border.west => 1,
                2 if plot.border.west && plot.border.north => 1,
                _ => 0,
            })
            .sum()
    }

    /// Counts the number of 90 degree corners formed by 2 plots.
    ///
    /// Only L-shape applies, each side must come from a different plot.
    fn inner_corners(&self) -> usize {
        // Create a HashMap for quick lookup.
        let mut plot_map: HashMap<Coord, Plot> = HashMap::new();
        for plot in &self.plots {
            plot_map.insert(plot.coord, *plot);
        }

        let mut corners = 0;

        for plot in &self.plots {
            if plot.border.size() == 0 {
                continue;
            }

            if plot.border.size() == 4 {
                continue;
            }

            // NW
            if plot.border.west
                && !plot.border.north
                && plot.coord.0 > 0
                && plot.coord.1 > 0
                && plot_map.contains_key(&(plot.coord.0 - 1, plot.coord.1 - 1))
                && plot_map[&(plot.coord.0 - 1, plot.coord.1 - 1)].border.south
            {
                corners += 1;
            }

            // NE
            if plot.border.north
                && !plot.border.east
                && plot.coord.0 > 0
                && plot_map.contains_key(&(plot.coord.0 - 1, plot.coord.1 + 1))
                && plot_map[&(plot.coord.0 - 1, plot.coord.1 + 1)].border.west
            {
                corners += 1;
            }

            // SE
            if plot.border.east
                && !plot.border.south
                && plot_map.contains_key(&(plot.coord.0 + 1, plot.coord.1 + 1))
                && plot_map[&(plot.coord.0 + 1, plot.coord.1 + 1)].border.north
            {
                corners += 1;
            }

            // SW
            if plot.border.south
                && !plot.border.west
                && plot.coord.1 > 0
                && plot_map.contains_key(&(plot.coord.0 + 1, plot.coord.1 - 1))
                && plot_map[&(plot.coord.0 + 1, plot.coord.1 - 1)].border.east
            {
                corners += 1;
            }
        }

        corners
    }
}

fn parse_input_to_grid(input: String) -> Vec<Vec<u8>> {
    input
        .split_terminator("\n")
        .map(|s| s.as_bytes().to_owned())
        .collect()
}

fn find_regions(grid: &[Vec<u8>]) -> Vec<Region> {
    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);

    let mut regions = Vec::default();

    // Get regions by flooding.

    let mut grid = grid
        .iter()
        .map(|row| row.iter().map(|symbol| Some(*symbol)).collect())
        .collect::<Vec<Vec<Option<_>>>>();

    for row in 0..row_count {
        for col in 0..col_count {
            if let Some(symbol) = grid[row][col] {
                let filled_coords = flood_fill((row, col), Some(symbol), None, &mut grid);

                regions.push(Region::new(&filled_coords));
            }
        }
    }

    regions
}

/// Fill the region by flooding, beginning at seed.
///
/// Returns all the [Coord]s that have been filled.
fn flood_fill(
    seed: Coord,
    orig_value: Option<u8>,
    new_value: Option<u8>,
    grid: &mut [Vec<Option<u8>>],
) -> Vec<Coord> {
    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);

    let (row, col) = seed;
    assert!(row < row_count);
    assert!(col < col_count);

    // Nothing to do.
    if grid[row][col] != orig_value {
        return Vec::default();
    }

    // Fill this Coord.
    grid[row][col] = new_value;

    let mut coords = vec![seed];

    // Go 4-way.
    if row > 0 {
        coords.extend_from_slice(&flood_fill((row - 1, col), orig_value, new_value, grid));
    }
    if col < col_count - 1 {
        coords.extend_from_slice(&flood_fill((row, col + 1), orig_value, new_value, grid));
    }
    if row < row_count - 1 {
        coords.extend_from_slice(&flood_fill((row + 1, col), orig_value, new_value, grid));
    }
    if col > 0 {
        coords.extend_from_slice(&flood_fill((row, col - 1), orig_value, new_value, grid));
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r"
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
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE_INPUT.trim().to_string())?, 1930);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE_INPUT.trim().to_string())?, 1206);

        Ok(())
    }
}
