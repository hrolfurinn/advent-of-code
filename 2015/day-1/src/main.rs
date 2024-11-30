use std::fs::read_to_string;
use std::io::{Read, Result};

fn process_line(directions: &str) -> std::result::Result<usize, &'static str> {
    let mut floor = 0;
    for (ix, char) in directions.chars().enumerate() {
        floor += match char {
            '(' => 1,
            ')' => -1,
            _ => unreachable!("Bad directions! {char}")
        };
        if floor < 0 {
            return Ok(ix + 1)
        };
    }
    return Err("No basement for")
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    for line in input.lines() {
        let end_floor = process_line(line).unwrap();
        println!("{end_floor}");
    };

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }.to_string()
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
