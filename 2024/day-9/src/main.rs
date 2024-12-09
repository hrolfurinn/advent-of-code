use itertools::Itertools;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn collapse_flat_vec(flat_vec: &Vec<Option<usize>>) -> usize {
    let mut output = flat_vec.clone();
    let mut ix = 0;
    while ix < output.len() {
        if output[ix].is_some() {
            ix += 1;
            continue;
        }
        if let Some(last) = output.pop() {
            if last.is_none() {
                continue;
            }
            output[ix] = last;
            ix += 1;
        }
    }
    output
        .iter()
        .enumerate()
        .map(|(ix, n)| ix * n.unwrap_or(0))
        .sum()
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    // We store the data efficiently, implementing a description of the file location and contents
    // instead of a data structure containing all blocks in the system and their associated values
    // in objects. The descriptions are enough for a successful logic built on top of them.
    //
    // Each block in the file system has these associated data:
    // (is_free,space,file_id,has_been_moved)
    // Note: Technically, each "block" here is "space" many blocks, based on the problem statement.
    let mut blocks: Vec<(bool, i32, Option<usize>, bool)> = Vec::new(); 

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

    // TODO: I wish I didn't have to use a for loop here but having the blocks object in the while
    // statement itself was causing me borrowing issues, presumably because the while loop didn't
    // like its children taking ownership of the blocks if it itself was using the object.
    for ix in (0..blocks.len() as i32).rev() {
        let (is_free, needed_space, _, has_been_moved) = blocks[ix as usize];
        if is_free || has_been_moved {
            continue;
        }

        // We look for a block earlier in the file system that is free and has enough space. We
        // then free up the block associate with our file, making sure to merge adjacent blocks,
        // and replace the contents of the free block with the file, leaving leftover space free.
        if let Some(free_block_ix) =
            blocks
                .iter()
                .enumerate()
                .position(|(block_ix, (block_is_free, block_space, _, _))| {
                    block_ix < ix as usize && *block_is_free && needed_space.le(block_space)
                })
        {
            // We start by removing the to-be-moved value from the file system. Since this will be
            // replaced by free space, we need to take care to remove its neighbors as well, which
            // we do safely without checking whether one of the neighbors is the to-be-moved-to
            // block in the file system. We are just freeing up the space around it and the index
            // of our free block will remain the same, and not affect the logic later.

            // We will insert and remove (ix-1,ix,ix+1) values all at the same index which simplifies
            // the logic, but then rightmost value needs to be removed last and added first in order
            // for the original order to be preserved.
            let insertion_index = ix as usize - 1;

            // Note that this working correctly assumes and ensures that there are no more than one
            // free space blocks in a row anywhere, so we don't need to look further than the
            // immediate neighbors.
            let left_side = blocks.remove(insertion_index); // We are guaranteed a left side in bounds
            let (_, needed_space, file_id, _) = blocks.remove(insertion_index);
            let mut right_side = None; // We are not guaranteed a right side in bounds
            if insertion_index < blocks.len() {
                right_side = Some(blocks.remove(insertion_index));
            }

            if let Some(right_side) = right_side {
                if right_side.0 {
                    // If the right side is free, we add free space for both the removed target
                    // file and the right side itself.
                    blocks.insert(
                        insertion_index,
                        (true, needed_space + right_side.1, None, false),
                    );
                } else {
                    // Otherwise, we place it as it was, along with a freed block where the file was
                    blocks.insert(insertion_index, right_side);
                    blocks.insert(insertion_index, (true, needed_space, None, false));
                }
            } else {
                blocks.insert(insertion_index, (true, needed_space, None, false));
            }
            if left_side.0 {
                let (_, curr_space, _, _) = blocks.remove(insertion_index);
                blocks.insert(
                    insertion_index,
                    (true, curr_space + left_side.1, None, false),
                );
            } else {
                blocks.insert(insertion_index, left_side);
            }

            let (_, free_block_space, _, _) = blocks.remove(free_block_ix);

            // We can now move the file content to the free space
            if free_block_space > needed_space {
                // Start by adding the rest of the free space to its location again
                blocks.insert(
                    free_block_ix,
                    (true, free_block_space - needed_space, None, false),
                );
            }
            // Then we can safely add the file IDs to the free space location without checks
            blocks.insert(free_block_ix, (false, needed_space, file_id, true));
        }
    }

    p2 += blocks
        .iter()
        .flat_map(|&(_, space, file_id, _)| vec![file_id; space as usize])
        .enumerate()
        .map(|(ix, n)| ix * n.unwrap_or(0))
        .sum::<usize>();

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
