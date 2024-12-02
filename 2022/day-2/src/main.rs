use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let loser_lookup: HashMap<&str, &str> = HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")]);
    let winner_lookup: HashMap<&str, &str> = HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")]);
    let score_lookup: HashMap<&str, usize> = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let plays = line.split_whitespace().map(|s| s.trim()).collect::<Vec<_>>();
        match plays[1] {
            "X" => { p2 += score_lookup.get(loser_lookup.get(plays[0]).unwrap()).unwrap() },
            "Y" => { p2 +=  4 + (plays[0].bytes().next().unwrap() as usize - b'A' as usize) },
            "Z" => { p2 += 6 + score_lookup.get(winner_lookup.get(plays[0]).unwrap()).unwrap() },
            _ => !unreachable!("NO"),
        }

        if plays[0].bytes().next().unwrap() == plays[1].bytes().next().unwrap() - 23 {
            p1 += 3 + score_lookup.get(plays[1]).unwrap();
            continue;
        }
        if &plays[1] == winner_lookup.get(&plays[0]).unwrap() {
            p1 += 6 + score_lookup.get(plays[1]).unwrap();
            continue;
        }
        p1 += score_lookup.get(plays[1]).unwrap();

    }

    println!("p1: {p1}\np2: {p2}");

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
