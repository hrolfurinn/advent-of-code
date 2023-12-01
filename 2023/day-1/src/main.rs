// use anyhow::Result;
// use clap::Parser;
use std::fs::File;
use std::io::{stdout, BufReader, Cursor, BufRead};
use std::path::Path;

// #[derive(Parser)]
// struct Cli {
//     pattern: String,
//     path: PathBuf,
// }

// #[derive(Debug)]
// struct CustomError(String);

fn main()  -> std::io::Result<()> {
    // env_logger::init();

    // log::info!("Starting up.");
    // let args: Cli = Cli::parse();
    let input_path: String = String::from("./input/input.txt");
    let output_path: String = String::from("./output/output.txt");

    let f: File = File::open(input_path)?;
    let reader: BufReader<File> = BufReader::new(f);

    let mut sum: i16 = 0;

    for line in reader.lines() {
        let output = &line?;
        println!("{output}");
    }

    // let _ = grrs::find_matches(reader, &args.pattern, &mut stdout())?;

    Ok(())
}

// fn create_number(line: ) -> i16 {

// }

// #[test]
// fn find_match() {
//     let data: &str = "lorem ipsum\ndolor sit amet";
//     let pattern: &str = "lorem";

//     let reader: BufReader<Cursor<&str>> = BufReader::new(Cursor::new(data));

//     let mut result: Vec<u8> = Vec::new();

//     let _ = grrs::find_matches(reader, pattern, &mut result);

//     assert_eq!(result, b"lorem ipsum\n");
// }
