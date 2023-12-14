use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::str::Lines;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        // Ok("".to_string()) => panic!("Input empty"),
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let map: Vec<Vec<char>> = input
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();
    let [row, column] = map
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            if row.contains(&'S') {
                println!("Found S in {:?}", row.iter().collect::<String>());
                Some([
                    row_index,
                    row.iter()
                        .enumerate()
                        .find_map(|(index, c)| if c.eq(&'S') { Some(index) } else { None })
                        .unwrap(),
                ])
            } else {
                None
            }
        })
        .unwrap();

    println!("Coords: {:?} and {:?}", row, column);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
