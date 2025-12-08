use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn compare_faulties(lower: &str, upper: &str) -> (u64, u64) {
    let lower_val: u64 = lower.parse().unwrap();
    let upper_val: u64 = upper.parse().unwrap();

    let mut two_part_sum: u64 = 0;
    let mut parts_sum: u64 = 0;
    let mut seen: HashSet<u64> = HashSet::new();
    for part_length in 1..=((upper_val.ilog10() + 1) / 2) {
        for part in (10_u64.pow(part_length - 1))..(10_u64.pow(part_length)) {
            for part_count in
                ((lower_val.ilog10() + 1) / part_length)..=((upper_val.ilog10() + 1) / part_length)
            {
                if part_count == 1 {
                    continue;
                };
                let full_value: u64 = (0..part_count)
                    .map(|val| 10_u64.pow(val * part_length) * part)
                    .sum();
                if full_value > upper_val {
                    break;
                };
                if full_value < lower_val {
                    continue;
                };
                if part_count == 2 {
                    two_part_sum += full_value;
                };
                if seen.contains(&full_value) {
                    continue;
                };
                seen.insert(full_value);
                parts_sum += full_value;
            }
        }
    }
    return (two_part_sum, parts_sum);
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let p1: u64;
    let p2: u64;

    // We iterate through possible faulty IDs, to save ourselves the headache of iterating through
    // every single number in a range. Giving us a headache with a quicker algorithm instead

    (p1, p2) = input
        .lines()
        .map(|line| line)
        .next()
        .unwrap()
        .split(',')
        .fold((0, 0), |(p1acc, p2acc), part| {
            let (lower, upper) = part.split_once('-').unwrap();
            let vals = compare_faulties(lower, upper);
            (p1acc + vals.0, p2acc + vals.1)
        });

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
