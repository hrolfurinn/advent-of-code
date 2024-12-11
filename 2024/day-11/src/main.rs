use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn get_digit_num(stone: u64) -> u32 {
    if stone < 10 {
        return 1;
    }
    get_digit_num(stone / 10) + 1
}

fn count_final_stones(stone: u64, iterations: u32, cache: &mut HashMap<(u64,u32),u64>) -> u64 {
    if iterations == 0 {
        return 1;
    }
    if let Some(result) = cache.get(&(stone,iterations)) {
        return *result
    }
    if stone == 0 {
        let result = count_final_stones(1, iterations - 1, cache);
        cache.insert((stone,iterations),result);
        return result
    }
    let num_digits = get_digit_num(stone);
    if num_digits % 2 == 0 {
        let half = 10_u64.pow(num_digits / 2);
        vec![stone / half, stone % half];
        // We compute the second half before the first since it is smaller and therefore more
        // likely to affect the cache
        let second_result = count_final_stones(stone % half, iterations - 1, cache);
        let first_result = count_final_stones(stone / half, iterations - 1, cache);
        let result = first_result + second_result;
        cache.insert((stone,iterations),result);
        return result
    } else {
        let result = count_final_stones(stone * 2024, iterations - 1, cache);
        cache.insert((stone,iterations),result);
        return result
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut stones = input
        .trim()
        .split(" ")
        .map(|str| str.parse::<u64>().unwrap())
        .collect_vec();

    let mut cache = HashMap::new();

    for stone in stones.iter() {
        p1 += count_final_stones(*stone, 25, &mut cache);
    }

    for stone in stones.iter() {
        p2 += count_final_stones(*stone, 75, &mut cache);
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
