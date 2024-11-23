use std::fs::read_to_string;
use std::io::Result;

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut code_length = 0;
    let mut string_length = 0;

    for line in input.lines().map(|l| l.chars().collect::<Vec<_>>()) {
        let mut ix = 0;
        while ix < line.len() {
            if line[ix] == '"' {
                ix += 1;
                continue;
            }
            string_length += 1;
            if !(line[ix] == '\\') {
                ix += 1;
                continue;
            }
            match line[ix + 1] {
                'x' => {
                    ix += 4;
                },
                '"' | '\\' => {
                    ix += 2;
                },
                _ => { unreachable!("No character found") },
            }
        }
        code_length += ix;
    }
    let result = code_length - string_length;

    println!("{result}");

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
