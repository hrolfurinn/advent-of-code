use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn find_xmas(string: &[u8]) -> u32 {
    string
        .windows(4)
        .map(|bytes| {
            if bytes == "XMAS".as_bytes() || bytes == "SAMX".as_bytes() {
                1
            } else {
                0
            }
        })
        .sum()
}

fn find_x_mas(bytes: Vec<Vec<u8>>) -> bool {
    let right_down = (0..3).map(|ix| bytes[ix][ix]).collect_vec();
    let left_down = (0..3).map(|ix| bytes[ix][2 - ix]).collect::<Vec<_>>();
    let good_strings = ["MAS".as_bytes().to_vec(), "SAM".as_bytes().to_vec()];
    good_strings.contains(&right_down) && good_strings.contains(&left_down)
}

fn print_bytes(bytes: &[u8]) {
    let string = std::str::from_utf8(bytes.as_ref()).unwrap();
    println!("{string}");
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut p1 = 0;
    let mut p2 = 0;

    let mut rows: Vec<&[u8]> = Vec::new();

    for line in input.lines() {
        let bytes = line.trim().as_bytes();
        rows.push(bytes);
        p1 += find_xmas(bytes);
    }
    let row_len = rows[0].len();
    let col_len = rows.len();
    let transpose: Vec<Vec<u8>> = (0..col_len)
        .map(|col_ix| (0..row_len).map(|row_ix| rows[row_ix][col_ix]).collect())
        .collect();
    for column in transpose.iter() {
        p1 += find_xmas(column);
    }
    let mut shifted_rows: Vec<Vec<u8>> = (0..row_len)
        .map(|row_ix| {
            (0..row_len - row_ix) // know input is square
                .map(|col_ix| rows[row_ix + col_ix][col_ix])
                .collect()
        })
        .collect();
    let mut more_shifted_rows: Vec<Vec<u8>> = (1..col_len)
        .map(|col_ix| {
            (0..col_len - col_ix)
                .map(|row_ix| rows[row_ix][col_ix + row_ix])
                .collect()
        })
        .collect();
    shifted_rows.append(&mut more_shifted_rows);
    for diagonal in shifted_rows.iter() {
        p1 += find_xmas(diagonal);
    }
    let mut shifted_transpose: Vec<Vec<u8>> = (0..row_len)
        .map(|row_ix| {
            (0..row_len - row_ix)
                .map(|col_ix| rows[row_ix + col_ix][col_len - col_ix - 1])
                .collect()
        })
        .collect();
    let mut more_shifted_transpose: Vec<Vec<u8>> = (1..col_len)
        .map(|col_ix| {
            (0..col_len - col_ix)
                .map(|row_ix| rows[row_ix][col_len - col_ix - row_ix - 1])
                .collect()
        })
        .collect();
    shifted_transpose.append(&mut more_shifted_transpose);
    for diagonal in shifted_transpose.iter() {
        p1 += find_xmas(diagonal);
    }
    for row_ix in 0..row_len - 2 {
        for col_ix in 0..col_len - 2 {
            p2 += find_x_mas(
                (0..3)
                    .map(|row_inc| {
                        (0..3)
                            .map(|col_inc| rows[row_ix + row_inc][col_ix + col_inc])
                            .collect_vec()
                    })
                    .collect_vec(),
            ) as u32;
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
