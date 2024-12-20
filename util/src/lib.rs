use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn add(&self, other: &Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn dist(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.x)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Hash for Point {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x, self.y).hash(state);
    }
}

// Wrong for MinHeap
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.x, other.y).cmp(&(self.x, self.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        (other.x, other.y).partial_cmp(&(self.x, self.y))
    }
}

pub const N: Point = Point { x: 0, y: -1 };
pub const NE: Point = Point { x: 1, y: -1 };
pub const E: Point = Point { x: 1, y: 0 };
pub const SE: Point = Point { x: 1, y: 1 };
pub const S: Point = Point { x: 0, y: 1 };
pub const SW: Point = Point { x: -1, y: 1 };
pub const W: Point = Point { x: -1, y: 0 };
pub const NW: Point = Point { x: -1, y: -1 };

pub const CARDINALS: [Point; 4] = [N, E, S, W];
pub const DIRECTIONS: [Point; 8] = [N, NE, E, SE, S, SW, W, NW];

pub struct Grid<T> {
    pub grid: Vec<T>,
    pub height: usize,
    pub width: usize,
}

pub struct GridIntoIterator<T> {
    grid: Grid<T>,
    index: Point,
}

impl Grid<u8> {
    pub fn parse(input: String) -> Self {
        let grid = input
            .lines()
            .map(|line| line.bytes().collect_vec())
            .collect_vec();
        Grid {
            height: grid.len(),
            width: grid[0].len(),
            grid: grid.into_iter().flatten().collect_vec(),
        }
    }
}

impl<T: std::clone::Clone> Grid<T> {
    pub fn from_dimensions(default: T, height: usize, width: usize) -> Self {
        Grid {
            grid: vec![default; height * width],
            height,
            width,
        }
    }
}

impl<T> Grid<T> {
    pub fn to_point(&self, ix: usize) -> Point {
        Point {
            y: ix as i32 / self.width as i32,
            x: (ix % self.width) as i32,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }
}

impl<T: Copy> Grid<T> {
    pub fn columns(&self) -> Vec<Vec<T>> {
        (0..self.width)
            .map(|column_ix| {
                (0..self.height)
                    .map(|row_ix| {
                        self[Point {
                            x: column_ix as i32,
                            y: row_ix as i32,
                        }]
                    })
                    .collect_vec()
            })
            .collect_vec()
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.y as usize * self.width + index.x as usize]
    }
}
impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.grid[index.y as usize * self.width + index.x as usize]
    }
}

#[derive(Eq, Clone, Copy, PartialEq)]
pub struct StateWithScore<T> {
    f_score: u32,
    state: T,
}

impl<T: Eq> Ord for StateWithScore<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl<T: PartialEq> PartialOrd for StateWithScore<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

fn reconstruct_path<T: Hash + Eq + Copy + Display>(
    came_from: &HashMap<T, T>,
    current: T,
) -> Vec<T> {
    let mut total_path = Vec::new();
    let mut current = current;
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path
}

pub fn a_star<T: Hash + Eq + Copy + Display>(
    start: T,
    h: &dyn Fn(T) -> u32,
    get_neighbors: &dyn Fn(T) -> Vec<T>,
    is_end: &dyn Fn(T) -> bool,
    number: Option<u32>,
    is_valid: &dyn Fn(&Vec<T>) -> bool,
) -> Vec<Vec<T>> {
    let mut open_set: BinaryHeap<StateWithScore<T>> = BinaryHeap::new(); // Min-heap since we wrap values in Reverse

    let mut came_from: HashMap<T, T> = HashMap::new();

    let mut g_score: HashMap<T, u32> = HashMap::new();

    let start_with_score = StateWithScore {
        state: start,
        f_score: h(start),
    };
    open_set.push(start_with_score);
    g_score.insert(start, 0);

    let mut paths = Vec::new();

    let mut count = 0;

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().state; // Lowest f-score state
        println!("Current: {}", current);
        if is_end(current) {
            let path = reconstruct_path(&came_from, current);
            if is_valid(&path) {
                println!("Found valid path");
                count += 1;
                paths.push(path);
                if let Some(number) = number {
                    println!("Found a number value");
                    if count == number {
                        return paths;
                    }
                } else {
                    println!("Found no number value");
                }

            }
        }

        for neighbor in get_neighbors(current).iter() {
            let tentative_g_score = g_score[&current] + 1;
            if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                open_set.push(StateWithScore {
                    f_score: tentative_g_score + h(*neighbor),
                    state: *neighbor,
                });
            }
        }
    }
    return paths;
}

pub fn load_input(test: bool) -> String {
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
