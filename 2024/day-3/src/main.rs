use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let cands = input.split("mul(");

    for cand in cands {
        let Some((part1, part2)) = cand.split_once(",") else {
            continue;
        };
        let Some((part2, _)) = part2.split_once(")") else {
            continue;
        };
        let Ok(num1) = part1.parse::<u32>() else {
            continue;
        };
        let Ok(num2) = part2.parse::<u32>() else {
            continue;
        };
        p1 += num1 * num2;
    }
    let cands = input.split("do()");

    for cand in cands {
        let valid = cand.split("don't()").collect::<Vec<_>>()[0];
        let mul_cands = valid.split("mul(");
        for mul_cand in mul_cands {
            let Some((part1, part2)) = mul_cand.split_once(",") else {
                continue;
            };
            let Some((part2, _)) = part2.split_once(")") else {
                continue;
            };
            let Ok(num1) = part1.parse::<u32>() else {
                continue;
            };
            let Ok(num2) = part2.parse::<u32>() else {
                continue;
            };
            p2 += num1 * num2;
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
