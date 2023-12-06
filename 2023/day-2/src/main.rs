use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let input_path: String = String::from("./input/input.txt");
    let sample_input_path: String = String::from("./input/sample_input.txt");

    // let f: File = File::open(input_path)?;
    let f: File = File::open(input_path)?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut sum1 = 0;
    let mut id = 0;

    let colors = ["red", "green", "blue"];

    let color_count = |color: &str| {
        for (index, c) in colors.iter().enumerate() {
            if c.eq(&color) {
                return Some(index + 12);
            }
        }
        return None;
    };

    for mut line in reader.lines() {
        let line = line?;
        println!("Line: {:?}", line);
        let mut segments = line.split(":").map(|s| {
            s.split(";")
                .map(|p| p.split(",").map(|r| r.split_whitespace()))
        });
        let id = segments
            .next()
            .unwrap()
            .next()
            .unwrap()
            .next()
            .unwrap()
            .collect::<Vec<_>>()[1]
            .parse::<i64>();
        let all_correct = segments.clone().next().unwrap().all(|mut s| {
            let segment_correct = s.all(|mut p| {
                // println!({"{}"})
                let is_correct = (p.next().unwrap().parse::<i64>().unwrap_or(0) as usize)
                    .le(&color_count(p.next().unwrap()).unwrap());
                return is_correct;
            });
            println!("{:?}", &segment_correct);
            return segment_correct;
        });
        if all_correct {
            sum1 = sum1 + id.unwrap_or(0) as i64;
        }
    }

    println!("{sum1}");

    Ok(())
}
