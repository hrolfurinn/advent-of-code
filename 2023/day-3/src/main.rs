use core::num;
use std::fs::File;
use std::io::{self, BufRead, Result, BufReader};
use std::str::FromStr;

fn neighbor_check(current_lines: &Vec<String>, index: usize, num_length: usize) -> bool {
    let start = index - 1;
    let end = index + num_length;
    let indices = start..=end;
    let chars: Vec<Vec<_>> = vec![current_lines[0].chars().collect(), current_lines[1].chars().collect(), current_lines[2].chars().collect()];
    for ix in indices {
        if !(chars[0][ix] == '.' && chars[2][ix] == '.') {
            return true
        }
    }
    if !(chars[1][start] == '.' && chars[1][end] == '.') {
        return false
    }
    else {
        return false
    }
}

fn main() -> Result<()> {
    let input_path = "./input/input.txt";
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut p1 = 0;
    let mut p2 = 0;

    // let previous_line = reader.lines().next();
    // let next_line = reader.lines().next();

    let mut lines = reader.lines();

    let mut current_lines: Vec<String> = Vec::new();
    for _ in 0..3 {
        if let Some(Ok(line)) = lines.next() {
            current_lines.push(line);
        }
    }
    for line in &current_lines {
        println!("{line}");
    }
    for line in [&current_lines[1]] {
        let characters: Vec<_> = line.chars().collect();
        for (ix, character) in characters.iter().enumerate() {
            // println!("{:?}, {:?}", ix, character);
            if character.is_digit(10) {
                // println!("A digit!");
                let number: String = characters[ix..].iter().take_while(|c| c.is_digit(10)).collect();
                if neighbor_check(&current_lines, ix, number.len()) {
                    println!("{:?}", number);
                    p1 += number.parse::<i32>().unwrap();    
                }
            }
        }
    }
    println!("{p1}");

    Ok(())
}
