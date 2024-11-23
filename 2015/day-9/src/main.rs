use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::Result;

struct Geography {
    cities: Vec<String>,
    distances: HashMap<(String, String), u32>,
}

impl Geography {
    fn add(&mut self, city_1: &str, city_2: &str, distance: &str) {
        let city_1 = city_1.to_string();
        let city_2 = city_2.to_string();
        let distance = distance.parse::<u32>().unwrap();
        if !self.cities.contains(&city_1) {
            self.cities.push(city_1.clone())
        }
        if !self.cities.contains(&city_2) {
            self.cities.push(city_2.clone())
        }
        self.distances.insert((city_1.clone(), city_2.clone()), distance);
        self.distances.insert((city_2.clone(), city_1.clone()), distance);
    }

    fn shortest_path(&self) -> u32 {
        self.cities
            .iter()
            .permutations(self.cities.len())
            .map(|path| {
                path.into_iter()
                    .tuple_windows()
                    .map(|(city_1, city_2)| self.distances.get(&(city_1.to_string(), city_2.to_string())).unwrap())
                    .sum::<u32>()
            })
            .max().unwrap()
    }
}

fn main() -> Result<()> {
    let test = false;

    let input = load_input(test);

    let mut geography = Geography {
        cities: Vec::new(),
        distances: HashMap::new(),
    };

    for line in input.lines() {
        let paths = line
            .split_whitespace()
            .collect::<Vec<_>>();
        let city_1 = paths[0];
        let city_2 = paths[2];
        let distance = paths[4];
        geography.add(city_1, city_2, distance);
    }

    let result = geography.shortest_path();

    print!("{result}");

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
