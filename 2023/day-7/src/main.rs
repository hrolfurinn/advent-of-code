use std::cmp::Ordering;
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

fn to_value(c: char) -> i32 {
    match c {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            panic!("{c:?} not a valid card")
        }
    }
}

struct Player {
    hand: Hand,
    bet: i32,
}

impl Player {
    pub fn from(line: &str) -> Self {
        let [cards, amount]: [&str; 2] = line
            .split_whitespace()
            .take(2)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            hand: Hand::from(cards),
            bet: match amount.parse::<i32>() {
                Ok(n) => n,
                Err(e) => {
                    panic!("{e:?}")
                }
            },
        }
    }
}

struct Hand {
    kind: i32,
    values: Vec<i32>,
}

impl Hand {
    pub fn from(hand: &str) -> Self {
        let mut counter_vec = (0..=14)
            .map(|v| {
                hand.chars()
                    .filter(|c| to_value(*c) == v)
                    .collect::<Vec<_>>()
                    .len() as i32
            })
            .enumerate()
            .map(|(value, count)| (value as i32, count))
            .collect::<Vec<_>>();
        counter_vec.sort_by(|(v1, c1), (v2, c2)| {
            return match c2.cmp(c1) {
                Ordering::Equal => v2.cmp(v1),
                x => x,
            };
        }); // highest first

        return match (counter_vec[0], counter_vec[1]) {
            ((v, 5), (_, _)) => Self {
                kind: 7,
                values: Vec::from([v]),
            },
            ((v, 4), (_, _)) => Self {
                kind: 6,
                values: Vec::from([v]),
            },
            ((v1, 3), (v2, 2)) => Self {
                kind: 5,
                values: Vec::from([v1, v2]),
            },
            ((v, 3), (_, _)) => Self {
                kind: 4,
                values: Vec::from([v]),
            },
            ((v1, 2), (v2, 2)) => Self {
                kind: 3,
                values: Vec::from([v1, v2]),
            },
            ((v, 2), (_, _)) => Self {
                kind: 2,
                values: Vec::from([v]),
            },
            ((v, 1), (_, _)) => Self {
                kind: 1,
                values: Vec::from([v]),
            },
            _ => panic!("Couldn't process hand {hand:?}"),
        };
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut lines = input.lines();

    let mut hands: Vec<Hand> = vec![];

    for line in lines {
        println!("{}", "-".to_string().repeat(20));
        println!("{line:?}");
        let player = Player::from(line);
        println!("kind: {:?}", player.hand.kind);
        println!("values: {:?}", player.hand.values);
        println!("bet: {:?}", player.bet);
    }

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
