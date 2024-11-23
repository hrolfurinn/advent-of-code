use std::fs::read_to_string;
use std::io::Result;

fn read_sequence(sequence: &Vec<u32>) -> Vec<u32> {
    let mut result = Vec::<u32>::new();
    let mut ix = 0;
    while ix < sequence.len() {
        let digit = sequence[ix];
        let mut count = 1;
        while ix + count < sequence.len() && sequence[ix + count] == digit {
            count += 1;
        }
        result.append(
            &mut count
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>(),
        );
        result.push(digit);
        ix += count;
    }
    return result;
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut sequence = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    for _ in 0..50 {
        sequence = read_sequence(&sequence);
    }

    let length = sequence.len();

    println!("{length}");

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
