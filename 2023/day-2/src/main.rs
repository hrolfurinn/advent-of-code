use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
#[derive(Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Game {
    id: i32,
    rounds: Vec<Vec<(Color, i32)>>, // Each round contains counts of each color
}

impl FromStr for Color {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

fn parse_game(line: &str) -> Result<Game, &'static str> {
    let parts: Vec<&str> = line.split(": ").collect();
    if parts.len() != 2 {
        return Err("Invalid game format");
    }

    let id = parts[0]
        .trim_start_matches("Game ")
        .parse::<i32>()
        .map_err(|_| "Invalid game ID")?;

    let rounds = parts[1]
        .split(';')
        .map(|round| {
            round
                .split(',')
                .map(|pair| {
                    let mut parts = pair.trim().split_whitespace();
                    let count = parts
                        .next()
                        .ok_or("Missing count")
                        .and_then(|c| c.parse::<i32>().map_err(|_| "Invalid count"))?;
                    let color = parts
                        .next()
                        .ok_or("Missing color")
                        .and_then(|c| Color::from_str(c).map_err(|_| "Invalid color"))?;
                    Ok((color, count))
                })
                .collect::<Result<Vec<_>, &'static str>>()
        })
        .collect::<Result<Vec<_>, &'static str>>()?;

    Ok(Game { id, rounds })
}

fn is_game_possible(game: &Game, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    game.rounds.iter().all(|round| {
        let (mut reds, mut greens, mut blues) = (0, 0, 0);
        for &(color, count) in round {
            match color {
                Color::Red => reds += count,
                Color::Green => greens += count,
                Color::Blue => blues += count,
            }
        }
        reds <= max_red && greens <= max_green && blues <= max_blue
    })
}

fn min_cubes_for_game(game: &Game) -> (i32, i32, i32) {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    for round in &game.rounds {
        let (mut reds, mut greens, mut blues) = (0, 0, 0);
        for &(color, count) in round {
            match color {
                Color::Red => reds = reds.max(count),
                Color::Green => greens = greens.max(count),
                Color::Blue => blues = blues.max(count),
            }
        }
        max_red = max_red.max(reds);
        max_green = max_green.max(greens);
        max_blue = max_blue.max(blues);
    }

    (max_red, max_green, max_blue)
}

fn main() -> io::Result<()> {
    let input_path = "./input/input.txt";
    let file = File::open(input_path)?;
    let reader = io::BufReader::new(file);

    let mut sum_ids = 0;
    let mut sum_power = 0;

    for line in reader.lines() {
        let game = parse_game(&line?).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        if is_game_possible(&game, 12, 13, 14) {
            sum_ids += game.id;
        }
        let (min_red, min_green, min_blue) = min_cubes_for_game(&game);
        sum_power += min_red * min_green * min_blue;
    }

    println!("Sum of Game IDs: {}", sum_ids);
    println!("Sum of Power: {}", sum_power);

    Ok(())
}