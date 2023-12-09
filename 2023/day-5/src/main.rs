use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn extract_numbers(line: String) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|n| n.parse::<i32>().ok())
        .collect()
}

fn map_seed(mut seed: &i32, mut mappings: &Vec<Vec<i32>>) -> i32 {
    let mut new_seed = *seed;
    for mapping in mappings.iter() {
        if mapping[1] <= new_seed && new_seed < mapping[1] + mapping[2] {
            new_seed = mapping[0] + (new_seed - mapping[1]);
        }
    }
    new_seed
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(sample_input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let mut seeds = lines
        .next()
        .ok_or("No lines")?
        .map(extract_numbers)
        .unwrap();

    println!("{:?}", seeds);

    let mut new_block = false;
    let mut mappings: Vec<Vec<i32>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        println!("{:?}", line);
        if line.trim().is_empty() {
            println!("It's empty");
            for seed in &mut seeds {
                *seed = map_seed(seed, &mappings)
            }
            mappings.clear();
            println!("{:?}", seeds);
            continue;
        }
        let numbers = extract_numbers(line);
        if !numbers.is_empty() {
            println!("It's numbers");
            mappings.push(numbers);
        }
    }
    for seed in &mut seeds {
        *seed = map_seed(seed, &mappings)
    }
    mappings.clear();
    println!("{:?}", seeds);
    
    p1 = *seeds.iter().min().unwrap();
    
    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
