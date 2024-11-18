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
    let mut santa_position = (0,0);
    let mut robo_santa_position = (0,0);
    let mut santas_turn = true;
    visited_homes.insert(santa_position);
    for char in directions.chars() {
        let directions = get_direction(char);
        if santas_turn {
            santa_position.0 += directions.0;
            santa_position.1 += directions.1;
            visited_homes.insert(santa_position);
        } else {
            robo_santa_position.0 += directions.0;
            robo_santa_position.1 += directions.1;
            visited_homes.insert(robo_santa_position);
        }
        santas_turn = !santas_turn
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