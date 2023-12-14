use std::env;
use std::fs::read_to_string;
use std::io::Result;

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
            map,
            start: (start_row, start_column),
            location: (start_row, start_column),
        }
    }

    pub fn go(&mut self, direction: char) {
        // println!(
        //     "Trying to go {:?} from location {:?}",
        //     direction, self.location
        // );
        let (mut row, mut column) = self.location;
        match direction {
            'n' => row -= 1,
            'e' => column += 1,
            'w' => column -= 1,
            's' => row += 1,
            _ => unreachable!("Unrecognized direction."),
        };
        self.location = (row, column);
        // println!("Ended up at location {:?}", self.location);
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
    pub fn from_direction(&self, exits: (char, char)) -> char {
        match exits {
            ('n', 's') | ('s', 'n') => '|',
            ('e', 'w') | ('w', 'e') => '-',
            ('n', 'e') | ('e', 'n') => 'L',
            ('n', 'w') | ('w', 'n') => 'J',
            ('s', 'w') | ('w', 's') => '7',
            ('s', 'e') | ('e', 's') => 'F',
            _ => unreachable!("Found invalid exits"),
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

    pub fn get_next(&self, previous_direction: &char) -> char {
        let paths = self.to_direction(&self.get_pipe());
        let entry = self.opposite_direction(previous_direction);
        if paths.0 == entry {
            paths.1
        } else {
            paths.0
        }
    }

    pub fn get_valid_directions(&mut self) -> Vec<char> {
        let mut directions = Vec::new();
        let (row, column) = self.start;
        if row != 0 {
            directions.push('n');
        }
        if row != self.map.len() {
            directions.push('s');
        }
        if column != 0 {
            directions.push('w');
        }
        if column != self.map[0].len() {
            directions.push('e');
        }
        let mut valid_directions = Vec::new();
        for direction in directions {
            println!("Is {:?} a valid direction?", direction);
            self.go(direction);
            if self.get_pipe() == '.' {
                self.location = self.start;
                continue;
            }
            if self.to_direction(&self.get_pipe()).0 == self.opposite_direction(&direction)
                || self.to_direction(&self.get_pipe()).1 == self.opposite_direction(&direction)
            {
                println!("Success");
                valid_directions.push(direction);
            }
            self.location = self.start;
        }
        if valid_directions.len() != 2 {
            panic!("Failed valid directions");
        }
        valid_directions
    }

    pub fn traverse(&mut self, initial_direction: char) -> Option<usize> {
        println!("Traversing...");
        println!("Starting at {:?}", self.start);
        println!("First direction {:?}", initial_direction);
        let mut pipe_loop = PipeLoop::from(&self.map);
        self.go(initial_direction);
        let mut previous_direction = initial_direction;
        let mut next_direction = self.get_next(&previous_direction);
        let mut loop_length = 0;
        pipe_loop.add(self.location, self.get_pipe());
        while !self.location.eq(&self.start) {
            let current_direction = next_direction;
            self.go(current_direction);
            if self.get_pipe().eq(&'.') {
                return None;
            }
            next_direction = self.get_next(&current_direction);
            previous_direction = current_direction;
            loop_length += 1;
            pipe_loop.add(self.location, self.get_pipe());
        }
        pipe_loop.add(
            self.location,
            self.from_direction((
                initial_direction,
                self.opposite_direction(&previous_direction),
            )),
        );
        println!("{:?}", loop_length);
        for line in pipe_loop.map.clone() {
            println!(
                "{:?}",
                line.iter().map(|c| c.to_string()).collect::<String>()
            );
        }
        let count = pipe_loop.count();
        println!("inner: {:?}", count);
        Some(loop_length / 2 + loop_length % 2)
    }
}

struct PipeLoop {
    map: Vec<Vec<char>>,
}

impl PipeLoop {
    pub fn from(map: &Vec<Vec<char>>) -> Self {
        Self {
            map: vec![vec!['.'; map[0].len()]; map.len()],
        }
    }

    pub fn add(&mut self, (row, column): (usize, usize), c: char) {
        self.map[row][column] = c;
    }

    pub fn count(&self) -> i32 {
        let mut inner = 0;
        for line in self.map.iter() {
            let mut inside = false;
            for value in line {
                if value == &'|' {
                    inside = !inside
                } else if value == &'.' {
                    inner += 1
                }
            }
        }
        inner
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut map = PipeMap::from(input);
    println!("Start: {:?}", map.start);

    let path: Vec<(usize, usize)> = Vec::new();

    let valid_directions = map.get_valid_directions();
    for direction in valid_directions {
        if let Some(ans) = map.traverse(direction) {
            p1 = ans as i64;
            break;
        }
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
