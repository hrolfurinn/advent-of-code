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

fn get_cracked_grid(grid: &Grid<u8>) -> Grid<u8> {
    let mut cracked_grid = Vec::new();
    for column in grid.columns() {
        let mut new_column = Vec::new();
        for value in column.iter() {
            new_column.push(match value {
                b'#' => b'#',
                b'O' => b']',
                b'.' | b'@' => b'.',
                _ => unreachable!("Failed new grid"),
            });
        }
        cracked_grid.push(
            column
                .iter()
                .map(|&c| if c == b'O' { b'[' } else { c })
                .collect_vec(),
        );
        cracked_grid.push(new_column);
    }
    cracked_grid = (0..cracked_grid[0].len())
        .map(|row_ix| {
            (0..cracked_grid.len())
                .map(|col_ix| cracked_grid[col_ix][row_ix])
                .collect_vec()
        })
        .collect_vec();
    Grid {
        width: cracked_grid[0].len(),
        height: cracked_grid.len(),
        grid: cracked_grid.iter().flatten().map(|&v| v).collect_vec(),
    }
}

fn get_other_half(val: &u8) -> Point {
    match val {
        b'[' => E,
        b']' => W,
        _ => unreachable!("Failed to get other half of block"),
    }
}

fn get_first_side(direction: Point) -> u8 {
    match direction {
        W => b']',
        E => b'[',
        _ => unreachable!("Non-horizontal side")
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let (map_str, instructions) = input.split_once("\n\n").unwrap();

    let mut grid = Grid::parse(map_str.to_string());
    let cracked_grid = get_cracked_grid(&grid);
    let mut robot = grid.to_point(grid.grid.iter().find_position(|&&c| c == b'@').unwrap().0);

    for instruction in instructions.chars().filter_map(|c| match c {
        '^' => Some(N),
        '>' => Some(E),
        'v' => Some(S),
        '<' => Some(W),
        _ => None,
    }) {
        let attempted_square = robot.add(&instruction);
        if grid[attempted_square] == b'.' {
            grid[attempted_square] = b'@';
            grid[robot] = b'.';
            robot = attempted_square;
        } else if grid[attempted_square] == b'#' {
        } else {
            let mut last_block = attempted_square;
            while grid[last_block] == b'O' {
                last_block = last_block.add(&instruction);
            }
            if grid[last_block] != b'#' {
                grid[robot] = b'.';
                grid[attempted_square] = b'@';
                grid[last_block] = b'O';
                robot = attempted_square;
            }
        }
    }

    for (point, val) in grid
        .grid
        .iter()
        .enumerate()
        .map(|(ix, val)| (grid.to_point(ix), val))
    {
        if val != &b'O' {
            continue;
        }
        p1 += (point.y * 100) + point.x;
    }

    // Part 2
    let mut grid = cracked_grid;
    let mut robot = grid.to_point(grid.grid.iter().find_position(|&&c| c == b'@').unwrap().0);

    for instruction in instructions.chars().filter_map(|c| match c {
        '^' => Some(N),
        '>' => Some(E),
        'v' => Some(S),
        '<' => Some(W),
        _ => None,
    }) {
        let attempted_square = robot.add(&instruction);
        if grid[attempted_square] == b'.' {
            grid[attempted_square] = b'@';
            grid[robot] = b'.';
            robot = attempted_square;
        } else if grid[attempted_square] == b'#' {
        } else if instruction == E || instruction == W {
            let mut last_block = attempted_square;
            let mut to_move = vec![robot];
            while grid[last_block] == get_first_side(instruction) {
                to_move.push(last_block);
                to_move.push(last_block.add(&instruction));
                last_block = last_block.add(&instruction).add(&instruction);
            }
            if grid[last_block] == b'#' { continue; }
            for point in to_move.iter().rev() {
                grid[point.add(&instruction)] = grid[*point];
                grid[*point] = b'.';
            }
            robot = attempted_square;
        } else {
            let mut frontiers = Vec::new();
            let mut frontier = HashSet::from([
                attempted_square,
                attempted_square.add(&get_other_half(&grid[attempted_square])),
            ]);
            frontiers.push(HashSet::from([robot]));
            frontiers.push(frontier.clone());
            let mut movable = true;
            'outer: while !frontier.is_empty() {
                let mut new_frontier = HashSet::new();
                for square in frontier.iter() {
                    let next = square.add(&instruction);
                    if grid[next] == b'#' {
                        movable = false;
                        break 'outer;
                    } else if grid[next] != b'.' {
                        let next_to_next = next.add(&get_other_half(&grid[next]));
                        new_frontier.insert(next);
                        new_frontier.insert(next_to_next);
                    }
                }
                frontiers.push(new_frontier.clone());
                frontier = new_frontier;
            }
            if movable {
                for frontier in frontiers.iter().rev().map(|set| set.iter()) {
                    for point in frontier {
                        grid[point.add(&instruction)] = grid[*point];
                        grid[*point] = b'.';
                    }
                }
                robot = attempted_square;
            }
        }
    }

    for (point, val) in grid
        .grid
        .iter()
        .enumerate()
        .map(|(ix, val)| (grid.to_point(ix), val))
    {
        if val != &b'[' {
            continue;
        }
        p2 += (100 * point.y) + point.x;
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
