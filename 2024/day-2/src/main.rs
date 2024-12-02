use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn check_safety(
    dist_set: &HashSet<i32>,
    ok_pos_dists: &HashSet<i32>,
    ok_neg_dists: &HashSet<i32>,
) -> bool {
    dist_set.is_subset(ok_pos_dists) || dist_set.is_subset(ok_neg_dists)
}

fn check_substrings(dists: &[i32], ok_dists: &HashSet<i32>) -> bool {
    let mut dists = dists.iter();
    let first = dists.next().unwrap();
    if !ok_dists.contains(first) {
        let next = dists.next().unwrap();
        let mut ok = dists
            .map(|&v| v)
            .collect::<HashSet<_>>()
            .is_subset(ok_dists);
        ok &= ok_dists.contains(next) || ok_dists.contains(&(next + first));
        return ok;
    }
    while let Some(curr) = dists.next() {
        if ok_dists.contains(curr) {
            continue;
        }
        if let Some(next) = dists.next() {
            let mut ok = ok_dists.contains(&(next + curr));
            ok &= dists
                .map(|&v| v)
                .collect::<HashSet<_>>()
                .is_subset(ok_dists);
            return ok;
        }
    }
    return true;
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let ok_pos_dists = HashSet::from([1, 2, 3]);
    let ok_neg_dists = HashSet::from([-1, -2, -3]);

    for line in input.lines() {
        let parts = line
            .split_whitespace()
            .map(|s| s.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let dists = parts.iter().tuple_windows().map(|(a, b)| b - a);
        let dist_set = dists.clone().collect::<HashSet<_>>();
        if check_safety(&dist_set, &ok_pos_dists, &ok_neg_dists) {
            p1 += 1;
            p2 += 1;
            continue;
        }
        let sign = dists.clone().filter(|&v| v > 0).collect::<Vec<_>>().len() > dists.len() - 2;
        let ok_dists;
        if sign {
            ok_dists = &ok_pos_dists;
        } else {
            ok_dists = &ok_neg_dists;
        }
        let new_method = check_substrings(&dists.collect::<Vec<_>>(), &ok_dists);
        p2 += new_method as i32;
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
