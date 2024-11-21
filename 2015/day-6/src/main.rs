use itertools::iproduct;
use std::cmp::{max, min};
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Action {
    On,
    Off,
    Toggle,
}

struct Point {
    x: usize,
    y: usize,
}

struct Rectangle {
    left: usize,
    right: usize,
    bottom: usize,
    top: usize,
}

struct Command {
    action: Action,
    rectangle: Rectangle,
}

#[derive(Debug)]
enum GenericError {
    ActionParseError { message: String },
    PointParseError { message: String },
    RectangleParseError { message: String },
    CommandParseError { message: String, reason: String },
}

use GenericError::*;

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionParseError { message } => write!(f, "ActionParseError: {}", message),
            PointParseError { message } => write!(f, "PointParseError: {}", message),
            RectangleParseError { message } => write!(f, "RectangleParseError: {}", message),
            CommandParseError { message, reason } => {
                write!(f, "CommandParseError: {}\n\tOrigin: {}", message, reason)
            }
        }
    }
}

impl FromStr for Action {
    type Err = GenericError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim() {
            "turn on" => Ok(Action::On),
            "turn off" => Ok(Action::Off),
            "toggle" => Ok(Action::Toggle),
            _ => Err(ActionParseError {
                message: "#{value}#".to_string(),
            }),
        }
    }
}

impl FromStr for Point {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").ok_or(PointParseError {
            message: format!("Invalid format: '{}'", s),
        })?;
        let x = x_str.trim().parse::<usize>().map_err(|_| PointParseError {
            message: format!("Invalid x value '{}' in string '{}'", x_str, s),
        })?;
        let y = y_str.trim().parse::<usize>().map_err(|_| PointParseError {
            message: format!("Invalid y value '{}' in string '{}'", y_str, s),
        })?;
        Ok(Point { x, y })
    }
}

impl FromStr for Rectangle {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(" ");
        let first = Point::from_str(parts.next().ok_or(RectangleParseError {
            message: format!("Invalid first coordinate: '{}'", s),
        })?)?;
        let second = Point::from_str(parts.last().ok_or(RectangleParseError {
            message: format!("Invalid second coordinate: '{}'", s),
        })?)?;
        Ok(Rectangle {
            left: min(first.x, second.x),
            right: max(first.x, second.x),
            top: min(first.y, second.y),
            bottom: max(first.y, second.y),
        })
    }
}

impl FromStr for Command {
    type Err = GenericError;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let first_digit_ix =
            value
                .chars()
                .position(|c| c.is_ascii_digit())
                .ok_or(CommandParseError {
                    message: format!("Invalid command: '{}'", value),
                    reason: "Splitting strings".to_string(),
                })?;
        let (action_str, second_part) = value.split_at(first_digit_ix - 1);
        let action = Action::from_str(action_str).map_err(|e| CommandParseError {
            message: format!("Failed to parse command: '{value}' because of part '{action_str}'"),
            reason: e.to_string(),
        })?;
        let rectangle = Rectangle::from_str(second_part).map_err(|e| CommandParseError {
            message: format!("Failed to parse command: '{value}' because of part '{second_part}'"),
            reason: e.to_string(),
        })?;
        Ok(Command { action, rectangle })
    }
}

struct Board {
    board: Vec<Vec<usize>>,
}

impl Board {
    fn execute(&mut self, command: Command) {
        for (x, y) in iproduct!(
            command.rectangle.left..=command.rectangle.right,
            command.rectangle.top..=command.rectangle.bottom
        ) {
            self.change(x, y, command.action);
        }
    }

    fn change(&mut self, x: usize, y: usize, action: Action) {
        self.board[x][y] = match action {
            Action::On => 1,
            Action::Off => 0,
            Action::Toggle => 1 - self.board[x][y],
        }
    }

    fn total(&self) -> usize {
        self.board
            .iter()
            .map(|v| v.iter().sum::<usize>())
            .sum::<usize>()
    }
}

fn main() -> std::result::Result<(), GenericError> {
    let test = false;

    let input = load_input(test);

    let mut board = Board {
        board: vec![vec![0; 1000]; 1000],
    };

    for line in input.lines() {
        let command = Command::from_str(line)?;
        board.execute(command);
    }
    let total = board.total();
    println!("{total}");

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
