use std::cmp::Ordering;
use std::collections::HashMap;
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

fn to_value(c: char) -> i64 {
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
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => {
            panic!("{c:?} not a valid card")
        }
    }
}

#[derive(Clone, Debug)]
struct Player {
    hand: Hand,
    bet: i64,
    cards: String,
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
            bet: match amount.parse::<i64>() {
                Ok(n) => n,
                Err(e) => {
                    panic!("{e:?}")
                }
            },
            cards: cards.to_string(),
        }
    }
}

#[derive(Clone, Debug)]
struct Hand {
    kind: i64,
    values: Vec<i64>,
}

impl Hand {
    pub fn from(hand: &str) -> Self {
        let mut counts = HashMap::new();

        for c in hand.chars() {
            *counts.entry(to_value(c)).or_insert(0) += 1;
        }
        let mut joker_count = *counts.entry(to_value('J')).or_default();
        let mut counter_vec: Vec<(i64, i64)> = counts.into_iter().collect::<Vec<_>>();
        counter_vec.sort_by(|&(v1, c1), &(v2, c2)| {
            return match c2.cmp(&c1) {
                Ordering::Equal => v2.cmp(&v1),
                other => other,
            };
        }); // highest first
        let (mut value, mut count) = counter_vec[0];
        if value == to_value('J') {
            if counter_vec.len() == 1 {
                // "JJJJJ" => kind: 5
                joker_count = 0;
            } else {
                // sufficient to mask the jokers to the most common card
                // this always results in the best hand
                counter_vec.remove(0);
                (value, count) = counter_vec[0];
            }
        };
        counter_vec[0] = (value, count + joker_count);

        let values = hand.chars().map(to_value).collect::<Vec<_>>();

        match counter_vec.as_slice() {
            &[(_, 5), ..] => Self { kind: 7, values },
            &[(_, 4), ..] => Self { kind: 6, values },
            &[(_, 3), (_, 2), ..] => Self { kind: 5, values },
            &[(_, 3), ..] => Self { kind: 4, values },
            &[(_, 2), (_, 2), ..] => Self { kind: 3, values },
            &[(_, 2), ..] => Self { kind: 2, values },
            &[(_, 1), ..] => Self { kind: 1, values },
            _ => panic!("Couldn't process hand {hand:?}"),
        }
    }
    pub fn cmp(&self, other: &Hand) -> Ordering {
        return match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.values.cmp(&other.values),
            ord => ord,
        };
    }
}

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = false;

    let input = load_input(test);

    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    let mut lines = input.lines();

    let mut players: Vec<Player> = vec![];

    for line in lines {
        let player = Player::from(line);
        players.push(player);
    }

    players.sort_by(|a, b| a.hand.cmp(&b.hand)); // note reverse order

    p2 = players
        .iter()
        .enumerate()
        .map(|(rank, player)| player.bet * (rank + 1) as i64)
        .sum();

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
