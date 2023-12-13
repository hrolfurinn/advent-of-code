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

#[derive(Clone, Copy, Debug)]
struct Node<'a> {
    key: &'a str,
    left: Option<&'a Node<'a>>,
    right: Option<&'a Node<'a>>,
}

#[derive(Clone)]
struct Graph<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
}

impl<'a> Node<'a> {
    pub fn from(key: &'a str) -> Self {
        Self {
            key: key,
            left: None,
            right: None,
        }
    }
    pub fn add_children(&mut self, left: &'a Node<'a>, right: &'a Node<'a>) {
        self.left = Some(left);
        self.right = Some(right);
    }
    pub fn add_child(&mut self, node: &'a Node<'a>, direction: &Direction) {
        println!("Adding to {:?}: {:?} as a {:?} child", self.key, node.key, direction);
        let _ = match direction {
            Direction::Left => {println!("Left child");self.left = Some(node)},
            Direction::Right => {println!("Right child");self.right = Some(node)},
        };
    }

    pub fn get_child(&self, direction: &Direction) -> &'a Node<'a> {
        println!("Node: {:?}", self);
        match direction {
            Direction::Left => self.left.expect("No L child for node"),
            Direction::Right => self.right.expect("No R child for node"),
        }
    }
}

impl<'a> Graph<'a> {
    pub fn from(mut lines: Lines<'a>) -> Self {
        let mut nodes: HashMap<&'a str, Node<'a>> = HashMap::new();
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
        for ((node_key, node), (_, _)) in new_nodes.clone() {
            nodes.insert(node_key, node);
        }
        for ((_, mut node), (left_key, _)) in new_nodes.clone() {
            let left_child = nodes.get(left_key).unwrap();
            node.add_child(&left_child, &Direction::Left);
        }
        for ((_, mut node), (_, right_key)) in new_nodes.clone() {
            let right_child = nodes.get(right_key).unwrap();
            node.add_child(&right_child, &Direction::Right);
        }
        for node in nodes.values() {
            println!("Current state of node {:?}", node.key);
            println!("Left child {:?}", node.left);
            println!("Right child {:?}", node.right);
        }
        Self { nodes: nodes }
    }
    pub fn get(&self, key: &'a str) -> &Node<'_> {
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

    let map = Graph::from(lines.clone());

    let mut count = 0;
    let mut current_node = map.get("AAA");

    for direction in instructions.list {
        println!("Current node: {:?}", current_node.key);
        current_node = current_node.get_child(&direction);
        count += 1;
    }
    println!("Count: {}", count);

    p1 = count;
    p2 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
