use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Index<Point> for Grid {
    type Output = bool;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.y as usize][index.x as usize]
    }
}

fn gcd(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn main() -> Result<()> {
    let test = false;
    
    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut antennae: HashMap<u8, Vec<Vec<usize>>> = HashMap::new();

    let input_vector = input.lines().collect_vec();
    let dimesnsions = vec![input_vector[0].trim().len(), input.len()];

    for (y, line) in input.lines().enumerate() {
        for (x, frequency) in line.trim().bytes().enumerate() {
            if frequency == b'.' {
                continue;
            }
            antennae.entry(frequency).or_insert_with(Vec::new).push(vec![x,y]);
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    let mut p2_antinodes: HashSet<Point> = HashSet::new();

    for locations in antennae.values() {
        for pair in locations.iter().combinations(2) {
            let translation = (0..2)
                .map(|ix| pair[1][ix] as isize - pair[0][ix] as isize)
                .collect_vec();
            let gcd = gcd(translation[0], translation[1]);
            let shortest_translation = [translation[0] / gcd, translation[1] / gcd];
        }
    }
    p1 += antinodes.len();
    p2 += p2_antinodes.len();

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
