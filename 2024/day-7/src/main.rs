use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

fn powerset_3(s: Vec<u32>) -> Vec<Vec<u32>> {
    (0..3usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().map(|(t, _)| ((i as u32) / 3_u32.pow(t as u32)) % 3_u32).collect_vec()
     }).collect()
}   

fn powerset_2(s: Vec<u32>) -> Vec<Vec<u32>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().map(|(t, _)| ((i as u32) / 2_u32.pow(t as u32)) % 2_u32).collect_vec()
     }).collect()
}   

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let (target_str, numbers_str) = line.split_once(":").unwrap();
        let numbers = numbers_str.trim().split(" ").map(|num| num.parse::<u64>().unwrap()).collect::<Vec<u64>>();
        let target = target_str.trim().parse::<u64>().unwrap();

        let mut numbers_iter = numbers.iter();
        let first_num = numbers_iter.next().unwrap();

        let subsets = powerset_2((0..numbers.len() - 1).map(|num| num as u32).collect_vec());
        for subset in subsets {
            let new_first_num = first_num.clone();
            let value_candidate = numbers_iter.clone().enumerate().fold(new_first_num, |acc, (ix, num)| {
                if subset[ix] == 1 {
                    return acc * num;
                } else {
                    return acc + num;
                }
            });
            if value_candidate == target { p1 += target; break;}
        }
        let subsets = powerset_3((0..numbers.len() - 1).map(|num| num as u32).collect_vec());
        for subset in subsets {
            let new_first_num = first_num.clone();
            let value_candidate = numbers_iter.clone().enumerate().fold(new_first_num, |acc, (ix, num)| {
                if subset[ix] == 2 {
                    acc * 10_u64.pow(num.ilog10() + 1) + num
                }else if subset[ix] == 1{
                    return acc * num;
                } else {
                    return acc + num;
                }
            });
            if value_candidate == target { p2 += target; break;}
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
