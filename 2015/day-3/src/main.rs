use std::fs::read_to_string;
use std::io::Result;

// Use AHashSet for faster hashing
use ahash::AHashSet;

#[inline(always)]
fn encode_position(x: i32, y: i32) -> i64 {
    ((x as i64) << 32) | (y as u32 as i64)
}

fn process_line(directions: &str) -> usize {
    let mut visited_homes = AHashSet::with_capacity(directions.len());
    let mut positions = [(0i32, 0i32), (0i32, 0i32)]; // [Santa, Robo-Santa]
    visited_homes.insert(encode_position(0, 0));

    for (i, &byte) in directions.as_bytes().iter().enumerate() {
        let idx = i & 1; // 0 for Santa, 1 for Robo-Santa
        match byte {
            b'>' => positions[idx].0 += 1,
            b'<' => positions[idx].0 += -1,
            b'^' => positions[idx].1 += 1,
            b'v' => positions[idx].1 += -1,
            _ => unreachable!("Invalid direction character!"),
        };

        visited_homes.insert(encode_position(positions[idx].0, positions[idx].1));
    }
    visited_homes.len()
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);
    let input = input.trim();

    let house_count = process_line(&input);
    println!("{house_count}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file: {e}");
        std::process::exit(1);
    })
}

