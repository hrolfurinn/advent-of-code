use std::fs::read_to_string;
use std::io::Result;
use std::collections::HashSet;

fn process_line(word: &str) -> u32 {
    let mut twin_pairs = HashSet::new();

    let mut has_dupe_twin_pair = false;
    let mut has_exe = false;

    for char_triple in word.as_bytes().windows(3) {
        let c1 = char_triple[0];
        let c2 = char_triple[1];
        let c3 = char_triple[2];
        match (c1 == c2, c1 == c3) {
            (_, true) => has_exe = true,
            (true,false) => { 
                if twin_pairs.contains(&c1) { 
                    has_dupe_twin_pair = true 
                } else { twin_pairs.insert(c1); };
            },
            _ => {}
        }
    }

    if has_exe && has_dupe_twin_pair { return 1 }
    0

}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut good_line_count = 0;
    for line in input.lines() {
        good_line_count += process_line(line);
    }
    println!("{good_line_count}");

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
