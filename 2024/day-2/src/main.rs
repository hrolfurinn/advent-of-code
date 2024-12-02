use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn check_safety(parts: Vec<i32>) -> (usize, usize) {
    let mut parts = parts.iter();
    let mut curr = parts.next().unwrap();
    let mut up_down = 0;
    for (ix, val) in parts.enumerate() {
        if (val - curr).abs() > 3 {
            return (0, ix);
        };
        if (val - curr).abs() < 1 {
            return (0, ix);
        };
        if up_down == 0 {
            up_down = match val > curr {
                true => 1,
                false => -1,
                _ => unreachable!("bno"),
            };
            curr = val;
            continue;
        }
        if val > curr {
            if up_down != 1 {
                return (0, ix);
            }
        } else {
            if up_down != -1 {
                return (0, ix);
            }
        }
        curr = val;
    }
    return (1, 0);
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut parts = line
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let (is_safe, _) = check_safety(parts.clone());
        p1 += is_safe;
        if is_safe == 1 {
            p2 += 1;
            continue;
        };
        for ix in 0..parts.len() {
            let mut parts_clone = parts.clone();
            parts_clone.remove(ix);
            if check_safety(parts_clone).0 == 1 { p2 += 1; break }
        }
    }

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
