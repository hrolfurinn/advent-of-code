use std::env;
use std::fs::read_to_string;
use std::io::Result;

fn load_input(test: bool) -> String {
    let path = if test {
        "./input/sample_input.txt"
    } else {
        "./input/input.txt"
    };
    match read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            println!("{e:?}");
            "dummy_path".to_string()
        }
    }
}

fn get_numbers(string: String) -> Vec<i64> {
    string
        .split(|c: char| !c.is_numeric() && !c.eq(&'-'))
        .filter_map(|s| match s.parse::<i64>() {
            Ok(num) => Some(num),
            Err(e) => {
                println!("{:?}", e);
                None
            }
        })
        .collect()
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let lines = input.lines();

    for line in lines {
        let mut numbers = get_numbers(line.to_string());
        let mut depth = 0;
        p1 += numbers[numbers.len() - 1];
        p2 += numbers[0];

        while numbers.iter().any(|n| !n.eq(&0)) {
            depth += 1;
            numbers = numbers
                .windows(2)
                .map(|window| match window {
                    [a, b] => b - a,
                    _ => unreachable!("Window size not two"),
                })
                .collect::<Vec<_>>();
            // adding the last numbers of the layers yields the next number in the seq
            p1 += numbers[numbers.len() - 1];
            // add alternating signs of the first number in each layer yields the previous number
            p2 += numbers[0] * ((-1_i64).pow(depth));
        }
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
