use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

#[derive(Debug)]
struct PartNumber {
    number: i32,
    indexes: Vec<usize>
}

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
    let start = index - 1;
    let end = index + 1;
    let mut numbers: Vec<PartNumber> = vec![];
    let mut skip = 0;

    for line in current_lines {
        let characters: Vec<_> = line.chars().collect();
        for (ix, c) in characters.iter().enumerate() {
            if skip != 0 {
                skip -= 1;
                continue;
            }
            if c.is_digit(10) {
                let number: String = characters[ix..].iter().take_while(|c| c.is_digit(10)).collect();
                let indexes: Vec<usize> = (ix..ix + number.len()).collect();
                let part_number = PartNumber { number: number.parse::<i32>().unwrap(), indexes: indexes };
                numbers.push(part_number);
                skip = number.len();
            }
        }
    }
    let mut count = 0;
    let mut ratio = 1;

    for part_number in &numbers {
        for ix in start..=end {
            if part_number.indexes.contains(&ix) {
                count += 1;
                ratio *= part_number.number; 
                break;
            }
            if count > 2 {
                return None
            }
        }
    }
    if count < 2 {
        return None
    }
    return Some(ratio)
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let input_path = "./input/input.txt";
    let sample_input_path = "./input/sample_input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

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
            if skip != 0 {
                skip -= 1;
                continue;
            }
            if character.is_digit(10) {
                let number: String = characters[ix..]
                    .iter()
                    .take_while(|c| c.is_digit(10))
                    .collect();
                skip = number.len() - 1;
                if neighbor_check(&current_lines, ix, number.len()) {
                    p1 += number.parse::<i32>().unwrap();
                } else {
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
