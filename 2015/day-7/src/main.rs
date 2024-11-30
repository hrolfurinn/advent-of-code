use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{Read, Result};

fn get_key(value: &str) -> [u8; 2] {
    let mut result = [0; 2];
    for (i, b) in value.bytes().take(2).enumerate() {
        result[i] = b;
    }
    result
}

struct Memory {
    map: HashMap<[u8; 2], String>,
    cache: HashMap<[u8; 2], u32>,
}

impl Memory {
    fn get(&mut self, value: &str) -> u32 {
        if let Ok(num) = value.parse::<u32>() {
            return num;
        }

        let key = get_key(value);
        if let Some(&cached) = self.cache.get(&key) {
            return cached;
        }

        let command = self
            .map
            .get(&key)
            .expect("Value not found in memory")
            .clone();
        let result = match command.trim().split(" ").collect::<Vec<_>>().as_slice() {
            [v1, "AND", v2] => self.get(v1) & self.get(v2),
            [v1, "OR", v2] => self.get(v1) | self.get(v2),
            [v, "LSHIFT", d] => self.get(v) << self.get(d),
            [v, "RSHIFT", d] => self.get(v) >> self.get(d),
            ["NOT", v] => !self.get(v),
            [v] => self.get(v),
            _ => unreachable!("Invalid command: {command}"),
        };
        self.cache.insert(key, result);
        result
    }

    fn set(&mut self, key: &str, command: &str) {
        self.map.insert(get_key(key), command.to_string());
    }

    fn reset_cache(&mut self) {
        self.cache.clear();
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut memory = Memory {
        map: HashMap::new(),
        cache: HashMap::new(),
    };

    for line in input.lines() {
        let (value, object) = line.split_once("->").unwrap();
        let object = object.trim();
        memory.set(object, value)
    }

    let a = memory.get("a");

    memory.set("b", &a.to_string());

    memory.reset_cache();

    let a = memory.get("a");

    println!("{a}");

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
