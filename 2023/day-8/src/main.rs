use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::Result;

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
}

#[derive(Clone, Copy)]
struct Node<'a> {
    key: &'a str,
    L: Option<&'a Node<'a>>,
    R: Option<&'a Node<'a>>,
}

#[derive(Clone)]
struct Map<'a> {
    nodes: HashMap<&'a str, Node<'a>>,
    unbound_nodes: HashMap<&'a str, Vec<(Node<'a>, Direction)>>,
}

impl<'a> Node<'a> {
    pub fn from(key: &'a str) -> Self {
        Self {
            key: key,
            L: None,
            R: None,
        }
    }
    pub fn add_child(&mut self, node: &'a Node<'a>, direction: &Direction) {
        match direction {
            Direction::L => self.L = Some(node),
            Direction::R => self.R = Some(node),
        }
    }
    pub fn get_child(&self, direction: &Direction) -> &'a Node<'a> {
        match direction {
            Direction::L => self.L.expect("No L child for node"),
            Direction::R => self.R.expect("No R child for node"),
        }
    }
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            unbound_nodes: HashMap::new(),
        }
    }
    pub fn add(mut self, line: &'a str) {
        let [key, left, right]: [&str; 3] = line
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| w.is_empty())
            .take(3)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut node = Node::from(key);
        self.nodes.insert(key, node);
        while let Some((mut parent, direction)) = self.unbound_nodes.get_mut(key).unwrap().pop() {
            parent.add_child(&node, &direction);
        }
        self.unbound_nodes.remove(key);
        if let Some(left_child) = self.nodes.get(left) {
            node.add_child(&left_child, &Direction::L);
        } else {
            self.unbound_nodes
                .entry(key)
                .or_insert(Vec::new())
                .push((node, Direction::L));
        }
        if let Some(right_child) = self.nodes.get(right) {
            node.add_child(&right_child, &Direction::R);
        } else {
            self.unbound_nodes
                .entry(key)
                .or_insert(Vec::new())
                .push((node, Direction::R));
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
                    Direction::L
                } else if c == 'R' {
                    Direction::R
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

    for direction in instructions.list {
        println!("{direction:?}");
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
    }

    p1 = 0;
    p2 = 0;

    println!("p1: {}", p1);
    println!("p2: {}", p2);

    Ok(())
}
