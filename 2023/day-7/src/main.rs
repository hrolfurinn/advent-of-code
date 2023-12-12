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
        'J' => 11,
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
        let mut counter_vec = (0..=14)
            .map(|v| {
                hand.chars()
                    .filter(|c| to_value(*c) == v)
                    .collect::<Vec<_>>()
                    .len() as i64
            })
            .enumerate()
            .map(|(value, count)| (value as i64, count))
            .collect::<Vec<_>>();
        counter_vec.sort_by(|(v1, c1), (v2, c2)| {
            return match c2.cmp(c1) {
                Ordering::Equal => v2.cmp(v1),
                x => x,
            };
        }); // highest first
        let values = hand.chars().map(|c| to_value(c)).collect::<Vec<_>>();

        return match (counter_vec[0], counter_vec[1]) {
            ((v, 5), (_, _)) => Self {
                kind: 7,
                values
            },
            ((v, 4), (_, _)) => Self {
                kind: 6,
                values
            },
            ((v1, 3), (v2, 2)) => Self {
                kind: 5,
                values
            },
            ((v, 3), (_, _)) => Self {
                kind: 4,
                values
            },
            ((v1, 2), (v2, 2)) => Self {
                kind: 3,
                values
            },
            ((v, 2), (_, _)) => Self {
                kind: 2,
                values
            },
            ((v, 1), (_, _)) => Self {
                kind: 1,
                values
            },
            _ => panic!("Couldn't process hand {hand:?}"),
        };
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
        println!("{}", "-".to_string().repeat(20));
        println!("{line:?}");
        let player = Player::from(line);
        println!("kind: {:?}", player.hand.kind);
        println!("values: {:?}", player.hand.values);
        println!("bet: {:?}", player.bet);
        players.push(player);
    }

    players.sort_by(|a, b| a.hand.cmp(&b.hand)); // note reverse order

    for (rank, player) in players.iter().enumerate() {
        println!("{}", "-".to_string().repeat(20));
        println!("player rank {:?}", rank);
        println!("player bet {:?}", player.bet);
        println!("player kind {:?}", player.hand.kind);
        println!("player card values {:?}", player.hand.values);
        println!("player cards {:?}", player.cards);
    }

    p1 = players
        .iter()
        .enumerate()
        .map(|(rank, player)| player.bet * (rank + 1) as i64)
        .sum();

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
