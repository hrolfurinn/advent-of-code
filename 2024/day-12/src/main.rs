use itertools::Itertools;
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

const N: Point = Point { x: 0, y: 1 };
const NE: Point = Point { x: 1, y: 1 };
const E: Point = Point { x: 1, y: 0 };
const SE: Point = Point { x: 1, y: -1 };
const S: Point = Point { x: 0, y: -1 };
const SW: Point = Point { x: -1, y: -1 };
const W: Point = Point { x: -1, y: 0 };
const NW: Point = Point { x: -1, y: 1 };

const CARDINALS: [Point;4] = [N,E,S,W];
const DIRECTIONS: [Point;8] = [N,NE,E,SE,S,SW,W,NW];

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
            x: ix as i32 / self.height as i32,
            y: (ix % self.height) as i32,
        }
    }

    fn in_grid(&self, point: Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.width as i32 && point.y < self.height as i32
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.y as usize * self.height + index.x as usize]
    }
}
impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.grid[index.y as usize * self.height + index.x as usize]
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let grid = Grid::parse(input);


    let mut seen: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Grid<bool>> = Vec::new();
    let mut current_row_values = (0,0);
    let mut current_column_values = (0,0);

    // To find perimeters, we count how often we cross a region perimeter, i.e. change regions.
    // In practice, this means that we iterate through the grid row-wise & column-wise keeping
    // track of the current region and counting changes. Rows give us vertical perimeters, columns
    // give us horizontal ones.
    //
    // To find sides, we count how often we cross corners in the grid, row-wise and column-wise,
    // keeping track of two rows/columns at a time.
    //
    // The grid is square, so we can iterate through rows and columns simultaneously.
    for (ix, value) in grid.grid.iter().enumerate() {
        let rpoint = grid.to_point(ix); // Iterator through the rows
        if rpoint.x == 0 { 
            current_row_values = (0,0);
            current_column_values = (0,0);
        }
        let cpoint = Point { x: rpoint.y, y: rpoint.x }; 
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
