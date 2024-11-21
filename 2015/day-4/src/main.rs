use std::fs::read_to_string;
use std::io::Result;

fn process_line(string: &str) -> u32 {
    let mut prefix = 1;
    while prefix > 0 {
        let new_string = string.to_owned() + &prefix.to_string();
        let hash = md5::compute(new_string);
        if format!("{:x}", hash)[0..6] == *"000000" {
            println!("For new string {string}{prefix} have {:x}", hash);
            return prefix;
        };
        prefix += 1;
    }
    prefix
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    for line in input.lines() {
        let prefix = process_line(line);
        println!("{prefix}");
    }

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read input file: {e}");
        std::process::exit(1);
    })
}
