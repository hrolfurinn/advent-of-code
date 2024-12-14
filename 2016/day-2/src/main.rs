use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    println!("p1: {p1}\np2: {p2}");

    let numpad = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];

    let mut code = Vec::new();

    for line in input.lines() {
        let (x,y) = line.trim().chars().fold((1_i32,1_i32), |(acc_x,acc_y),char| match char {
            'R' => ((acc_x+1).min(2),acc_y),
            'L' => ((acc_x-1).max(0),acc_y),
            'U' => (acc_x,(acc_y-1).max(0)),
            'D' => (acc_x,(acc_y+1).min(2)),
            _ => unreachable!("AHH")
        });
        code.push(numpad[y as usize][x as usize]);
    }
    for digit in code.iter() {
        print!("{digit}");
    }

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
