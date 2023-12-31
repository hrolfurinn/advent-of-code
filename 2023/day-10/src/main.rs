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
        Self {
            map,
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
        self.location = (row, column);
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
            self.go(direction);
            if self.get_pipe() == '.' {
                self.location = self.start;
                continue;
            }
            if self.to_direction(&self.get_pipe()).0 == self.opposite_direction(&direction)
                || self.to_direction(&self.get_pipe()).1 == self.opposite_direction(&direction)
            {
                valid_directions.push(direction);
            }
            self.location = self.start;
        }
        if valid_directions.len() != 2 {
            unreachable!("Invalid direction count")
        }
        valid_directions
    }

    pub fn traverse(&mut self, initial_direction: char) -> Option<(i64,i64)> {
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
        pipe_loop.count -= 1; // discount the addition of the start square
        pipe_loop.process()
    }
}

struct PipeLoop {
    map: Vec<Vec<char>>,
    count: i64,
}

impl PipeLoop {
    pub fn from(map: &Vec<Vec<char>>) -> Self {
        Self {
            map: vec![vec!['.'; map[0].len()]; map.len()],
            count: 0,
        }
    }

    pub fn add(&mut self, (row, column): (usize, usize), c: char) {
        self.map[row][column] = c;
        self.count += 1;
    }

    pub fn s_corner(&self, corner: char) -> char {
        // corner that makes the loop take an 'S' turn, like F and J below
        // ...|.
        // .F-J.
        // .|...
        // Can only be former symbol, since line is read left to right
        match corner {
            'F' => 'J',
            'L' => '7',
            _ => unreachable!("Invalid corner char {:?}", corner),
        }
    }
    pub fn u_corner(&self, corner: char) -> char {
        // corner that makes the loop take an 'S' turn, like F and 7 below
        // .....
        // .F-7.
        // .|.|.
        // Can only be former symbol, since line is read left to right
        match corner {
            'F' => '7',
            'L' => 'J',
            _ => unreachable!("Invalid corner char {:?}", corner),
        }
    }

    pub fn process(&self) -> Option<(i64,i64)> {
        let mut inner = 0;
        for (row_index, line) in self.map.iter().enumerate() {
            let mut prev_corner = '\0';
            let mut inside = false;
            for (column_index, value) in line.iter().enumerate() {
                match value {
                    '|' => {
                        inside = !inside;
                    }
                    '7' | 'J' => {
                        if prev_corner == '\0' {
                            unreachable!("Previous corner was null")
                        }
                        let s_corner = self.s_corner(prev_corner);
                        let u_corner = self.u_corner(prev_corner);
                        match value {
                            v if v == &s_corner => {
                                inside = !inside;
                            }
                            v if v == &u_corner => prev_corner = '\0',
                            _ => unreachable!(
                                "Invalid corner combo for value {:?} and prev {:?}",
                                value, prev_corner
                            ),
                        }
                        prev_corner = '\0'
                    }
                    'F' | 'L' => {
                        if prev_corner != '\0' {
                            unreachable!(
                                "Previous corner was not null at {:?}: {:?}, {:?}",
                                value, row_index, column_index
                            )
                        }
                        prev_corner = *value
                    }
                    '-' => {
                        if prev_corner == '\0' {
                            unreachable!(
                                "I should not be allowed here at {:?}: {:?}, {:?}",
                                value, row_index, column_index
                            )
                        }
                    }
                    '.' => {
                        if inside {
                            inner += 1;
                        }
                    }
                    _ => unreachable!("Invalid character in loop counting"),
                }
            }
        }
        Some((inner,(self.count / 2) + self.count%2 as i64))
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut map = PipeMap::from(input);

    let path: Vec<(usize, usize)> = Vec::new();

    let valid_directions = map.get_valid_directions();
    for direction in valid_directions {
        if let Some(ans) = map.traverse(direction) {
            (p2,p1) = ans;
            break;
        }
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
