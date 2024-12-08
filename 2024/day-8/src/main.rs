use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn from(x: usize, y: usize) -> Self {
        Point{x: x as i32, y: y as i32}
    }

    fn add(&self, dist: &[i32]) -> Self {
        Point {
         x: self.x + dist[0],
         y: self.y + dist[1],
        }
    }

    fn in_grid(&self, height: i32, width: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width && self.y < height
    }
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut antennae: HashMap<u8,Vec<Point>> = HashMap::new();

    let input_vector = input.lines().collect_vec();
    let width = input_vector[0].trim().len() as i32;
    let height = input_vector.len() as i32;

    for (y, line) in input.lines().enumerate() {
        for (x, frequency) in line.trim().bytes().enumerate() {
            if frequency == b'.' { continue; }
            if antennae.contains_key(&frequency) {
                antennae.get_mut(&frequency).unwrap().push(Point::from(x,y));
            } else {
                antennae.insert(frequency,vec![Point::from(x,y)]);
            }
        }
    }

    let mut antinodes = HashSet::new();
    let mut p2_antinodes: HashSet<Point> = HashSet::new();

    for locations in antennae.values() {
        for pair in locations.iter().combinations(2) {
            let first_point = Point{ 
                x: pair[0].x + (2 * (pair[1].x - pair[0].x)), 
                y: pair[0].y + (2 * (pair[1].y - pair[0].y)), 
            };
            let second_point = Point{ 
                x: pair[1].x + (2 * (pair[0].x - pair[1].x)), 
                y: pair[1].y + (2 * (pair[0].y - pair[1].y)), 
            };
            if first_point.in_grid(height,width) {
                antinodes.insert(first_point);
            };
            if second_point.in_grid(height,width) {
                antinodes.insert(second_point);
            };
            let dist = [(pair[0].x - pair[1].x),(pair[0].y - pair[1].y)];
            let gcd = gcd(dist[0],dist[1]);
            let atom_dist = [dist[0] / gcd, dist[1] / gcd];
            let mut new_point = pair[0].clone();
            if new_point.in_grid(height,width) {
                p2_antinodes.insert(new_point.clone());
            }
            while new_point.in_grid(height,width) {
                p2_antinodes.insert(new_point.clone());
                new_point = new_point.add(&atom_dist);
            }
            let atom_dist = [-atom_dist[0], -atom_dist[1]];
            let mut new_point = pair[0].clone();
            while new_point.in_grid(height,width) {
                p2_antinodes.insert(new_point.clone());
                new_point = new_point.add(&atom_dist);
            }
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
