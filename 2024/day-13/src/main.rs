use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::ops::{Index, IndexMut};

type Point = (i64, i64);

fn get_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn solve_linear_eq(a: Point, b: Point, x: Point) -> (i64, i64) {
    let mut n = 0;
    let mut m = 0;

    // We can apply the solution
    // N = D^{-1} X
    // i.e.
    // N = (1/det(D)) * adj(D) * X
    // n = (1/det(D)) ( x_1*b_2 - x_2*b_1 )
    // m = (1/det(D)) ( x_2*a_1 - x_1*a_1 )
    let det = (a.0 * b.1) - (a.1 * b.0);
    let adj_x = ((x.0 * b.1) - (x.1 * b.0), (x.1 * a.0) - (x.0 * a.1));
    if adj_x.0 % det == 0 && adj_x.1 % det == 0 {
        n = adj_x.0 / det;
        m = adj_x.1 / det;
    }
    (n, m)
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    for description in input.split("\n\n") {
        let (a, b, x) = description
            .lines()
            .map(|line| {
                line.split(|c: char| !c.is_digit(10))
                    .filter(|segment| !segment.is_empty())
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect_tuple::<(i64, i64)>()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        // The problem is a system of linear equations
        // n*A + m*B = X
        // where our desired solution for part 1 is n + m
        //
        // We can rewrite the equation in matrix form
        // D * N = X
        // Where D is the direction matrix (A B), i.e. with columns as our possible directions,
        // and N = (n // m) is a column vector with rows n and m.
        //
        // We have a closed form solution since all A,B are linearly indpendent in input.

        let (n, m) = solve_linear_eq(a, b, x);
        p1 += n * 3 + m;
        // Part 2 works the same but we change x

        let x = (x.0 + 10000000000000, x.1 + 10000000000000);

        let (n, m) = solve_linear_eq(a, b, x);
        p2 += n * 3 + m;
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
