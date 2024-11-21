use std::fs::read_to_string;
use std::io::Result;

fn process_line(directions: &str) -> i32 {
    directions
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!("Bad directions! {c}"),
        })
        .sum()
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    for line in input.lines() {
        let end_floor = process_line(line);
        println!("{end_floor}");
    };

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
