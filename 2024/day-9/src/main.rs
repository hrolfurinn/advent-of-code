use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn print_flat_vec(flat_vec: &Vec<Option<usize>>) {
    for char in flat_vec {
        print!(
            "{}",
            if char.is_some() {
                char.unwrap().to_string()
            } else {
                ".".to_string()
            }
        )
    }
    println!();
}

fn print_blocks(blocks: &Vec<(bool, i32, Option<usize>, bool)>) {
    for block in blocks.iter() {
        print!("|");
        (0..block.1).for_each(|_| {
            print!(
                "{}",
                if block.0 {
                    ".".to_string()
                } else {
                    block.2.unwrap().to_string()
                }
            )
        });
    }
    println!();
}

fn collapse_flat_vec(flat_vec: &Vec<Option<usize>>) -> usize {
    let mut output = flat_vec.clone();
    let mut ix = 0;
    while ix < output.len() {
        print_flat_vec(&output);
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
        .map(|(ix, n)| { println!("{} * {} = {}", ix, n.unwrap(), ix * n.unwrap()); ix * n.unwrap() } )
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
        println!("In {ix}");
        print_blocks(&blocks);

        let (is_free, space, _, has_been_moved) = blocks[ix as usize];
        if is_free || has_been_moved {
            ix -= 1;
            continue;
        }
        println!("Need space: {space}");
        if let Some(block_ix) =
            blocks
                .iter()
                .enumerate()
                .position(|(block_ix, (block_is_free, block_space, _, _))| {
                    block_ix < ix as usize && *block_is_free && space.le(block_space)
                })
        {
            println!("Found needed space at {block_ix}");

            // We start by removing the to-be-moved value from the file system. Since this will be
            // replaced by free space, we need to take care to remove its neighbors as well, which
            // we do safely without checking whether one of the neighbors is the to-be-moved-to
            // block in the file system.
            let mut right_side = None; // We are not guaranteed a right side in bounds
            if ix + 1 < blocks.len() as i32 {
                right_side = Some(blocks.remove(ix as usize + 1));
            }
            let (_, space, file_id, _) = blocks.remove(ix as usize);
            let left_side = blocks.remove(ix as usize - 1); // We are guaranteed a left side in bounds

            println!("Removed all three surrounding elements");
            print_blocks(&blocks);
            if let Some(right_side) = right_side {
                if right_side.0 {
                    blocks.insert(
                        ix as usize - 1,
                        (true, space + right_side.1, None, false),
                    );
                } else {
                    blocks.insert(ix as usize - 1, right_side);
                    blocks.insert(ix as usize - 1, (true, space, None, false));
                }
            } else {
                blocks.insert(ix as usize - 1, (true, space, None, false));
            }
            if left_side.0 {
                let (_, curr_space, _, _) = blocks.remove(ix as usize - 1);
                blocks.insert(
                    ix as usize - 1,
                    (true, curr_space + left_side.1, None, false),
                );
            } else {
                blocks.insert(ix as usize - 1, left_side);
            }
            println!("Attempted merging");
            print_blocks(&blocks);

            // Removing the block of free space from the filesystem leaves all indices after it
            // shifted by 1, which includes all relevant indices in the rest of this loop
            let (block_is_free, block_space, _, _) = blocks.remove(block_ix);
            if !block_is_free {
                panic!("Found non-free block")
            };
            if ix == 8855 && block_ix == 8854 {
                println!("print here");
                println!(
                    "if statement {} {}",
                    ix as usize - 1 + 1 < blocks.len(),
                    blocks[ix as usize - 1 + 1].0
                )
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

    println!("part 2:");
    print_flat_vec(&flat_vec);
    println!("part 2 collapsing:");

    p2 += flat_vec.iter().enumerate().map(|(ix,n)| ix * n.unwrap_or(0)).sum::<usize>();

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
