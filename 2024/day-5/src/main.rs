use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    // We use the fact that no value will be larger than 100.
    // The orderings vector is indexed with [100*first_num + second_num],
    // which contains 'Less' iff the pair first_num|second_num was seen,
    // i.e. iff first_num < second_num, and 'Greater' iff the opposite.
    let mut orderings = vec![Equal; 100 * 100];

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let nums = line
            .split("|")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        orderings[100 * nums[0] + nums[1]] = Less;
        orderings[100 * nums[1] + nums[0]] = Greater;
    }
    for line in lines {
        let mut print_attempt = line
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        let middle_ix = print_attempt.len() / 2;
        if print_attempt.is_sorted_by(|a, b| orderings[100 * a + b] == Less) {
            p1 += print_attempt[middle_ix];
        } else {
            print_attempt.sort_by(|a, b| orderings[100 * a + b]);
            p2 += print_attempt[middle_ix];
        }
    }

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
