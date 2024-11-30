use std::fs::read_to_string;
use std::io::{Read, Result};

fn get_total(json: &serde_json::Value) -> i64 {
    match json {
        serde_json::Value::Number(n) => n.as_i64().unwrap(),
        serde_json::Value::Array(a) => a.iter().map(get_total).sum::<i64>(),
        serde_json::Value::Object(o) => {
            if o.values().any(|v| v == "red") {
                0
            } else {
                o.values().map(get_total).sum::<i64>()
            }
        }
        _ => 0,
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let json: serde_json::Value = serde_json::from_str(&input).unwrap();

    let total = get_total(&json);

    println!("{total}");

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
