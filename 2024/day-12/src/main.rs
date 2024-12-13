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

    let directions = [
        Point { x: 0, y: 1 },
        Point { x: 1, y: 0 },
        Point { x: 0, y: -1 },
        Point { x: -1, y: 0 },
    ];

    let mut seen: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Grid<bool>> = Vec::new();

    for ix in 0..grid.grid.len() {
        let point = grid.to_point(ix);
        if seen.contains(&point) {
            continue;
        };
        let mut region = Grid::from_dimensions(false, grid.height, grid.width);
        region[point] = true;
        let mut curr_points = vec![point];
        loop {
            let mut next_points = Vec::new();
            for curr_point in curr_points.iter() {
                for direction in directions {
                    let next_point = curr_point.add(&direction);
                    if grid.in_grid(next_point)
                        && !region[next_point]
                        && grid[next_point] == grid[*curr_point]
                    {
                        seen.insert(next_point);
                        region[next_point] = true;
                        next_points.push(next_point);
                    }
                }
            }
            if next_points.is_empty() {
                break;
            } else {
                curr_points = next_points
            }
        }
        regions.push(region);
    }

    // Iterate through the entire grid row-wise and column-wise as each time we switch from a
    // section of the region to a section outside of it we cross a perimeter, and vice-versa
    for region in regions.iter() {
        let mut perimeter = 0;
        let mut area = 0;

        // Row-wise, here we also count the total area
        let mut point = Point { x: 0, y: 0 };
        while region.in_grid(point) {
            let mut current_value = false; // The outside of the grid is not in the region
            let curr_row_start = point;
            while region.in_grid(point) {
                if region[point] {
                    area += 1;
                }
                if region[point] != current_value {
                    perimeter += 1;
                    current_value = !current_value;
                }
                point = point.add(&Point { x: 1, y: 0 });
            }
            if current_value {
                perimeter += 1;
            } // The grid border is a perimeter
            point = curr_row_start.add(&Point { x: 0, y: 1 });
        }

        // Column-wise
        let mut point = Point { x: 0, y: 0 };
        while region.in_grid(point) {
            let curr_column_start = point;
            let mut current_value = false; // The outside of the grid is not in the region
            while region.in_grid(point) {
                if region[point] != current_value {
                    perimeter += 1;
                    current_value = !current_value;
                }
                point = point.add(&Point { x: 0, y: 1 });
            }
            if current_value {
                perimeter += 1;
            } // The grid border is a perimeter
            point = curr_column_start.add(&Point { x: 1, y: 0 });
        }

        p1 += area * perimeter;
        
        // We iterate through the region row-wise and column-wise but two at a time. We count the
        // number of sides by noting that a side is a continuous stretch of of one row/column being
        // outisde the region and the other being inside it
        let mut sides = 0;

        // Row-wise
        let mut lower_point = Point { x: 0, y: -1 };
        let mut upper_point = Point { x: 0, y: 0 };
        while region.in_grid(lower_point) || region.in_grid(upper_point) {
            let mut current_values = (false, false); // (Is lower in region, Is upper in region)
            let curr_lower_row_start = lower_point;
            while region.in_grid(lower_point) || region.in_grid(upper_point) {
                let lower = if region.in_grid(lower_point) { region[lower_point] } else { false };
                let upper = if region.in_grid(upper_point) { region[upper_point] } else { false };
                if (lower,upper) != current_values {
                    if lower ^ upper { sides += 1 };
                    current_values =  (lower, upper);
                }
                lower_point = lower_point.add(&Point { x: 1, y: 0 });
                upper_point = upper_point.add(&Point { x: 1, y: 0 });
            }
            lower_point = curr_lower_row_start.add(&Point { x: 0, y: 1 });
            upper_point = lower_point.add(&Point { x: 0, y: 1});
        }

        // Column-wise
        let mut left_point = Point { x: -1, y: 0 };
        let mut right_point = Point { x: 0, y: 0 };
        while region.in_grid(left_point) || region.in_grid(right_point) {
            let mut current_values = (false, false); // (Is left in region, Is right in region)
            let curr_left_column_start = left_point;
            while region.in_grid(left_point) || region.in_grid(right_point) {
                let left = if region.in_grid(left_point) { region[left_point] } else { false };
                let right = if region.in_grid(right_point) { region[right_point] } else { false };
                if (left,right) != current_values {
                    if left ^ right { sides += 1 };
                    current_values =  (left, right);
                }
                left_point = left_point.add(&Point { x: 0, y: 1 });
                right_point = right_point.add(&Point { x: 0, y: 1 });
            }
            left_point = curr_left_column_start.add(&Point { x: 1, y: 0 });
            right_point = left_point.add(&Point { x: 1, y: 0});
        }
        p2 += area * sides;
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
