use itertools::Itertools;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    // R is subtraction, L additition
    // to model resets at 100, use remainder
    p1 = input
        .lines()
        .filter_map(|line| match line.is_empty() {
            true => None,
            false => Some(line.chars()),
        })
        .fold((50, 0), |(mut val, mut count), mut chars| {
            // TODO: Add a byte -> number parser given a string?
            // {L/R}{NN...}
            let sign: i32 = match chars.next() {
                Some('R') => -1,
                Some('L') => 1,
                Some(_) => !unreachable!("Faulty sign"),
                None => !unreachable!("Empty line"),
            };
            let number = chars.collect::<String>().parse::<i32>().unwrap();
            val = (val + (sign * number)) % 100;
            count += if val == 0 { 1 } else { 0 };
            print!("val: {val}\n count: {count}\n\n");
            (val, count)
        })
        .1;

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }
            .to_string()
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
