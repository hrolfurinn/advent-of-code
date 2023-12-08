use core::num;
use std::collections::{VecDeque, vec_deque};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Result};
use std::ops::Add;
use std::str::FromStr;
use std::thread::current;

fn neighbor_check(current_lines: &VecDeque<String>, index: usize, num_length: usize) -> bool {
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
        return true;
    } else {
        return false;
    }
}

fn ratio_check(current_lines: &VecDeque<String>, index: usize) -> Option<i32> {
    Some(2)
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    // let previous_line = reader.lines().next();
    // let next_line = reader.lines().next();

    let mut lines = reader.lines();
    let mut first_line = lines.next().unwrap()?;

    macro_rules! pad {
        ($s:expr) => {
            format!(".{}.", $s)
        }
    }

    let dummy_line: String = ["."].repeat(first_line.len() + 2).concat().into();

    let mut current_lines: VecDeque<String> = VecDeque::new();
    current_lines.push_front(dummy_line.clone());
    current_lines.push_front(pad!(first_line));

    let mut ongoing: bool = true;

    while ongoing {
        // println!("{:?}", ".".to_string().repeat(50));
        // println!("NEW LINE");
        // println!("{:?}", line);
        if let Some(Ok(next_line)) = lines.next() {
            current_lines.push_front(pad!(next_line));
        } else {
            current_lines.push_front(dummy_line.clone());
            ongoing = false;
        }

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
                // println!("Found number {:?}", number);
                skip = number.len() - 1;
                if neighbor_check(&current_lines, ix, number.len()) {
                    // println!("Adjacent.");
                    p1 += number.parse::<i32>().unwrap();
                } else {
                    // println!("Not adjacent!")
                }
            }
            if *character == '*' {
                if let Some(ratio) = ratio_check(&current_lines, ix) {
                    p2 += ratio;
                } else {

                }
            }
        }

        current_lines.pop_back();
    }
    println!("p1 {p1}");
    println!("p2 {p2}");

    Ok(())
}
