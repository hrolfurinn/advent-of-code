use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut code_length = 0;
    let mut string_length = 0;

    for line in input.lines().map(|l| l.chars().collect::<Vec<_>>()) {
        let mut ix = 0;
        while ix < line.len() {
            if line[ix] == '"' {
                ix += 1;
                continue;
            }
            string_length += 1;
            if !(line[ix] == '\\') {
                ix += 1;
                continue;
            }
            match line[ix + 1] {
                'x' => {
                    ix += 4;
                }
                '"' | '\\' => {
                    ix += 2;
                }
                _ => {
                    unreachable!("No character found")
                }
            }
        }
        code_length += ix;
    }
    let p1_result = code_length - string_length;

    let new_chars = input
        .chars()
        .filter(|&c| c == '\\' || c == '"')
        .collect::<Vec<_>>()
        .len()
        + (2 * input.lines().collect::<Vec<_>>().len());

    println!("{p1_result}");
    println!("{new_chars}");

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
