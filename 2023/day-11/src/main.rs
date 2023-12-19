use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;
use std::io::Result;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        // Ok("".to_string()) => panic!("Input empty"),
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);

    let p1: i64;
    let p2: i64;

    let mut row_count: usize = 0;
    let mut column_count: usize = 0;

    let mut filled_rows: HashSet<usize> = HashSet::new();
    let mut filled_columns: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for (row_index, line) in input.lines().enumerate() {
        column_count = line.len();
        for (column_index, character) in line.trim().chars().enumerate() {
            if character == '#' {
                filled_rows.insert(row_index);
                filled_columns.insert(column_index);
                galaxies.push((row_index, column_index));
            }
        }
        row_count += 1;
    }

    let (mut cum_row_shift1, mut cum_row_shift2) = (0, 0);
    let (row_shifts1, row_shifts2): (Vec<_>, Vec<_>) = (0..row_count)
        .map(|index| {
            cum_row_shift1 += !filled_rows.contains(&index) as usize;
            cum_row_shift2 += (1000000 - 1) * (!filled_rows.contains(&index) as usize);
            (index + cum_row_shift1, index + cum_row_shift2)
        })
        .unzip();
    let (mut cum_column_shift1, mut cum_column_shift2) = (0, 0);
    let (column_shifts1, column_shifts2): (Vec<_>, Vec<_>) = (0..column_count)
        .map(|index| {
            cum_column_shift1 += !filled_columns.contains(&index) as usize;
            cum_column_shift2 += (1000000 - 1) * (!filled_columns.contains(&index) as usize);
            (index + cum_column_shift1, index + cum_column_shift2)
        })
        .unzip();
    let (galaxies1, galaxies2): (Vec<_>, Vec<_>) = galaxies
        .iter()
        .map(|(row_index, column_index)| {
            (
                (row_shifts1[*row_index], column_shifts1[*column_index]),
                (row_shifts2[*row_index], column_shifts2[*column_index]),
            )
        })
        .unzip();
    p1 = galaxies1
        .iter()
        .enumerate()
        .flat_map(|(id, (row1, col1))| {
            galaxies1[id + 1..].iter().map(move |(row2, col2)| {
                let row_diff = (*row2 as i64 - *row1 as i64).abs();
                let col_diff = (*col2 as i64 - *col1 as i64).abs();
                row_diff + col_diff
            })
        })
        .sum();

    p2 = galaxies2
        .iter()
        .enumerate()
        .flat_map(|(id, (row1, col1))| {
            galaxies2[id + 1..].iter().map(move |(row2, col2)| {
                let row_diff = (*row2 as i64 - *row1 as i64).abs();
                let col_diff = (*col2 as i64 - *col1 as i64).abs();
                row_diff + col_diff
            })
        })
        .sum();

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
