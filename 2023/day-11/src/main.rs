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
    let test = true;

    let input = load_input(test);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut row_count: usize = 0;
    let mut column_count: usize = 0;

    let mut filled_rows: HashSet<usize> = HashSet::new();
    let mut filled_columns: HashSet<usize> = HashSet::new();
    let mut galaxies: Vec<(usize,usize)> = Vec::new();
    
    for (row_index, line) in input.lines().enumerate() {
        column_count = line.len();
        for (column_index, character) in line.chars().enumerate() {
            if character == '#' {
                filled_rows.insert(row_index);
                filled_columns.insert(column_index);
                galaxies.push((row_index,column_index))
            }
        }
        row_count += 1;
    }

    let mut cum_row_shift = 0;
    let mut row_shifts: Vec<_> = (0..row_count).map(|index| {
        cum_row_shift += !filled_rows.contains(&index) as usize;
        index + cum_row_shift
    }).collect();
    let mut cum_column_shift = 0;
    let mut column_shifts: Vec<_> = (0..column_count).map(|index| {
        cum_column_shift += !filled_columns.contains(&index) as usize;
        index + cum_row_shift
    }).collect();
    for (id, galaxy) in galaxies.iter().enumerate() {println!("Location {:?}: {:?}", id, galaxy)}
    galaxies = galaxies.iter().map(
        |(row_index, column_index)| {
            (row_index + row_shifts[*row_index], column_index + column_shifts[*column_index])
        }
    ).collect();
    for (id, galaxy) in galaxies.iter().enumerate() {println!("New Location {:?}: {:?}", id, galaxy)}


    p1 = galaxies.iter().enumerate().map(
        |(id, (row1, col1))| {
            let distances = galaxies.iter().skip(id + 1).map(
                |(row2, col2)| {
                    let distance = (*row2 as i64 - *row1 as i64).abs() + (*col2 as i64 - *col1 as i64).abs();
                    println!("Distance between ({:?},{:?}) and ({:?},{:?}) is {:?}", row1,col1,row2,col2,distance);
                    distance
                }
            );
            distances.sum::<i64>()
        }
    ).sum::<i64>();

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
