use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn get_num_value(byte: u8) -> u32 {
    if b'A' <= byte && b'Z' >= byte {
        return (byte - b'A' + 27) as u32
    }
    return (byte - b'a' + 1) as u32
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut items = line.bytes();
        let mut sacks: Vec<HashSet<u8>>= items
            .chunks(line.len() / 2)
            .into_iter()
            .map(|chunk| HashSet::from_iter(chunk.collect_vec().into_iter()))
            .collect::<Vec<_>>();
        let common = sacks[0].intersection(&sacks[1]).next().unwrap();
        println!("pure u8 {common}");
        let num_value = get_num_value(common.clone());
        println!("num value {num_value}");
        p1 += get_num_value(*common);
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
