use itertools::Itertools;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn run(program: &Vec<u64>, a: u64, b: u64, c: u64) -> Vec<u64> {
    let mut instruction_ix = 0;

    let mut output = vec![];

    let (mut a,mut b,mut c) = (a,b,c);

    while instruction_ix < program.len() {
        let (opcode, literal_operand) = (program[instruction_ix],program[instruction_ix + 1]);

        let combo = |literal_operand| {
	        match literal_operand {
	            x @ 0..=3 => x,
	            4 => a,
	            5 => b,
	            6 => c,
	            _ => unreachable!("Invalid operand"),
	        }
        };

        match opcode {
            0 => { a /= 1 << combo(literal_operand); }
            1 => { b ^= literal_operand; }
            2 => { b = combo(literal_operand) % 8; }
            3 => { if a != 0 { instruction_ix = literal_operand as usize; continue; } }
            4 => { b = b ^ c; }
            5 => { output.push(combo(literal_operand) % 8); }
            6 => { b = a / (1 << combo(literal_operand)); }
            7 => { c = a / (1 << combo(literal_operand)); }
            _ => unreachable!("Invalid opcode"),
        }
        instruction_ix += 2;
    }
    return output
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let (head, tail) = input.split_once("\n\n").unwrap();

    let (mut a, b, c) = head
        .split(|c: char| !c.is_digit(10))
        .filter_map(|num| num.parse::<u64>().ok())
        .collect_tuple()
        .unwrap();

    let program = tail
        .strip_prefix("Program: ")
        .unwrap()
        .trim()
        .split(",")
        .map(|num| num.parse::<u64>().unwrap())
        .collect_vec();

    let mut output = run(&program, a, b, c);

    let p1 = output.clone();

    print!("p1: ");
    for val in p1.iter() { print!("{val},"); }
    println!();

    // For part 2, we note that between outputs, A is always divided by 8. So the last output will
    // be the same for all A that are equivalent mod 8. The second to last output will be the same
    // for all A that are equivalent mod 8**2. This is not entirely true, since A appears in the
    // closed form for each output as potentially A/(2**i) for i < 8. There is some math and
    // guesswork behind my thinking that it would work, and evidently it does.
    //
    // But this means we can construct possible A's out of A's possible remainders mod 8^i, working
    // our way through suffices of the output to gradually increase A. 

    let mut possible_as = vec![0];

    // I found this way of collecting suffices on Stack Overflow, but it needed some tweaks. I was
    // tired, so I'll let that external help slide.
    let suffices = (0..program.len()).rev().map(|cut| &program[cut..]).collect_vec();

    for suffix in suffices {

        let as_to_check = possible_as.clone();
        possible_as.clear();

        for prev_a in as_to_check.iter() {
	        for remainder in 0..8 {

	            a = prev_a*8 + remainder;

                output = run(&program, a, b, c);
                
                if output == suffix { possible_as.push(a); }
	        }
        }
    }

    let p2 = possible_as.iter().min().unwrap();
    println!("p2: {p2}");

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
