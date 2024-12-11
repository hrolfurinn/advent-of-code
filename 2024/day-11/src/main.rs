use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut stones = input
        .trim()
        .split(" ")
        .map(|str| str.parse::<u64>().unwrap())
        .collect_vec();

    for _ in 0..25 {
        stones = stones
            .into_iter()
            .flat_map(|stone| {
                if stone == 0 {
                    return vec![1];
                }
                let num_digits = stone.ilog10() + 1;
                if num_digits % 2 == 0 {
                    let half = 10_u64.pow(num_digits / 2);
                    vec![stone / half, stone % half]
                } else {
                    vec![stone * 2024]
                }
            })
            .collect_vec();
    }

    p1 += stones.len();

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
