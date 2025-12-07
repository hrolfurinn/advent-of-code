use itertools::Itertools;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use util::*;

fn main() {
    let test = true;
    let input = load_input(test);

    let grid = Grid::parse(input);

    let start = grid.to_point(grid.grid.iter().position(|&v| v == b'S').unwrap());
    let end = grid.to_point(grid.grid.iter().position(|&v| v == b'E').unwrap());

    println!("Start {}", start);
    println!("End {}", end);

    let h = |state: Point| state.dist(&end);
    let d = |current: &Point, neighbor: &Point| current.dist(&neighbor);

    let get_neighbors = |point: Point| {
        CARDINALS
            .iter()
            .filter_map(|direction| {
                let neighbor = point.add(&direction);
                if !grid.contains(neighbor) || grid[neighbor] == b'#' {
                    None
                } else {
                    Some(neighbor)
                }
            })
            .collect_vec()
    };

    let is_end = |state: Point| state == end;

    let is_valid = |_: &Vec<Point>| true;

    let default = a_star(start, &h, &d, &get_neighbors, &is_end, &is_valid)[0].len();

    println!("Total moves without teleporting: {}", default);

    let mut depth = 0;
    let mut frontier: Vec<(Point, Option<Point>, Option<Point>)> = vec![(start, None, None)];

    let get_neighbors =
        |point: &Point, first_cheat: &Option<Point>, second_cheat: &Option<Point>| {
            CARDINALS
                .iter()
                .filter_map(|direction| {
                    let neighbor = point.add(&direction);
                    if !grid.contains(neighbor) {
                        None
                    } else if grid[neighbor] == b'#' {
                        if first_cheat.is_none() {
                            Some((neighbor, Some(neighbor), None))
                        } else if second_cheat.is_none() {
                            Some((neighbor, *first_cheat, Some(neighbor)))
                        } else {
                            None
                        }
                    } else {
                        if first_cheat.is_some() && second_cheat.is_none() {
                            Some((neighbor, *first_cheat, Some(neighbor)))
                        } else {
                            Some((neighbor, *first_cheat, *second_cheat))
                        }
                    }
                })
                .collect_vec()
        };

    let mut cheats = Grid::from_dimensions(HashSet::new(),grid.height,grid.width);
    let mut visited: HashSet<(Point, Option<Point>, Option<Point>)> = HashSet::new();

    while depth + if test { 50 } else { 100 } < default && !frontier.is_empty() {
        let mut new_frontier = Vec::new();
        for (point, first_cheat, second_cheat) in frontier.iter() {
            println!("In point {}", point);
            for (n_point, n_first_cheat, n_second_cheat) in
                get_neighbors(point, first_cheat, second_cheat)
            {
                if let Some(first{
                } else if is_end(n_point) {
                    cheats.insert((n_first_cheat,n_second_cheat));
                } else if !visited.contains(&(n_point,n_first_cheat,n_second_cheat)) {
                    println!("Not in visited");
                    visited.insert((n_point,n_first_cheat,n_second_cheat));
                    new_frontier.push((n_point,n_first_cheat,n_second_cheat));
                }
            }
        }
        frontier = new_frontier;
    }

    let p1 = cheats.iter().filter(|(first, _)| first.is_some()).collect_vec().len();

    let mut cheats_iter = cheats.clone().into_iter();

    while let Some((Some(first), Some(second))) = cheats_iter.next() {
        println!("Have cheat {}, {}", first, second);
    }

    println!("{}",p1);
}
