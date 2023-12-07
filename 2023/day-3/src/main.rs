use core::num;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::ops::Add;
use std::str::FromStr;
use std::thread::current;

fn neighbor_check(current_lines: &VecDeque<String>, index: usize, num_length: usize) -> bool {
    env::set_var("RUST_BACKTRACE", "1");
    let start = index - 1;
    let end = index + num_length;
    let indices = start..=end;
    let chars: Vec<Vec<_>> = vec![
        current_lines[0].chars().collect(),
        current_lines[1].chars().collect(),
        current_lines[2].chars().collect(),
    ];
    for ix in indices {
        if !(chars[0][ix] == '.' && chars[2][ix] == '.') {
            return true;
        }
    }
    if !(chars[1][start] == '.' && chars[1][end] == '.') {
        return false;
    } else {
        return false;
    }
}

fn main() -> Result<()> {
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(sample_input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    // let previous_line = reader.lines().next();
    // let next_line = reader.lines().next();

    let mut lines = reader.lines();

    let mut current_lines: VecDeque<String> = VecDeque::new();
    let first_line = for line in &current_lines {
        println!("{line}");
    };
    while let Some(Ok(line)) = lines.next() {
        println!("{:?}", ".".to_string().repeat(50));
        // println!("NEW LINE");
        // println!("{:?}", line);
        if current_lines.is_empty() {
            for _ in 0..3 {
                current_lines.push_back(["."].repeat(line.len() + 2).concat().into());
            }
        }
        current_lines.pop_front();
        current_lines.push_back(".".to_string().add(&line).add("."));
        assert_eq!(current_lines.len(), 3);
        let characters: Vec<_> = current_lines[1].chars().collect();
        let mut skip = 0;
        for (ix, character) in characters.iter().enumerate() {
            // println!("{:?}, {:?}", ix, character);
            if skip != 0 {
                skip -= 1;
                continue;
            }
            if character.is_digit(10) {
                // println!("A digit!");
                let number: String = characters[ix..]
                    .iter()
                    .take_while(|c| c.is_digit(10))
                    .collect();
                // println!("In lines:");
                // for cline in &current_lines {
                //     println!("{:?}", cline);
                // }
                println!("Found number {:?}", number);
                skip = number.len() - 1;
                if neighbor_check(&current_lines, ix, number.len()) {
                    println!("Adjacent.");
                    p1 += number.parse::<i32>().unwrap();
                } else {
                    println!("Not adjacent!")
                }
            }
        }
    }
    println!("{p1}");

    Ok(())
}
