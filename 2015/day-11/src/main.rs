use std::fs::read_to_string;
use std::io::{Read, Result};

fn check(password: &str) -> bool {
    let mut has_ilo = false;
    let mut has_seq = false;
    let mut pair_count = 0;

    let mut last_letter = b'\n';
    let mut succeeds_pair = false;
    let mut running_seq_length = 1;
    for c in password.bytes() {
        if c == b'i' || c == b'o' || c == b'l' {
            has_ilo = true;
            break;
        }
        if c == last_letter && !succeeds_pair {
            pair_count += 1;
            succeeds_pair = true;
            last_letter = c;
            running_seq_length = 1;
            continue;
        }
        succeeds_pair = false;
        if c == last_letter + 1 {
            running_seq_length += 1;
        } else {
            running_seq_length = 1;
        }
        if running_seq_length == 3 {
            has_seq = true;
        }
        last_letter = c;
    }
    has_seq && pair_count > 1 && !has_ilo
}

fn next_password(password: String) -> String {
    let mut new_password = password.as_bytes().to_vec();
    for ix in (0..password.len()).rev() {
        new_password[ix] = (new_password[ix] - b'a' + 1) % 26 + b'a';
        if [b'i', b'o', b'l'].contains(&new_password[ix]) {
            new_password[ix] += 1;
            break;
        }
        if new_password[ix] != b'a' {
            break;
        }
    }
    String::from_utf8(new_password).expect("Invalid UTF-8 sequence")
}

fn clean_password(password: String) -> String {
    let mut new_password = password.as_bytes().to_vec();
    if let Some(ix) = password.find(|c| c == 'i' || c == 'o' || c == 'l') {
        new_password[ix] += 1;
        for jx in ix + 1..new_password.len() {
            new_password[jx] = b'a';
        }
    }
    String::from_utf8(new_password).expect("Invalid UTF-8 sequence")
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut password = input.trim().to_string();

    password = clean_password(password);

    while !check(&password) {
        password = next_password(password);
    }

    println!("{password}");
    
    password = next_password(password);

    while !check(&password) {
        password = next_password(password);
    }

    println!("{password}");

    Ok(())
}

fn load_input(test: bool) -> String {
    let path = if let Some(arg) = std::env::args().nth(1) {
        if arg == "--default-input" {
            if test {
                "./input/sample_input.txt"
            } else {
                "./input/input.txt"
            }.to_string()
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
