use itertools::Itertools;
use std::fs::read_to_string;
use std::io::{Read, Result};

// Get the number of times that one would hit a multiple of 100 going from one number to a
// another number. One cannot hit a multiple of 100 at the former number, only the latter. For some
// reason, it's better to normalize such that the previous one is lower than the current one. We do
// that by flipping their sign. If they're equal, zero is fine.
fn get_touched(prev: &i32, curr: &i32) -> i32 {
    let lower = prev * (curr - prev).signum();
    let higher = curr * (curr - prev).signum();
    higher.div_euclid(100) - lower.div_euclid(100)
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    assert_eq!(get_touched(&-100, &200), 3);
    assert_eq!(get_touched(&-100, &-200), 1);
    assert_eq!(get_touched(&100, &-200), 3);
    assert_eq!(get_touched(&100, &0), 1);
    assert_eq!(get_touched(&-100, &-50), 0);
    assert_eq!(get_touched(&100, &50), 0);
    assert_eq!(get_touched(&50, &-250), 3);

    let mut p1 = 0;
    let mut p2 = 0;

    // L is subtraction, R additition
    // to model resets at 100, use remainder
    // FIXME: Since we start at a non-100 number, we can count all the times we touch a 100 number
    // if we always include end-of-move 100s as well.
    let (_, round_count, touch_count) = input
        .lines()
        .filter_map(|line| match line.is_empty() {
            true => None,
            false => Some(line.chars()),
        })
        .fold(
            (50, 0, 0),
            |(abs_val, round_count, touch_count), mut chars| {
                // TODO: Add a byte -> number parser given a string?
                // Directions are of the form: {L/R}{NN...}
                let ln = chars.clone().collect::<String>();
                println!(
                    "state: {abs_val}\n\trc: {round_count}\n\ttc: {touch_count}\n\tCOMMAND: {ln}"
                );
                let sign: i32 = match chars.next() {
                    Some('L') => -1,
                    Some('R') => 1,
                    Some(_) => !unreachable!("Faulty sign"),
                    None => !unreachable!("Empty line"),
                };
                let number = chars.collect::<String>().parse::<i32>().unwrap();
                let new_val = abs_val + (sign * number);
                (
                    new_val,
                    round_count + if (new_val % 100) == 0 { 1 } else { 0 },
                    touch_count + get_touched(&abs_val, &new_val),
                )
            },
        );
    p1 = round_count;
    p2 = touch_count;

    println!("p1: {p1}");
    println!("p2: {p2}");

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
