use std::fs::read_to_string;
use std::io::Result;

fn is_bad_pair(c1: u8, c2: u8) -> bool {
    match (c1, c2) {
        (b'a', b'b') => true,
        (b'c', b'd') => true,
        (b'p', b'q') => true,
        (b'x', b'y') => true,
        _ => false,
    }
}

fn is_vowel(c: u8) -> bool {
    match c {
        b'a' => true,
        b'e' => true,
        b'i' => true,
        b'o' => true,
        b'u' => true,
        _ => false,
    }
}

fn process_line(word: &str) -> u32 {
    let length = word.len();
    let mut vowel_count = 0;
    let mut has_twin_letter = false;

    for (ix, char_pair) in word.as_bytes().windows(2).enumerate() {
        let c1 = char_pair[0];
        let c2 = char_pair[1];
        if is_bad_pair(c1, c2) {
            return 0;
        };
        if c1 == c2 {
            has_twin_letter = true;
        };
        if is_vowel(c1) { vowel_count += 1 };
        if ix == length - 2 {
            if is_vowel(c2) { vowel_count += 1 };
        };
    }

    if vowel_count > 2 && has_twin_letter == true {
        return 1
    }
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
