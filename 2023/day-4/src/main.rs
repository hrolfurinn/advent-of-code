use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    for line in lines {
        let mut line = line?;
        let (card_id, numbers) = line.split_once(":").unwrap();
        let numbers_vec: Vec<_> = numbers
            .split("|")
            .map(|s| {
                s.to_string()
                    .split_whitespace()
                    .map(|n| n.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        
        let matched_numbers = numbers_vec[1]
            .iter()
            .filter(|n| numbers_vec[0].contains(n))
            .collect::<Vec<_>>();
        if !matched_numbers.is_empty() {
            p1 += 2_i32.pow(
                matched_numbers
                    .len() as u32
                    - 1,
            );
        };
    }

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
