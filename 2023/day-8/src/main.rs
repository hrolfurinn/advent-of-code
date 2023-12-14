use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::str::Lines;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Node {
    key: String,
    left: Option<String>,
    right: Option<String>,
}

#[derive(Clone)]
struct Graph {
    nodes: HashMap<String, Node>,
}

impl Node {
    pub fn from(key: &str) -> Self {
        Self {
            key: key.to_string(),
            left: None,
            right: None,
        }
    }
    pub fn add_child(&mut self, child_key: &str, direction: Direction) {
        println!("Adding to {:?}: {:?} as a {:?} child", self.key, child_key, direction);
        let child_key = child_key.to_string();
        let _ = match direction {
            Direction::Left => {println!("Left child");self.left = Some(child_key)},
            Direction::Right => {println!("Right child");self.right = Some(child_key)},
        };
    }

    pub fn get_child(&self, direction: Direction) -> String {
        println!("Node: {:?}", self);
        match direction {
            Direction::Left => self.left.clone().expect("No L child for node"),
            Direction::Right => self.right.clone().expect("No R child for node"),
        }
    }
}

impl Graph {
    pub fn from(mut lines: Lines) -> Self {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        fn parse(line: &str) -> Vec<&str> {
            return line
                .split(|c: char| !c.is_alphabetic())
                .filter(|w| w.is_empty())
                .take(3)
                .collect::<Vec<_>>();
        }
        let new_nodes = lines.filter(|line| !line.is_empty()).map(|line| {
            println!("Line: {:?}", line);
            match line
                .split(|c: char| !c.is_alphabetic())
                .filter(|w| !w.is_empty())
                .take(3)
                .collect::<Vec<_>>()
                .split_first()
            {
                Some((parent_key, [left_key, right_key])) => (
                    (*parent_key, Node::from(*parent_key)),
                    (*left_key, *right_key),
                ),
                _ => panic!("Failed to split line {line}"),
            }
        });
        for ((node_key, mut node), (left_key, right_key)) in new_nodes.clone() {
            node.add_child(left_key, Direction::Left);
            node.add_child(right_key, Direction::Right);
            nodes.insert(node_key.to_string(), node);
        }
        for node in nodes.values() {
            println!("Current state of node {:?}", node.key);
            println!("Left child {:?}", node.left);
            println!("Right child {:?}", node.right);
        }
        Self { nodes: nodes }
    }
    pub fn get(&self, key: &str) -> &Node {
        match self.nodes.get(key) {
            Some(node) => node,
            None => panic!("Could not find node!"),
        }
    }
}

#[derive(Debug)]
struct Instructions {
    state: i32,
    list: Vec<Direction>,
}

impl Instructions {
    pub fn from(line: &str) -> Self {
        let list = line
            .chars()
            .map(|c| {
                if c == 'L' {
                    Direction::Left
                } else if c == 'R' {
                    Direction::Right
                } else {
                    unreachable!("Not a valid character for directions!");
                }
            })
            .collect::<Vec<_>>();
        Self {
            state: 0,
            list: list,
        }
    }
}

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

fn main() -> Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let test = true;

    let input = load_input(test);

    let mut p1: i64;
    let mut p2: i64;

    let mut lines = input.lines();

    let instructions = Instructions::from(lines.next().unwrap());

    let list_len = instructions.list.len();
    let mut ix = 0;

    let map = Graph::from(lines.clone());

    let mut count = 0;
    let mut current_node = map.get("AAA");

    while let direction = instructions.list[ix] {
        println!("Current node: {:?}", current_node.key);
        current_node = map.get(&current_node.get_child(direction));
        count += 1;
        ix = (ix + 1) % list_len;
        if current_node.key == "ZZZ" { break }
    }
    println!("Count: {}", count);

    p1 = count;
    p2 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
