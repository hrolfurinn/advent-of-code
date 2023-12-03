use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input_path: String = String::from("./input/input.txt");
    let sample_input_path: String = String::from("./input/sample_input.txt");

    // let f: File = File::open(input_path)?;
    let f: File = File::open(sample_input_path)?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut sum1 = 0;

    let colors = ["red", "green", "blue"];

    for line in reader.lines() {
        let mut line = line?;
        let words: Vec<_> = line.split_whitespace().map(|w| w.to_string()).collect();
        for word in words.iter() {
            println!("{word}");
        }
    }

    println!("{sum1}");

    Ok(())
}