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
        println!("Trying to go {:?} from location {:?}", direction, self.location);
        let (mut row, mut column) = self.location;
        match direction {
            'n' => row -= 1,
            'e' => column += 1,
            'w' => column -= 1,
            's' => row += 1,
            _ => unreachable!("Unrecognized direction."),
        };
        self.location = (row,column);
        println!("Ended up at location {:?}", self.location);
    }

    pub fn get_pipe(&self) -> char {
        let (current_row, current_column) = self.location;
        self.map[current_row][current_column]
    }

    pub fn to_direction(&self, pipe: &char) -> (char, char) {
        match pipe {
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
    
    pub fn opposite_direction(&self, direction: &char) -> char {
        match direction {
            'n' => 's',
            'e' => 'w',
            'w' => 'e',
            's' => 'n',
            _ => unreachable!("Unrecognized direction."),
        }
    }    

    pub fn get_next(&self, pipe: &char, previous_direction: &char) -> char {
        let paths = self.to_direction(pipe);
        let entry = self.opposite_direction(previous_direction);
        if paths.0 == entry {
            paths.1
        } else {
            paths.0
        }
    }

    pub fn traverse(&mut self, initial_direction: char) -> Option<(usize, usize)>{
        println!("Traversing...");
        println!("Starting at {:?}", self.start);
        println!("First direction {:?}", initial_direction);
        self.go(initial_direction);
        let mut previous_direction = initial_direction;
        let mut next_direction = self.get_next(&self.get_pipe(), &previous_direction);
        let mut loop_length = 0;
        while !self.location.eq(&self.start) {
            let current_direction = next_direction;
            self.go(current_direction);
            previous_direction = current_direction;
            next_direction = self.get_next(&self.get_pipe(), &current_direction);
            loop_length += 1;
        }
        println!("{:?}", loop_length);
        None
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let mut map = PipeMap::from(input);
    println!("Start: {:?}", map.start);

    let directions = ['n', 'e', 's', 'w'];
    let path: Vec<(usize, usize)> = Vec::new();

    let first_direction = 'e';
    let _ = map.traverse(first_direction);

    // for direction in directions {}

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
