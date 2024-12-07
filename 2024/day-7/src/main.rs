use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let (target_str, numbers_str) = line.split_once(":").unwrap();
        let mut numbers_iter = numbers_str.trim().split(" ").map(|num| num.parse::<u64>().unwrap());
        let target = target_str.trim().parse::<u64>().unwrap();

        let first_num = numbers_iter.next().unwrap();

        if numbers_iter.clone().fold(vec![first_num], |acc_vec, num| {
            acc_vec.into_iter().flat_map(|acc| {
                let add = acc + num;
                let mult = acc * num;
                vec![add,mult]
            }).filter(|val| *val <= target).collect()
        }).contains(&target) {p1 += target}

        if numbers_iter.fold(vec![first_num], |acc_vec, num| {
            acc_vec.into_iter().flat_map(|acc| {
                let add = acc + num;
                let concat = acc * 10_u64.pow(num.ilog10() + 1) + num;
                let mult = acc * num;
                vec![add,concat,mult]
            }).filter(|val| *val <= target).collect()
        }).contains(&target) {p2 += target}
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
