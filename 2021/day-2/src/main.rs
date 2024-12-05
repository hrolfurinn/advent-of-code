use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in input.lines() {
        let parts = line.split_whitespace().map(|s| s.trim()).collect::<Vec<_>>();
        let dist = parts[1].parse::<i32>().unwrap();
        match parts[0] {
            "forward" => { x += dist; depth += aim * dist; },
            "down" => { y += dist; aim += dist; },
            "up" => { y -= dist; aim -= dist; },
            _ => unreachable!("NO"),
        }
    }

    p1 = x * y;
    p2 = x * depth;

    println!("p1: {p1}\np2: {p2}");

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
