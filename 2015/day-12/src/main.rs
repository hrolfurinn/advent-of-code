use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Result;


fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut total = 0;

    let mut running_number = String::new();

    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '-' || c.is_ascii_digit() {
            running_number.push(c);
            continue
        }; 
        if running_number.is_empty() {
            continue;
        }
        let number = running_number.parse::<i32>().expect("Failed to parse string");
        total += number;
        running_number = String::new();
    }

    println!("{total}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file: {e}");
        std::process::exit(1);
    })
}
