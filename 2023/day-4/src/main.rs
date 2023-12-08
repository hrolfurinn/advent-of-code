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

    Ok(())
}
