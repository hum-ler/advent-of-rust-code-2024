use std::fs::read_to_string;

use anyhow::{anyhow, Result};

const EXAMPLE_INPUT: &str = "2333133121414131402";

const INPUT_FILE: &str = "inputs/day-9.txt";

const ASCII_ZERO: u8 = 48;

pub fn run_example_1() -> Result<usize> {
    part_1(EXAMPLE_INPUT)
}

pub fn run_part_1() -> Result<usize> {
    part_1(read_to_string(INPUT_FILE)?.trim())
}

pub fn run_example_2() -> Result<usize> {
    part_2(EXAMPLE_INPUT)
}

pub fn run_part_2() -> Result<usize> {
    part_2(read_to_string(INPUT_FILE)?.trim())
}

fn part_1(input: &str) -> Result<usize> {
    let disk = parse_disk_map_to_disk(input);

    let disk = compact(disk)?;

    Ok(checksum(&disk))
}

fn part_2(input: &str) -> Result<usize> {
    let blocks = parse_disk_map_to_contiguous_blocks(input);

    let blocks = compact_contiguous_blocks(blocks)?;

    // Convert back to same disk format as part 1.
    let disk = convert_contiguous_blocks_to_disk(blocks);

    Ok(checksum(&disk))
}

fn parse_disk_map_to_disk(input: &str) -> Vec<Option<usize>> {
    input
        .bytes()
        .enumerate()
        .flat_map(|(id, size)| {
            assert!(size >= ASCII_ZERO);

            let size = usize::from(size - ASCII_ZERO);

            if id % 2 == 0 {
                // File
                vec![Some(id / 2); size]
            } else {
                // Free space
                vec![None; size]
            }
        })
        .collect()
}

fn compact(mut disk: Vec<Option<usize>>) -> Result<Vec<Option<usize>>> {
    assert!(!disk.is_empty());

    for head in 0..disk.len() {
        if disk[head].is_none() {
            let Some(tail) = disk.iter().rposition(|e| e.is_some()) else {
                return Err(anyhow!("Cannot locate last file block"));
            };

            if head > tail {
                break;
            }

            disk.swap(head, tail);
        }
    }

    Ok(disk)
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .map(|(pos, id)| if let Some(id) = id { id * pos } else { 0 })
        .sum()
}

#[derive(Clone, Copy)]
struct ContiguousBlock {
    pub id: Option<usize>,
    pub size: usize,
}

impl ContiguousBlock {
    pub fn new(id: Option<usize>, size: usize) -> Self {
        Self { id, size }
    }

    pub fn is_file(&self) -> bool {
        self.id.is_some()
    }

    pub fn is_free_space(&self) -> bool {
        self.id.is_none()
    }

    pub fn fits_into(&self, other: &ContiguousBlock) -> bool {
        if self.is_free_space() || other.is_file() {
            return false;
        }

        self.size <= other.size
    }

    pub fn shrink(&self, amount: usize) -> Result<Self> {
        if self.is_file() {
            return Err(anyhow!("Cannot shrink a file"));
        }

        if self.size <= amount {
            return Err(anyhow!("Not enough space"));
        }

        Ok(Self {
            id: self.id,
            size: self.size - amount,
        })
    }

    pub fn expand(&self, amount: usize) -> Result<Self> {
        if self.is_file() {
            return Err(anyhow!("Cannot expand a file"));
        }

        Ok(Self {
            id: self.id,
            size: self.size + amount,
        })
    }
}

fn parse_disk_map_to_contiguous_blocks(input: &str) -> Vec<ContiguousBlock> {
    input
        .bytes()
        .enumerate()
        .filter_map(|(id, size)| {
            assert!(size >= ASCII_ZERO);

            let size = usize::from(size - ASCII_ZERO);

            if id % 2 == 0 {
                // File
                Some(ContiguousBlock::new(Some(id / 2), size))
            } else {
                // Free space
                if size > 0 {
                    Some(ContiguousBlock::new(None, size))
                } else {
                    None
                }
            }
        })
        .collect()
}

fn compact_contiguous_blocks(mut blocks: Vec<ContiguousBlock>) -> Result<Vec<ContiguousBlock>> {
    let Some(largest_id) = blocks
        .iter()
        .filter(|b| b.is_file())
        .filter_map(|b| b.id)
        .max_by(|x, y| x.cmp(y))
    else {
        return Err(anyhow!("Cannot find largest file ID"));
    };

    for id in (0..=largest_id).rev() {
        // Get the file.
        let Some(mut file_pos) = blocks.iter().position(|b| b.id == Some(id)) else {
            return Err(anyhow!("Cannot find position of file: {}", id));
        };
        let file = blocks[file_pos];

        // Find the next free space that is big enough.
        if let Some(free_space_pos) = blocks
            .iter()
            .enumerate()
            .position(|(pos, b)| pos < file_pos && b.is_free_space() && file.fits_into(b))
        {
            let free_space = blocks[free_space_pos];

            // Swap blocks between file and free space.
            if free_space.size == file.size {
                // Direct swap if possible.
                blocks.swap(free_space_pos, file_pos);
            } else {
                // Replace orig file with new free space.
                blocks[file_pos] = ContiguousBlock::new(None, file.size);

                // Insert the file and shrink the orig free space.
                blocks[free_space_pos] = free_space.shrink(file.size)?;
                blocks.insert(free_space_pos, file);
                file_pos += 1;
            }

            // Merge free spaces around file_pos.

            if file_pos < blocks.len() - 1 && blocks[file_pos + 1].is_free_space() {
                let second_block = blocks.remove(file_pos + 1);
                blocks[file_pos] = blocks[file_pos].expand(second_block.size)?;
            }

            if file_pos > 0 && blocks[file_pos - 1].is_free_space() {
                let second_block = blocks.remove(file_pos);
                blocks[file_pos - 1] = blocks[file_pos - 1].expand(second_block.size)?;
            }
        }
    }

    Ok(blocks)
}

/// Converts blocks to the same disk format as part 1.
fn convert_contiguous_blocks_to_disk(blocks: Vec<ContiguousBlock>) -> Vec<Option<usize>> {
    blocks.iter().flat_map(|b| vec![b.id; b.size]).collect()
}
