// use anyhow::Result;
// use clap::Parser;
use std::fs::File;
// use std::io::{stdout, BufRead, BufReader, Cursor};
use std::io::{BufRead, BufReader};
// use std::path::Path;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: PathBuf,
// }

// #[derive(Debug)]
// struct CustomError(String);
const DIGITS: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn main() -> std::io::Result<()> {
    // env_logger::init();
    
    // log::info!("Starting up.");
    // let args: Cli = Cli::parse();
    let input_path: String = String::from("./input/input.txt");
    // let output_path: String = String::from("./output/output.txt");
    
    let f: File = File::open(input_path)?;
    let reader: BufReader<File> = BufReader::new(f);

    
    let mut sum: u32 = 0;


    // println!("{}", create_number("abc".to_string()));

    for line in reader.lines() {
        sum = sum + create_number(line?);
    }

    // let _ = grrs::find_matches(reader, &args.pattern, &mut stdout())?;

    println!("{sum}");

    println!("testing");
    let test_num = create_number_with_letters("onetwothreefour".to_string());
    println!("{test_num}");

    Ok(())
}

fn create_number(line: String) -> u32 {
    let mut first_digit: char = '\0';
    let mut last_digit: char = '\0';
    for character in line.chars() {
        if character.is_digit(10) {
            if first_digit == '\0' {
                first_digit = character;
                last_digit = character;
            } else {
                last_digit = character;
            }
        }
    }
    10 * first_digit.to_digit(10).unwrap_or(0) as u32 + last_digit.to_digit(10).unwrap_or(0) as u32
}

fn create_number_with_letters(line: String) -> u32 {
    let windows: std::slice::Windows<'_, u8> = line[..].as_bytes().windows(5);
    println!("{:?}", windows);
    // convert all byte windows to the equivalent number, based on the digits iterator
    // filter out the ones that don't map to a number
    let found_digits = windows.into_iter().filter_map(|byte_slice| convert_to_number(byte_slice));
    println!("{:?}", found_digits);
    found_digits.sum()
}

fn convert_to_number(byte_slice: &[u8]) -> Option<u32>{
    println!("In covert to number for slice {:?}", byte_slice);
    let digit_iterators = DIGITS.iter().map(|digit| digit.as_bytes().iter());
    for digit in digit_iterators {
        println!("{:?}", digit);
    }
    Some(2)
}

// #[test]
// fn find_match() {
//     let data: &str = "lorem ipsum\ndolor sit amet";
//     let pattern: &str = "lorem";

//     let reader: BufReader<Cursor<&str>> = BufReader::new(Cursor::new(data));

//     let mut result: Vec<u8> = Vec::new();

//     let _ = grrs::find_matches(reader, pattern, &mut result);

//     assert_eq!(result, b"lorem ipsum\n");
// }
