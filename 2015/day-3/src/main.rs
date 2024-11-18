use std::env;
use std::fs::read_to_string;
use std::collections::HashSet;
use std::io::Result;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn get_direction(direction_char: char) -> (i8, i8) {
    match direction_char {
        '>' => (1,0),
        '<'=> (-1,0),
        '^' => (0,1),
        'v' => (0,-1),
        _ => unreachable!("Invalid direction character"),
    }
}

fn process_line(directions: &str) -> usize {
    let mut visited_homes = HashSet::new();
    let mut position = (0,0);
    visited_homes.insert(position);
    for char in directions.chars() {
        let directions = get_direction(char);
        position.0 += directions.0;
        position.1 += directions.1;
        visited_homes.insert(position);
    };
    visited_homes.len()
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);

    let lines = input.lines();

    for line in lines {
        let house = process_line(line);
        println!("{house}");
    };
    Ok(())
}