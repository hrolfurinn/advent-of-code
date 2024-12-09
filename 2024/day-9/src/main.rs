use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn print_flat_vec(flat_vec: &Vec<Option<usize>>) {
    for char in flat_vec {
        print!(
            "{}|",
            if char.is_some() {
                char.unwrap().to_string()
            } else {
                "".to_string()
            }
        )
    }
    println!();
}

fn collapse_flat_vec(flat_vec: &Vec<Option<usize>>) -> usize {
    let mut output = flat_vec.clone();
    let mut ix = 0;
    while ix < output.len() {
        if output[ix].is_some() {
            ix += 1;
            continue;
        }
        while ix < output.len() {
            if let Some(last) = output.pop() {
                if last.is_none() {
                    continue;
                }
                output[ix] = last;
                break;
            }
        }
        ix += 1;
    }
    output
        .iter()
        .enumerate()
        .map(|(ix, n)| ix * n.unwrap())
        .sum()
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut blocks: Vec<(bool, i32, Option<usize>, bool)> = Vec::new(); // (is_free,space,file_id,has_been_moved)

    for (ix, space) in input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as i32)
        .enumerate()
    {
        if ix % 2 == 0 {
            blocks.push((false, space, Some(ix / 2), false));
        } else {
            blocks.push((true, space, None, false));
        }
    }

    let flat_vec = blocks
        .iter()
        .flat_map(|&(is_free, space, file_id, _)| {
            if is_free {
                vec![None; space as usize]
            } else {
                vec![Some(file_id.unwrap()); space as usize]
            }
        })
        .collect_vec();

    p1 += collapse_flat_vec(&flat_vec);

    let mut ix = blocks.len() as i32 - 1;
    while ix >= 0 {
        let (is_free, space, _, has_been_moved) = blocks[ix as usize];
        if is_free || has_been_moved {
            ix -= 1;
            continue;
        }
        if let Some(block_ix) =
            blocks
                .iter()
                .enumerate()
                .position(|(block_ix, (block_is_free, block_space, _, _))| {
                    block_ix < ix as usize && *block_is_free && space.le(block_space)
                })
        {
            let (_, space, file_id, _) = blocks.remove(ix as usize);
            blocks.insert(ix as usize, (true, space, None, false));
            
            // Removing the block of free space from the filesystem leaves all indices after it
            // shifted by 1, which includes all relevant indices in the rest of this loop
            let (block_is_free, block_space, _, _) = blocks.remove(block_ix);
            if !block_is_free {
                panic!("Found non-free block")
            };

            let mut new_block_space = block_space;
            if ix as usize - 1 + 1 < blocks.len() && ix as usize - 1 != block_ix && blocks[ix as usize - 1 + 1].0 {
                let (left_is_free, left_space, _, _) = blocks[ix as usize - 1 + 1];
                if left_is_free {
                    new_block_space += left_space;
                }
                blocks.remove(ix as usize - 1 + 1);
                blocks.remove(ix as usize - 1);
                blocks.insert(ix as usize - 1, (true, new_block_space, None, false));
            }
            if ix > 0 && blocks[ix as usize - 1 - 1].0 {
                let (left_is_free, left_space, _, _) = blocks[ix as usize - 1 - 1];
                if left_is_free {
                    new_block_space += left_space;
                }
                blocks.remove(ix as usize - 1 - 1);
                blocks.remove(ix as usize - 1 - 1);
                blocks.insert(ix as usize - 1 - 1, (true, new_block_space, None, false));
            }
            // Move block in file system
            if block_space > space {
                let prev_len = blocks.len();
                blocks.insert(block_ix, (true, block_space - space, None, false));
                if prev_len == blocks.len() {
                    panic!("Did not change length");
                }
            }
            blocks.insert(block_ix, (false, space, file_id, true));
        }
        ix -= 1;
    }
    let mut flat_vec = Vec::new();
    for (_, space, file_id, _) in blocks {
        flat_vec.extend(vec![file_id; space as usize]);
    }

    p2 += collapse_flat_vec(&flat_vec);

    println!("p1: {p1}\np2: {p2}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }
            .to_string()
        } else {
            String::new()
        }
    } else {
        String::new()
    };
    if path != "" {
        read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("Failed to read input file: {e}");
            std::process::exit(1);
        })
    } else {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    }
}
