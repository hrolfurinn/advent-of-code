use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn find(
    towels: &HashSet<Vec<u8>>,
    longest: u64,
    pattern: &Vec<u8>,
    cache: &mut HashMap<Vec<u8>, u64>,
) -> u64 {
    if let Some(solutions) = cache.get(pattern) {
        return *solutions;
    }

    let mut solutions = 0;

    if towels.contains(pattern) { solutions += 1; }

    for cut in 1..(longest + 1).min(pattern.len() as u64) {
        let (first_part, second_part) = pattern.split_at(cut as usize);
        if towels.contains(&first_part.to_vec()) {
            solutions += find(&towels, longest, &second_part.to_vec(), cache);
        }
    }

    cache.insert(pattern.to_vec(), solutions);
    return solutions;
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let (head, tail) = input.split_once("\n\n").unwrap();

    let towels = HashSet::from_iter(head.split(", ").map(|towel| towel.bytes().collect_vec()));

    let longest = towels.iter().map(|towel| towel.len() as u64).max().unwrap();

    let mut cache: HashMap<Vec<u8>, u64> = HashMap::new();

    for pattern in tail.lines().map(|line| line.bytes().collect_vec()) {
        let solutions = find(&towels, longest, &pattern, &mut cache);

        p2 += solutions;

        if solutions != 0 { p1 += 1; }
    }

    println!("p1: {}\np2: {}", p1, p2);

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
