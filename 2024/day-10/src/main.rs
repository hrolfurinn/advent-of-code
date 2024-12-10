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

impl Point {
    fn add(&self, other: &Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Grid {
    grid: Vec<usize>,
    height: usize,
    width: usize,
}

impl Grid {
    fn parse(input: String) -> Self {
        let grid = input.lines().map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as usize)
                .collect_vec()
        }).collect_vec();
        Grid {
            height: grid.len(),
            width: grid[0].len(),
            grid: grid.into_iter().flatten().collect_vec(),
        }
    }

    fn to_point(&self, ix: usize) -> Point {
        Point {
            x: ix as i32 / self.height as i32,
            y: (ix % self.height) as i32,
        }
    }

    fn in_grid(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }
}

impl Index<Point> for Grid {
    type Output = usize;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.y as usize * self.height + index.x as usize]
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let grid = Grid::parse(input);

    let directions = [Point { x: 0, y: 1 }, Point { x: 1, y: 0 }, Point { x: 0, y: -1 }, Point { x: -1, y: 0 }];

    for ix in 0..grid.grid.len() {
        let trailhead = grid.to_point(ix);
        if grid[trailhead] != 0 { continue; };
        let mut curr_points = vec![trailhead];
        let mut summits = HashSet::new();
        loop {
            let mut next_points = Vec::new();
            for point in curr_points.iter() {
                for direction in directions {
                    let next_point = point.add(&direction);
                    if !grid.in_grid(next_point) || grid[next_point] != grid[*point] + 1 {
                    } else if grid[next_point] == 9 {
                        summits.insert((next_point.x, next_point.y));
                        p2 += 1;
                    } else {
                        next_points.push(next_point);
                    }
                }
            }
            if next_points.is_empty() { break; } else { curr_points = next_points }
        }
        p1 += summits.len();
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
