use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut lines = reader.lines();

    let mut point_tracker: Vec<i32> = vec![];
    let mut counter: Vec<u32> = vec![];

    for line in lines {
        let mut line = line?;
        let (card_id, numbers) = line.split_once(":").unwrap();
        let numbers_vec: Vec<_> = numbers
            .split("|")
            .map(|s| {
                s.to_string()
                    .split_whitespace()
                    .map(|n| n.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let matched_numbers = numbers_vec[1]
            .iter()
            .filter(|n| numbers_vec[0].contains(n))
            .collect::<Vec<_>>();
        if !matched_numbers.is_empty() {
            let points = 2_i32.pow(matched_numbers.len() as u32 - 1);
            p1 += points;
            point_tracker.push(matched_numbers.len() as i32);
        } else {
            point_tracker.push(0);
        };
        counter.push(1);
    }

    println!("Counter: {:?}", counter);
    println!("Point tracker: {:?}", point_tracker);

    for card_ix in 0..counter.len() {
        match point_tracker[card_ix] {
            0 => println!("No points here {card_ix}"),
            _ => {
                if card_ix + 1 < counter.len() {
                    println!("in here");
                    for next_card_ix in card_ix + 1
                        ..(card_ix + point_tracker[card_ix] as usize + 1).min(counter.len())
                    {
                        counter[next_card_ix] += counter[card_ix];
                        println!("Added {} to {}", counter[card_ix], next_card_ix)
                    }
                }
            }
        }
        println!("After {card_ix} have counter {:?}", counter)
    }

    p2 = counter.iter().sum();

    println!("p1: {p1}");
    println!("p2: {p2}");

    Ok(())
}
