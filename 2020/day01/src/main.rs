use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut seen_residuals: HashSet<u32> = HashSet::new(); // { 2020 - num }
    let mut double_residuals: HashMap<u32, u32> = HashMap::new(); // { (2020 - num1 - num2): num1 * num2 }

    for number in input.lines().map(|line| line.parse::<u32>().unwrap()) {
        let residual = 2020 - number;

        if seen_residuals.contains(&number) {
            p1 = residual*number;
        }

        seen_residuals.insert(residual);

        if double_residuals.keys().contains(&number) {
            p2 = double_residuals[&number] * number;
        } else {
            for seen_residual in seen_residuals.iter() {
                double_residuals.insert(seen_residual - number,(2020 - seen_residual) * number);
            }
        }
    }

    println!("p1: {}\np2: {}", p1, p2);

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
