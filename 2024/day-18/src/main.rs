use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
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

const N: Point = Point { x: 0, y: -1 };
const NE: Point = Point { x: 1, y: -1 };
const E: Point = Point { x: 1, y: 0 };
const SE: Point = Point { x: 1, y: 1 };
const S: Point = Point { x: 0, y: 1 };
const SW: Point = Point { x: -1, y: 1 };
const W: Point = Point { x: -1, y: 0 };
const NW: Point = Point { x: -1, y: -1 };

const CARDINALS: [Point; 4] = [N, E, S, W];
const DIRECTIONS: [Point; 8] = [N, NE, E, SE, S, SW, W, NW];

struct Grid<T> {
    grid: Vec<T>,
    height: usize,
    width: usize,
}

struct GridIntoIterator<T> {
    grid: Grid<T>,
    index: Point,
}

impl Grid<u8> {
    fn parse(input: String) -> Self {
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
    fn from_dimensions(default: T, height: usize, width: usize) -> Self {
        Grid {
            grid: vec![default; height * width],
            height,
            width,
        }
    }
}

impl<T> Grid<T> {
    fn to_point(&self, ix: usize) -> Point {
        Point {
            y: ix as i32 / self.width as i32,
            x: (ix % self.width) as i32,
        }
    }

    fn in_grid(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }
}

impl<T: Copy> Grid<T> {
    fn columns(&self) -> Vec<Vec<T>> {
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
struct State {
    point: Point,
}

#[derive(Eq, Clone, Copy, PartialEq)]
struct StateWithScore {
    f_score: u32,
    state: State,
}

impl Ord for StateWithScore {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for StateWithScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Hash for State {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.point.x, self.point.y).hash(state);
    }
}

fn get_path_score(came_from: &HashMap<State, State>, current: State) -> u32 {
    let mut score = 0;
    let mut current = current;
    while came_from.contains_key(&current) {
        let next = came_from[&current];
        score += 1;
        current = next;
    }
    score
}

fn print_path(came_from: &HashMap<State, State>, current: State, grid: &Grid<bool>) {
    let mut current = current;
    let mut path = HashSet::new();
    path.insert(current.point);
    while came_from.contains_key(&current) {
        let next = came_from[&current];
        path.insert(next.point);
        current = next;
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            print!(
                "{}",
                if path.contains(&point) {
                    "o"
                } else if grid[point] {
                    "#"
                } else {
                    "."
                }
            );
        }
        println!();
    }
    println!();
}

fn a_star(start: Point, end: Point, grid: &Grid<bool>, h: &dyn Fn(Point) -> u32) -> u32 {
    let mut open_set: BinaryHeap<StateWithScore> = BinaryHeap::new(); // Min-heap since we wrap values in Reverse

    let mut came_from: HashMap<State, State> = HashMap::new();

    let mut g_score: HashMap<State, u32> = HashMap::new();

    let start_state = State { point: start };
    let start_state_with_score = StateWithScore {
        state: start_state,
        f_score: h(start),
    };
    open_set.push(start_state_with_score);
    g_score.insert(start_state, 0);

    while !open_set.is_empty() {
        let current = open_set.pop().unwrap().state; // Lowest f-score state
                                                     // print_path(&came_from, current, grid);
        if current.point == end {
            return get_path_score(&came_from, current);
        }

        for neighbor in CARDINALS.iter().filter_map(|direction| {
            let point = current.point.add(direction);
            if grid.in_grid(point) && !grid[point] {
                return Some(State {
                    point: current.point.add(direction),
                });
            } else {
                None
            }
        }) {
            let tentative_g_score = g_score[&current] + 1;
            if !g_score.contains_key(&neighbor) || tentative_g_score < g_score[&neighbor] {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(StateWithScore {
                    f_score: tentative_g_score + h(neighbor.point),
                    state: neighbor,
                });
            }
        }
    }
    return 0;
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = Point { x: -1, y: -1 };

    let side_len = if test { 6 + 1 } else { 70 + 1 };

    let mut grid = Grid::from_dimensions(false, side_len, side_len);

    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: side_len as i32 - 1,
        y: side_len as i32 - 1,
    };
    let fallen = if test { 12 } else { 1024 };

    let h = |current: Point| current.x.abs_diff(end.x) + current.y.abs_diff(end.y);

    for (ix, line) in input.lines().enumerate() {
        let (x, y) = line
            .split(",")
            .map(|n| n.parse::<i32>().unwrap())
            .collect_tuple::<(i32, i32)>()
            .unwrap();
        let point = Point { x, y };
        if grid.in_grid(point) {
            grid[point] = true;
        } else {
            unreachable!("Not in grid ({},{})", point.x, point.y)
        }
        if ix + 1 == fallen {
            p1 += a_star(start, end, &grid, &h);
        } else if ix + 1 > fallen && a_star(start, end, &grid, &h) == 0 {
            p2 = point;
            break;
        }
    }

    println!("p1: {}\np2: {},{}", p1, p2.x, p2.y);

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
