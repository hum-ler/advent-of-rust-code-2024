use std::collections::HashMap;

use anyhow::Result;

use crate::{file_to_lines, string_to_lines};

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

const INPUT_FILE: &str = "inputs/day-12.txt";

pub fn run_example_1() -> Result<usize> {
    part_1(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_1() -> Result<usize> {
    part_1(&file_to_lines(INPUT_FILE)?)
}

pub fn run_example_2() -> Result<usize> {
    part_2(&string_to_lines(EXAMPLE_INPUT))
}

pub fn run_part_2() -> Result<usize> {
    part_2(&file_to_lines(INPUT_FILE)?)
}

fn part_1(lines: &[String]) -> Result<usize> {
    let grid = parse_lines_to_grid(lines);

    let regions = find_regions(&grid);

    Ok(regions.iter().map(|r| r.area() * r.perimeter()).sum())
}

fn part_2(lines: &[String]) -> Result<usize> {
    let grid = parse_lines_to_grid(lines);

    let regions = find_regions(&grid);

    Ok(regions.iter().map(|r| r.area() * r.sides()).sum())
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
    pub fn sum(&self) -> usize {
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
        self.plots.iter().map(|p| p.border.sum()).sum()
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
        // Use a HashMap for quick lookup.
        let mut hash_map: HashMap<Coord, Plot> = HashMap::new();
        for plot in plots {
            hash_map.insert(plot.coord, *plot);
        }

        for plot in plots {
            // N
            if plot.coord.0 > 0 && hash_map.contains_key(&(plot.coord.0 - 1, plot.coord.1)) {
                hash_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.north = false);
                hash_map
                    .entry((plot.coord.0 - 1, plot.coord.1))
                    .and_modify(|p| p.border.south = false);
            }

            // E
            if hash_map.contains_key(&(plot.coord.0, plot.coord.1 + 1)) {
                hash_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.east = false);
                hash_map
                    .entry((plot.coord.0, plot.coord.1 + 1))
                    .and_modify(|p| p.border.west = false);
            }

            // S
            if hash_map.contains_key(&(plot.coord.0 + 1, plot.coord.1)) {
                hash_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.south = false);
                hash_map
                    .entry((plot.coord.0 + 1, plot.coord.1))
                    .and_modify(|p| p.border.north = false);
            }

            // W
            if plot.coord.1 > 0 && hash_map.contains_key(&(plot.coord.0, plot.coord.1 - 1)) {
                hash_map
                    .entry(plot.coord)
                    .and_modify(|p| p.border.west = false);
                hash_map
                    .entry((plot.coord.0, plot.coord.1 - 1))
                    .and_modify(|p| p.border.east = false);
            }
        }

        hash_map.values().copied().collect()
    }

    /// Counts the number of 90 degree corners in each plot border.
    ///
    /// L-shape = 1 corner
    /// U-shape = 2 corners
    /// Square = 4 corners
    fn outer_corners(&self) -> usize {
        self.plots
            .iter()
            .map(|p| match p.border.sum() {
                4 => 4,
                3 => 2,
                2 if p.border.north && p.border.east => 1,
                2 if p.border.east && p.border.south => 1,
                2 if p.border.south && p.border.west => 1,
                2 if p.border.west && p.border.north => 1,
                _ => 0,
            })
            .sum()
    }

    /// Counts the number of 90 degree corners formed by 2 plots.
    ///
    /// Only L-shape applies, each side must come from a different plot.
    fn inner_corners(&self) -> usize {
        // Use a HashMap for quick lookup.
        let mut hash_map: HashMap<Coord, Plot> = HashMap::new();
        for plot in &self.plots {
            hash_map.insert(plot.coord, *plot);
        }

        let mut corners = 0;

        for plot in &self.plots {
            if plot.border.sum() == 0 {
                continue;
            }

            if plot.border.sum() == 4 {
                continue;
            }

            // NW
            if plot.border.west
                && !plot.border.north
                && plot.coord.0 > 0
                && plot.coord.1 > 0
                && hash_map.contains_key(&(plot.coord.0 - 1, plot.coord.1 - 1))
                && hash_map[&(plot.coord.0 - 1, plot.coord.1 - 1)].border.south
            {
                corners += 1;
            }

            // NE
            if plot.border.north
                && !plot.border.east
                && plot.coord.0 > 0
                && hash_map.contains_key(&(plot.coord.0 - 1, plot.coord.1 + 1))
                && hash_map[&(plot.coord.0 - 1, plot.coord.1 + 1)].border.west
            {
                corners += 1;
            }

            // SE
            if plot.border.east
                && !plot.border.south
                && hash_map.contains_key(&(plot.coord.0 + 1, plot.coord.1 + 1))
                && hash_map[&(plot.coord.0 + 1, plot.coord.1 + 1)].border.north
            {
                corners += 1;
            }

            // SW
            if plot.border.south
                && !plot.border.west
                && plot.coord.1 > 0
                && hash_map.contains_key(&(plot.coord.0 + 1, plot.coord.1 - 1))
                && hash_map[&(plot.coord.0 + 1, plot.coord.1 - 1)].border.east
            {
                corners += 1;
            }
        }

        corners
    }
}

fn parse_lines_to_grid(lines: &[String]) -> Vec<Vec<u8>> {
    lines.iter().map(|s| s.as_bytes().to_owned()).collect()
}

fn find_regions(grid: &[Vec<u8>]) -> Vec<Region> {
    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);

    let mut regions = Vec::default();

    // Get regions by flooding.

    let mut grid = grid
        .iter()
        .map(|r| r.iter().map(|s| Some(*s)).collect())
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

fn flood_fill(
    coord: Coord,
    orig_value: Option<u8>,
    new_value: Option<u8>,
    grid: &mut [Vec<Option<u8>>],
) -> Vec<Coord> {
    let row_count = grid.len();
    let col_count = grid.first().map_or(0, Vec::len);

    let (row, col) = coord;
    assert!(row < row_count);
    assert!(col < col_count);

    // Nothing to do.
    if grid[row][col] != orig_value {
        return Vec::default();
    }

    let mut coords = vec![coord];

    // Fill this coord.
    grid[row][col] = new_value;

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
