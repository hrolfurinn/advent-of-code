use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut orderings: HashMap<usize, Vec<usize>> = HashMap::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let nums = line
            .split("|")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        if let Some(list) = orderings.get_mut(&nums[0]) {
            list.push(nums[1]);
        } else {
            orderings.insert(nums[0], vec![nums[1]; 1]);
        }
        if !orderings.contains_key(&nums[1]) {
            orderings.insert(nums[1], Vec::new());
        }
    }
    for line in lines {
        let print_attempt = line
            .split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_vec();
        let mut ok = true;
        for first_ix in 0..print_attempt.len() {
            for second_ix in first_ix + 1..print_attempt.len() {
                if let Some(list) = orderings.get(&print_attempt[second_ix]) {
                    if list.contains(&print_attempt[first_ix]) {
                        ok = false
                    };
                }
            }
        }
        if ok {
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
                    && !orderings
                        .get(&new_line[jx])
                        .unwrap()
                        .contains(&new_line[jx - 1])
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
