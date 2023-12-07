use std::fs::File;
use std::io::{self, BufRead, Result, BufReader};
use std::str::FromStr;

fn main() -> Result<()> {
    let input_path = "./input/input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    // let previous_line = reader.lines().next();
    // let next_line = reader.lines().next();

    let mut lines = reader.lines();

    let previous_line = lines.next();
    let current_line = lines.next();

    while let Some(next_line) = lines.next() {
        println!("{:?}", next_line);
    }

    Ok(())
}
