use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn extract_numbers(line: String) -> Vec<i64> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect()
}

fn map_seed(mut seed: &i64, mut mappings: &Vec<Vec<i64>>) -> i64 {
    let mut new_seed = *seed;
    for mapping in mappings.iter() {
        if mapping[1] <= new_seed && new_seed < mapping[1] + mapping[2] {
            new_seed = mapping[0] + (new_seed - mapping[1]);
            return new_seed;
        }
    }
    new_seed
}

fn main() -> Result<(), Box<dyn Error>> {
    // env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let mut seeds = lines
        .next()
        .ok_or("No lines")?
        .map(extract_numbers)
        .unwrap();
    
    let mut more_seeds: Vec<i64> = vec![];

    let mut more_seeds = seeds.chunks(2).map(
        |seed_and_index| {
            let seed = seed_and_index[0];
            let count = seed_and_index[1];
            (seed..(seed + count)).collect::<Vec<_>>()
        }
    ).flatten().collect::<Vec<_>>();

    let mut new_block = false;
    let mut mappings: Vec<Vec<i64>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            for seed in &mut seeds {
                *seed = map_seed(seed, &mappings)
            }
            for seed in &mut more_seeds {
                *seed = map_seed(seed, &mappings)
            }
            mappings.clear();
            continue;
        }
        let numbers = extract_numbers(line);
        if !numbers.is_empty() {
            mappings.push(numbers);
        }
    }
    for seed in &mut seeds {
        *seed = map_seed(seed, &mappings)
    }
    for seed in &mut more_seeds {
        *seed = map_seed(seed, &mappings)
    }
    mappings.clear();

    p1 = *seeds.iter().min().unwrap();
    p2 = *more_seeds.iter().min().unwrap();

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
