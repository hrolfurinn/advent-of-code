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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy)]
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
    phantom_visited_points: Vec<Vec<Option<Direction>>>,
    guard: Point,
    original_guard: Point,
    guard_direction: Direction,
    new_obstacles: Vec<Vec<bool>>,
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
        self.phantom_visited_points = vec![vec![None; self.width + 2]; self.height + 2];
        self.new_obstacles = vec![vec![false; self.width + 2]; self.height + 2];
        self.visit(self.guard, self.guard_direction);
    }

    fn visit(&mut self, point: Point, direction: Direction) {
        self.visited_points[point.y][point.x] = Some(direction);
    }

    fn phantom_visit(&mut self, point: Point, direction: Direction) {
        self.phantom_visited_points[point.y][point.x] = Some(direction);
    }

    fn add_obstacle(&mut self, point: Point) {
        let potential_obstacle = point.add(&self.guard_direction);
        if self.is_off_grid(potential_obstacle) {
            return;
        }
        if let Some(visted_direction) = self.visited_points[point.y][point.x] {
            if visted_direction == self.guard_direction.turn_right() {
                self.new_obstacles[potential_obstacle.y][potential_obstacle.x] = true;
                return;
            }
        }
        if let Some(visted_direction) = self.phantom_visited_points[point.y][point.x] {
            if visted_direction == self.guard_direction.turn_left() {
                self.new_obstacles[potential_obstacle.y][potential_obstacle.x] = true;
                return;
            }
        }
    }

    fn has_obstacle(&self, point: &Point) -> bool {
        self.obstacles[point.y][point.x]
    }

    fn is_off_grid(&self, point: Point) -> bool {
        let banned_coords = [0, self.width + 1, self.height + 1];
        banned_coords.contains(&point.x) || banned_coords.contains(&point.y)
    }

    fn guard_step(&mut self) -> bool {
        let new_point = self.guard.add(&self.guard_direction);
        if self.is_off_grid(new_point) {
            return false;
        }
        if !self.has_obstacle(&new_point) {
            self.add_obstacle(new_point);
            self.guard = new_point;
            return true;
        }
        self.guard_direction = self.guard_direction.turn_right();
        self.guard_step()
    }

    fn phantom_guard_step(&mut self) -> bool {
        let new_point = self.guard.add(&self.guard_direction);
        if self.is_off_grid(new_point) {
            return false;
        }
        if !self.has_obstacle(&new_point) {
            self.guard = new_point;
            return true;
        }
        self.guard_direction = self.guard_direction.turn_left();
        self.guard_step()
    }
}

fn main() -> Result<()> {
    let test = true;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut grid = Grid {
        width: 0,
        height: 0,
        obstacles: Vec::new(),
        visited_points: Vec::new(),
        phantom_visited_points: Vec::new(),
        guard: Point { x: 0, y: 0 },
        original_guard: Point { x: 0, y: 0 },
        guard_direction: Direction::Up,
        new_obstacles: Vec::new(),
    };


    grid.from(&input);

    println!("Guard is at {} {}", grid.guard.x, grid.guard.y);
    grid.guard_direction = Direction::Down;

    while grid.phantom_guard_step() {
        grid.phantom_visit(grid.guard, grid.guard_direction);
    }

    grid.guard = grid.original_guard;
    grid.guard_direction = Direction::Up;

    while grid.guard_step() {
        grid.visit(grid.guard, grid.guard_direction);
    }

    p1 += grid
        .visited_points
        .iter()
        .map(|v| v.iter().map(|&d| if d.is_some() { 1 } else { 0 }).sum::<usize>())
        .sum::<usize>();
    for (y, row) in grid.new_obstacles.iter().enumerate() {
        for (x, &d) in row.iter().enumerate() {
            if d {
            println!("{x} {y}");
            }
        }
    }
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
