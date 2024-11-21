use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::Result;

fn process_line(word: &str) -> u32 {
    println!("--------------------------");
    println!("inspecting {word}");
    let mut twin_pairs = HashSet::new();

    let length = word.len();

    let mut has_dupe_twin_pair = false;
    let mut has_exe = false;

    for (ix, char_triple) in word.as_bytes().windows(3).enumerate() {
        let c1 = char_triple[0];
        let c2 = char_triple[1];
        let c3 = char_triple[2];
        println!("inspecting {},{},{}", c1, c2, c3);

        has_dupe_twin_pair |= twin_pairs.contains(&(c1, c2));

        if ix == length - 3 {
            println!("End of the word");
            has_dupe_twin_pair |= twin_pairs.contains(&(c2, c3));
        }

        if c1 == c3 {
            println!("First and last identical");
            has_exe = true;

            if c1 == c2 {
                println!("So is the second, holding off on adding it to HashSet");
                continue;
            };
        }
        println!("Adding first two to HashSet");
        twin_pairs.insert((c1, c2));
    }

    if has_exe && has_dupe_twin_pair {
        println!("SUCCESS");
        return 1;
    }
    println!("FAILURE");
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
