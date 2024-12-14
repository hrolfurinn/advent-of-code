use itertools::Itertools;
use std::cmp::Ordering::*;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut quadrants = vec![0; 4];

    let width = if test { 11 } else { 101 };
    let height = if test { 7 } else { 103 };
    let half_width = width / 2;
    let half_height = height / 2;

    let mut robots = Vec::new();

    for line in input.lines() {
        let (px, py, vx, vy) = line
            .split(|c: char| !c.is_digit(10) && c != '-')
            .filter_map(|num| num.parse::<i32>().ok())
            .collect_tuple()
            .unwrap();
        robots.push(((px, py), (vx, vy)));
        match (
            (px + (100 * vx)).rem_euclid(width).cmp(&half_width),
            (py + (100 * vy)).rem_euclid(height).cmp(&half_height),
        ) {
            (Less, Less) => quadrants[0] += 1,
            (Less, Greater) => quadrants[1] += 1,
            (Greater, Less) => quadrants[2] += 1,
            (Greater, Greater) => quadrants[3] += 1,
            _ => {}
        }
    }

    for ix in 0..(101 * 103) {
        println!("{ix}");
        let mut grid = vec![vec![false; width as usize]; height as usize];
        let mut new_robots = Vec::new();
        for (position, velocity) in robots.iter() {
            grid[position.1 as usize][position.0 as usize] = true;
            new_robots.push((
                (
                    (position.0 + velocity.0).rem_euclid(width),
                    (position.1 + velocity.1).rem_euclid(height),
                ),
                *velocity,
            ));
        }
        if ix % 101 == 79 {
            for row in grid.iter() {
                for spot in row.iter() {
                    print!("{}", if *spot { "X" } else { "." });
                }
                println!();
            }
        }
        robots = new_robots;
        println!("-----------------------------------------------------");
    }

    p1 += quadrants.iter().product::<u32>();

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
