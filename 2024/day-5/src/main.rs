use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};
use std::cmp::Ordering;

fn lt(orderings: &Vec<bool>, first_num: &usize, second_num: &usize) -> bool {
    // Returns true iff the first number is supposed to appear before the second.
    // This is not a total ordering, since some number pairs can appear in any order.
    orderings[first_num * 100 + second_num]
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    // We use the fact that no value will be larger than 100.
    // The orderings vector is indexed with [100*first_num + second_num],
    // which contains a 1 iff the pair first_num|second_num was seen,
    // i.e. iff first_num < second_num
    let mut orderings = vec![false;100 * 100];

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let nums = line
            .split("|")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        orderings[100 * nums[0] + nums[1]] = true;
    }
    for line in lines {
        let print_attempt = line
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        if print_attempt.is_sorted_by(|a,b| lt(&orderings,a,b)) {
            p1 += print_attempt[print_attempt.len() / 2];
        } else {
            // Implementation of this insertion sort pseudo code
            // Source: https://en.wikipedia.org/wiki/Insertion_sort
            // i ← 1
            // while i < length(A)
            //    j ← i
            //    while j > 0 and A[j-1] > A[j]
            //        swap A[j] and A[j-1]
            //        j ← j - 1
            //    end while
            //    i ← i + 1
            // end while
            let mut new_line = print_attempt.clone();
            let mut ix = 0;
            while ix < print_attempt.len() {
                let mut jx = ix;
                while jx > 0
                    && !lt(&orderings,&new_line[jx],&new_line[jx-1])
                {
                    let larger = new_line[jx - 1];
                    let smaller = new_line[jx];
                    new_line[jx - 1] = smaller;
                    new_line[jx] = larger;
                    jx = jx - 1;
                }
                ix = ix + 1;
            }
            p2 += new_line[new_line.len() / 2];
        }
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
