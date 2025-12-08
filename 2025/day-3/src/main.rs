use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{Read, Result};

use itertools::Itertools;

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let p1: u64;
    let p2: u64;

    (p1, p2) = input.lines().fold((0, 0), |(p1acc, p2acc), line| {
        if line.is_empty() {
            return (p1acc, p2acc);
        };
        // Reversed parsed numbers because position_max finds the last element
        let rev_nums = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .rev()
            .collect_vec();
        let first_digit_pos = rev_nums[1..rev_nums.len()].iter().position_max().unwrap() + 1;
        let second_digit_pos = rev_nums[0..first_digit_pos].iter().position_max().unwrap();

        let p2_val = (0..12)
            .rev()
            .fold((0_u64, rev_nums.len()), |(cum, pos), idx| {
                let max_pos = rev_nums[idx..pos].iter().position_max().unwrap() + idx;
                (cum * 10 + rev_nums[max_pos], max_pos)
            })
            .0;

        (
            p1acc + rev_nums[first_digit_pos] * 10 + rev_nums[second_digit_pos],
            p2acc + p2_val,
        )
    });

    println!("p1: {p1}");
    println!("p2: {p2}");

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
