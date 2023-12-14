use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::str::Lines;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        // Ok("".to_string()) => panic!("Input empty"),
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

struct PipeMap {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    location: (usize, usize),
}

impl PipeMap {
    pub fn from(input: String) -> Self {
        let map: Vec<Vec<char>> = input
            .split_whitespace()
            .map(|line| line.chars().collect())
            .collect();
        let (start_row, start_column) = map
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                if row.contains(&'S') {
                    println!("Found S in {:?}", row.iter().collect::<String>());
                    Some((
                        row_index,
                        row.iter()
                            .enumerate()
                            .find_map(|(index, c)| if c.eq(&'S') { Some(index) } else { None })
                            .unwrap(),
                    ))
                } else {
                    None
                }
            })
            .unwrap();
        println!("Start coords: {:?} and {:?}", start_row, start_column);
        Self {
            map: map,
            start: (start_row, start_column),
            location: (start_row, start_column),
        }
    }

    pub fn go(&mut self, direction: char) {
        let (mut row, mut column) = self.location;
        match direction {
            'n' => row -= 1,
            'e' => column += 1,
            'w' => column -= 1,
            's' => row += 1,
            _ => unreachable!("Unrecognized direction."),
        };
        self.location = (row,column);
    }
}

fn to_direction(c: &char) -> (char, char) {
    match c {
        '|' => ('n', 's'),
        '-' => ('e', 'w'),
        'L' => ('n', 'e'),
        'J' => ('n', 'w'),
        '7' => ('s', 'w'),
        'F' => ('s', 'e'),
        '.' => ('.', '.'),
        'S' => ('S', 'S'),
        _ => unreachable!("Found invalid character"),
    }
}

fn opposite_direction(c: &char) -> char {
    match c {
        'n' => 's',
        'e' => 'w',
        'w' => 'e',
        's' => 'n',
        _ => unreachable!("Unrecognized direction."),
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let mut map = PipeMap::from(input);
    println!("Start: {:?}", map.start);
    map.go('s');
    println!("Current: {:?}", map.location);

    let directions = ['n', 'e', 's', 'w'];
    let path: Vec<(usize, usize)> = Vec::new();

    for direction in directions {}

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
