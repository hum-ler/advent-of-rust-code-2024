use anyhow::{Result, anyhow};

use cli::{Part, get_part};

fn main() {
    match get_part("input/day-9.txt") {
        Ok(Part::Part1(input)) => println!("{:?}", part_1(&input)),
        Ok(Part::Part2(input)) => println!("{:?}", part_2(&input)),
        Err(error) => println!("{:?}", error),
    }
}

fn part_1(input: &str) -> Result<u64> {
    let mut disk = expand_input_into_disk_map(input)?;

    defrag_by_blocks(&mut disk);

    Ok(disk
        .into_iter()
        .enumerate()
        .map(|(index, file_id)| {
            if let Some(file_id) = file_id {
                index as u64 * file_id
            } else {
                0
            }
        })
        .sum())
}

fn part_2(input: &str) -> Result<u64> {
    let mut chunks = parse_input_into_chunks(input)?;

    chunks = defrag_by_files(chunks)?;

    Ok(expand_chunks_into_disk_map(&chunks)
        .into_iter()
        .enumerate()
        .map(|(index, file_id)| {
            if let Some(file_id) = file_id {
                index as u64 * file_id
            } else {
                0
            }
        })
        .sum())
}

fn expand_input_into_disk_map(input: &str) -> Result<Vec<Option<u64>>> {
    let mut disk = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;
    for byte in input.bytes() {
        if !byte.is_ascii_digit() {
            return Err(anyhow!("Invalid byte: {}", byte));
        }

        let size = (byte - b'0') as usize;

        if is_file {
            disk.extend(vec![Some(file_id); size]);
            file_id += 1;
        } else {
            disk.extend(vec![None; size]);
        }

        is_file = !is_file;
    }

    Ok(disk)
}

fn defrag_by_blocks(disk: &mut [Option<u64>]) {
    let mut front_ptr = 0;
    let mut back_ptr = disk.len() - 1;

    loop {
        while disk[front_ptr].is_some() {
            front_ptr += 1;
        }
        while disk[back_ptr].is_none() {
            back_ptr -= 1;
        }

        if front_ptr >= back_ptr {
            break;
        }

        disk.swap(front_ptr, back_ptr);
    }
}

#[derive(Clone, Copy)]
enum Chunk {
    /// (file_id, file_size)
    File(u64, usize),
    Space(usize),
}

fn parse_input_into_chunks(input: &str) -> Result<Vec<Chunk>> {
    let mut disk = Vec::new();
    let mut file_id = 0;
    let mut is_file = true;
    for byte in input.bytes() {
        if !byte.is_ascii_digit() {
            return Err(anyhow!("Invalid byte: {}", byte));
        }

        let size = (byte - b'0') as usize;

        if is_file {
            disk.push(Chunk::File(file_id, size));
            file_id += 1;
        } else {
            disk.push(Chunk::Space(size));
        }

        is_file = !is_file;
    }

    Ok(disk)
}

fn defrag_by_files(mut chunks: Vec<Chunk>) -> Result<Vec<Chunk>> {
    let Some(max_file_id) = chunks
        .iter()
        .filter_map(|chunk| {
            if let Chunk::File(id, _) = chunk {
                Some(*id)
            } else {
                None
            }
        })
        .max()
    else {
        return Err(anyhow!("Cannot find max file ID"));
    };

    for file_id in (0..=max_file_id).rev() {
        // Get file pos and size.
        let Some(file_pos) = chunks.iter().position(|chunk| {
            if let Chunk::File(id, _) = chunk {
                *id == file_id
            } else {
                false
            }
        }) else {
            return Err(anyhow!("Cannot find file with ID: {}", file_id));
        };
        let Chunk::File(_, file_size) = chunks[file_pos] else {
            return Err(anyhow!("Cannot get file at pos: {}", file_pos));
        };

        // Get the first space in front that can fit the file.
        let Some(space_pos) = chunks.iter().enumerate().position(|(index, chunk)| {
            if index >= file_pos {
                return false;
            }

            if let Chunk::Space(size) = chunk {
                return *size >= file_size;
            }

            false
        }) else {
            continue;
        };
        let Chunk::Space(space_size) = chunks[space_pos] else {
            return Err(anyhow!("Cannot get space at pos: {}", space_pos));
        };

        if space_size == file_size {
            // Direct swap.
            chunks.swap(file_pos, space_pos);
        } else {
            // Insert and shrink space.
            chunks[file_pos] = Chunk::Space(file_size);
            chunks[space_pos] = Chunk::Space(space_size - file_size);
            chunks.insert(space_pos, Chunk::File(file_id, file_size));
        }

        chunks = clean_up_spaces(chunks);
    }

    Ok(chunks)
}

fn expand_chunks_into_disk_map(chunks: &[Chunk]) -> Vec<Option<u64>> {
    chunks
        .iter()
        .flat_map(|chunk| match chunk {
            Chunk::File(file_id, size) => vec![Some(*file_id); *size],
            Chunk::Space(size) => vec![None; *size],
        })
        .collect()
}

/// Cleans up spaces by merging consecutive spaces, and removing empty ones.
fn clean_up_spaces(chunks: Vec<Chunk>) -> Vec<Chunk> {
    let mut cleaned_up_chunks = vec![chunks[0]];
    let mut prev_chunk = chunks[0];

    for mut chunk in chunks.into_iter().skip(1) {
        match (prev_chunk, chunk) {
            // Remove empty space.
            (_, Chunk::Space(0)) => continue,

            // Combine spaces.
            (Chunk::Space(prev_size), Chunk::Space(size)) => {
                chunk = Chunk::Space(prev_size + size);
                cleaned_up_chunks.pop();
            }
            _ => (),
        }

        cleaned_up_chunks.push(chunk);
        prev_chunk = chunk;
    }

    cleaned_up_chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402";

    #[test]
    fn example_1() -> Result<()> {
        assert_eq!(part_1(EXAMPLE)?, 1928);

        Ok(())
    }

    #[test]
    fn example_2() -> Result<()> {
        assert_eq!(part_2(EXAMPLE)?, 2858);

        Ok(())
    }
}
