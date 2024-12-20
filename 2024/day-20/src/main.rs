use itertools::Itertools;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use util::*;

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    point: Point,
    moves_left: u8,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.point, self.moves_left).hash(state)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} w/ {} left", self.point, self.moves_left)
    }
}

fn main() {
    let test = true;
    let input = load_input(test);

    let grid = Grid::parse(input);

    let start = grid.to_point(grid.grid.iter().position(|&v| v == b'S').unwrap());
    let end = grid.to_point(grid.grid.iter().position(|&v| v == b'E').unwrap());

    println!("Start {}", start);
    println!("End {}", end);

    let h = |state: State| state.point.dist(&end);

    let neighbors = |state: State| {
        CARDINALS
            .iter()
            .filter_map(|direction| {
                let neighbor = state.point.add(&direction);
                if !grid.contains(neighbor) {
                    return None;
                } else if grid[neighbor] == b'#' {
                    if state.moves_left == 0 {
                        return None;
                    } else {
                        return Some(State {
                            point: neighbor,
                            moves_left: state.moves_left - 1,
                        });
                    }
                } else {
                    return Some(State {
                        point: neighbor,
                        moves_left: if state.moves_left == 1 {
                            0
                        } else {
                            state.moves_left
                        },
                    });
                }
            })
            .collect_vec()
    };

    let is_end = |state: State| state.point == end;

    let start_state = State {
        point: start,
        moves_left: 0,
    };

    let is_valid = |_: &Vec<State>| true;

    let default = a_star(start_state, &h, &neighbors, &is_end, Some(1), &is_valid)[0].len();


    println!("Total moves without teleporting: {}", default);

    let is_valid = |path: &Vec<State>| {
        println!("Found path with len {}", path.len());
        path.len() < default
    };

    let start_state = State {
        point: start,
        moves_left: 2,
    };

    let save = a_star(start_state, &h, &neighbors, &is_end, None, &is_valid);

    println!("Paths that save: {}", save.len())
}
