use std::fs::read_to_string;
use std::io::{Read, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut elves = Vec::new();

    let mut curr_elf = 0;

    for line in input.lines() {
        if line == "" {
            elves.push(curr_elf);
            curr_elf = 0;
            continue;
        }
        curr_elf += line.parse::<u32>().unwrap();
    }
    if curr_elf != 0 { elves.push(curr_elf) }

    for elf in elves.iter() {
        println!("{elf}");
    }

    let result: u32 = elves.iter().sorted().rev().take(3).sum();

    println!("{result}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }.to_string()
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
