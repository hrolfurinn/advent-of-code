use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn print_flat_vec(flat_vec: &Vec<String>) {
    for char in flat_vec {
        print!("{}", char)
    }
    println!();
}

fn collapse_flat_vec(flat_vec: &Vec<String>) -> usize {
    let mut output = flat_vec.clone();
    let mut ix = 0;
    while ix < output.len() {
        if output[ix] != "." {
            ix += 1;
            continue;
        }
        while let Some(last) = output.pop() {
            if last == "." {
                continue;
            }
            output[ix] = last;
            break;
        }
        ix += 1;
    }
    println!("Have output:");
    print_flat_vec(&output);
    output
        .iter()
        .enumerate()
        .map(|(ix, n)| ix * n.parse::<usize>().unwrap())
        .sum()
}

fn main() -> Result<()> {
    let test = true;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut blocks: Vec<(bool, i32, Option<usize>)> = Vec::new(); // (is_free,space,file_id)

    for (ix, space) in input
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap() as i32)
        .enumerate()
    {
        if ix % 2 == 0 {
            blocks.push((false, space, Some(ix / 2)));
        } else {
            blocks.push((true, space, None));
        }
    }

    let flat_vec = blocks
        .iter()
        .flat_map(|&(is_free, space, file_id)| {
            if is_free {
                vec![".".to_string(); space as usize]
            } else {
                vec![file_id.unwrap().to_string(); space as usize]
            }
        })
        .collect_vec();

    print_flat_vec(&flat_vec);

    p1 += collapse_flat_vec(&flat_vec);

    let flat_vec = blocks
        .iter()
        .flat_map(|&(is_free, space, file_id)| {
            if is_free {
                vec![".".to_string(); space as usize]
            } else {
                vec![file_id.unwrap().to_string(); space as usize]
            }
        })
        .collect_vec();

    print_flat_vec(&flat_vec);

    let mut flat_vec = Vec::new();

    let mut ix = 0;
    while ix < blocks.len() {
        println!("In {ix}");
        let debug_flat_vec = blocks
            .iter()
            .flat_map(|&(is_free, space, file_id)| {
                if is_free {
                    vec![".".to_string(); space as usize]
                } else {
                    vec![file_id.unwrap().to_string(); space as usize]
                }
            })
            .collect_vec();
        print_flat_vec(&debug_flat_vec);
        print_flat_vec(&flat_vec);
        let (is_free, mut space, file_id) = blocks.remove(0);
        if !is_free {
            flat_vec.extend(vec![file_id.unwrap().to_string(); space as usize]);
            ix += 1;
            continue;
        }
        println!("Need {space} space");
        while 0.lt(&space) {
            if let Some(block_ix_rev) = blocks
                .iter()
                .rev()
                .position(|(is_free, block_space, _)| (!is_free) && block_space.le(&space))
            {
                let block_ix = blocks.len() - 1 - block_ix_rev;
                let (is_free, block_space, block_file_id) = blocks[block_ix];
                if is_free {
                    panic!("Found free block")
                };
                space -= block_space;
                flat_vec.extend(vec![
                    block_file_id.unwrap().to_string();
                    block_space as usize
                ]);
                blocks[block_ix] = (true, block_space, None);
            } else {
                break;
            }
        }
        if 0.ne(&space) {
            flat_vec.extend(vec![".".to_string(); space as usize]);
            break;
        }
        ix += 1;
    }
    for (is_free, space, file_id) in blocks {
        if is_free {
            flat_vec.extend(vec![".".to_string(); space as usize]);
        } else {
            flat_vec.extend(vec![file_id.unwrap().to_string(); space as usize]);
        }
    }
    print_flat_vec(&flat_vec);

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
