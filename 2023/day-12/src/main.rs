use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::io::Result;

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

fn get_numbers(string: &str) -> Vec<i64> {
    string
        .split(|c: char| !c.is_numeric() && !c.eq(&'-'))
        .filter_map(|s| match s.parse::<i64>() {
            Ok(num) => Some(num),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .collect()
}


fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let lines = input.lines();

    let mut p1: i64;
    let mut p2: i64;

    for line in lines {
        let (specs, alt_data) = line.trim().split_once(" ").expect("No space in string");
        let damage_lengths = get_numbers(alt_data);

        let mut in_damaged = false;
        let mut options = Vec::new();

        for c in specs.chars() {
            match c {
                "#" => {}
            }
        }
    }

    p1 = 0;
    p2 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
