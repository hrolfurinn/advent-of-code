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
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn get_numbers(string: String) -> Vec<i64> {
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
    let test = false;

    let input = load_input(test);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let mut numbers = get_numbers(line.to_string());
        let mut levels = 0;
        let mut last_numbers = vec![numbers[numbers.len() - 1]];
        let mut first_numbers = vec![numbers[0]];
        while numbers.iter().any(|n| !n.eq(&0)) {
            levels += 1;
            let mut new_numbers = numbers
                .windows(2)
                .map(|window| match window {
                    [a, b] => b - a,
                    _ => unreachable!("Window size not two"),
                })
                .collect::<Vec<_>>();
            if new_numbers.iter().all(|n| n.eq(&0)) {
                last_numbers.push(new_numbers[new_numbers.len() - 1]);
                first_numbers.push(new_numbers[0]);
                break
            } else {
                last_numbers.push(new_numbers[new_numbers.len() - 1]);
                first_numbers.push(new_numbers[0]);
                numbers = new_numbers;
            }
        }
        println!("{:?}", line);
        println!("{:?}", numbers);
        println!("{:?}", levels);
        let next: i64 = last_numbers.iter().sum();
        p1 += next;
        println!("{:?}", next);
        let prev: i64 = first_numbers.iter().enumerate().map(
            |(index,value)| (-1 as i64).pow(index as u32 % 2) * value
        ).sum();
        p2 += prev;
        println!("{:?}", prev);
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
