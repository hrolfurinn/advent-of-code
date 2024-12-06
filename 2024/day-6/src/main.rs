use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};

// Todo:
// Make a function "Attempt to place obstacle" which checks whether a candidate obstacle position
// is valid, i.e. neither original_guard nor out of bounds.
//
// For points visited on the path, store them as directions to make sure that we aren't counting
// instances where the guard crosses previous path sections which couldn't have a new obstacle
// placed.

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn add(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Grid {
    // Padded at all sides
    width: usize,
    height: usize,
    obstacles: Vec<Vec<bool>>,
    visited_points: Vec<Vec<Option<Direction>>>,
    phantom_visited_points: Vec<Vec<HashSet<Direction>>>,
    original_guard: Point,
    guard: Point,
    guard_direction: Direction,
    phantom_guard: Point,
    phantom_guard_direction: Direction,
    new_obstacles: Vec<Vec<bool>>,
    tried_obstacles: Vec<Vec<bool>>,
}

impl Grid {
    fn from(&mut self, input: &str) {
        for (y, line) in input.lines().enumerate() {
            if self.obstacles.is_empty() {
                self.width = line.len();
                self.obstacles.push(vec![false; self.width + 2]);
            }
            if let Some(x) = line.find("^") {
                self.guard = Point { x: x + 1, y: y + 1 };
                self.phantom_guard = Point { x: x + 1, y: y + 1 };
                self.original_guard = Point { x: x + 1, y: y + 1 };
            }
            let mut row = vec![false; 1];
            row.extend(line.trim().bytes().map(|byte| byte == b'#').collect_vec());
            row.push(false);
            self.obstacles.push(row);
        }
        self.obstacles.push(vec![false; self.width + 2]);
        self.height = self.obstacles.len();
        self.visited_points = vec![vec![None; self.width + 2]; self.height + 2];
        self.phantom_visited_points = vec![vec![HashSet::new(); self.width + 2]; self.height + 2];
        self.new_obstacles = vec![vec![false; self.width + 2]; self.height + 2];
        self.tried_obstacles = vec![vec![false; self.width + 2]; self.height + 2];
        self.visit();
    }

    fn visit(&mut self) {
        self.visited_points[self.guard.y][self.guard.x] = Some(self.guard_direction);
    }

    fn is_visted(&self) -> bool {
        if let Some(visited_direction) = self.visited_points[self.guard.y][self.guard.x] {
            if visited_direction == self.guard_direction {
                return true;
            }
        }
        false
    }

    fn phantom_visit(&mut self) {
        self.phantom_visited_points[self.phantom_guard.y][self.phantom_guard.x]
            .insert(self.phantom_guard_direction);
    }

    fn is_phantom_visited(&self) -> bool {
        self.phantom_visited_points[self.phantom_guard.y][self.phantom_guard.x].contains(&self.phantom_guard_direction)
    }

    fn is_tried(&self, point: Point) -> bool {
        self.tried_obstacles[point.y][point.x]
    }

    fn add_obstacle(&mut self, new_point: Point) {
        if self.original_guard == new_point {
            return;
        };
        if self.is_tried(new_point) { return };
        self.tried_obstacles[new_point.y][new_point.x] = true;
        // We know the point is not off the grid and not at previous obstacle location
        self.phantom_guard = self.guard;
        self.phantom_guard_direction = self.guard_direction.turn_right();
        self.phantom_visited_points = vec![vec![HashSet::new(); self.width + 2]; self.height + 2];
        if self.phantom_traverse(new_point) {
            self.new_obstacles[new_point.y][new_point.x] = true;
        }
    }

    fn has_obstacle(&self, point: &Point) -> bool {
        self.obstacles[point.y][point.x]
    }

    fn is_off_grid(&self, point: Point) -> bool {
        let banned_coords = [0, self.width + 1, self.height + 1];
        banned_coords.contains(&point.x) || banned_coords.contains(&point.y)
    }

    fn guard_traverse(&mut self) {
        self.visit();
        let new_point = self.guard.add(&self.guard_direction);
        if self.is_off_grid(new_point) {
            return;
        }
        if !self.has_obstacle(&new_point) {
            self.add_obstacle(new_point);
            self.guard = new_point;
            return self.guard_traverse();
        }
        self.guard_direction = self.guard_direction.turn_right();
        self.guard_traverse()
    }

    fn phantom_traverse(&mut self, phantom_obstacle: Point) -> bool {
        if self.is_phantom_visited() {
            return true;
        }
        self.phantom_visit();
        let new_point = self.phantom_guard.add(&self.phantom_guard_direction);
        if self.is_off_grid(new_point) {
            return false;
        }
        if !self.has_obstacle(&new_point) && !(phantom_obstacle == new_point) {
            self.phantom_guard = new_point;
            return self.phantom_traverse(phantom_obstacle);
        }
        self.phantom_guard_direction = self.phantom_guard_direction.turn_right();
        self.phantom_traverse(phantom_obstacle)
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut grid = Grid {
        width: 0,
        height: 0,
        obstacles: Vec::new(),
        visited_points: Vec::new(),
        phantom_visited_points: Vec::new(),
        original_guard: Point { x: 0, y: 0 },
        guard: Point { x: 0, y: 0 },
        guard_direction: Direction::Up,
        phantom_guard: Point { x: 0, y: 0 },
        phantom_guard_direction: Direction::Up,
        new_obstacles: Vec::new(),
        tried_obstacles: Vec::new(),
    };

    grid.from(&input);

    grid.guard_traverse();

    p1 += grid
        .visited_points
        .iter()
        .map(|v| {
            v.iter()
                .map(|&d| if d.is_some() { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum::<usize>();
    p2 += grid
        .new_obstacles
        .iter()
        .map(|v| v.iter().map(|&b| if b { 1 } else { 0 }).sum::<usize>())
        .sum::<usize>();

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
