use std::fs::read_to_string;
use std::io::{Read, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let (item1, item2) = line.trim().split_once("  ").unwrap();
        list1.push(item1.trim().parse::<usize>().unwrap());
        list2.push(item2.trim().parse::<usize>().unwrap());
    }

    let mut result = 0;

    list1.sort();
    let counts = list2.iter().counts();

    for ix in (0..list2.len()) {
        result += list1[ix] * counts.get(&list1[ix]).unwrap_or(&0);
    }

    println!("{result}");




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
