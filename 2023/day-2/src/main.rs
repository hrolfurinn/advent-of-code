use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input_path: String = String::from("./input/input.txt");
    let sample_input_path: String = String::from("./input/sample_input.txt");

    // let f: File = File::open(input_path)?;
    let f: File = File::open(sample_input_path)?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut sum1 = 0;
    let mut id = 0;

    let colors = ["red", "green", "blue"];

    for mut line in reader.lines() {
        let line = line?;
        println!("Line: {:?}", line);
        let mut words: Vec<String> = line.split_whitespace().map(|w| w.to_string()).collect();
        let mut chunks = words.chunks(2);
        let id_inner = &chunks.next().unwrap()[1];
        println!("{:?}", id_inner);
        let id = id_inner
            .parse::<u32>();
        println!("ID: {:?}", id);
        for chunk in chunks {
            println!("{:?}", chunk)
        }
    }

    println!("{sum1}");

    Ok(())
}
