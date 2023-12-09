use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(sample_input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let seeds = if let Some(Ok(line)) = lines.next() {
        line.split_whitespace()
            .filter_map(|n| {
                if let Ok(number) = n.parse::<i32>() {
                    Some(number)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    } else {
        println!("Failed to read first line.");
        vec![]
    };

    println!("{:?}", seeds);

    let mut new_block = false;
    let mut mappings: Vec<Vec<i32>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        println!("{:?}", line);
        if line.is_empty() && new_block {
            // Compute the mappings for all the seeds
        } else if line.chars().any(|c| c.is_digit(10)) {
            // Parse numbers, add to mappings variable
            println!("This is a place where I can find my mappings");
        } else {
            new_block = true
        }
    }

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
