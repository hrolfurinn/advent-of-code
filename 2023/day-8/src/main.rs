use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::io::Result;
use std::str::Lines;
use num_integer::Integer;

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
        let child_key = child_key.to_string();
        let _ = match direction {
            Direction::Left => self.left = Some(child_key),
            Direction::Right => self.right = Some(child_key),
        };
    }

    pub fn get_child(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.left.clone().expect("No L child for node"),
            Direction::Right => self.right.clone().expect("No R child for node"),
        }
    }
}

impl Graph {
    pub fn from(mut lines: Lines) -> (Self, Vec<String>) {
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let mut starting_keys: Vec<String> = Vec::new();
        let new_nodes = lines.filter(|line| !line.is_empty()).map(|line| {
            match line
                .split(|c: char| !c.is_alphanumeric())
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
            let node_key = node_key.to_string();
            if node_key.clone().ends_with("A") {
                starting_keys.push(node_key.clone())
            }
            node.add_child(left_key, Direction::Left);
            node.add_child(right_key, Direction::Right);
            nodes.insert(node_key, node);
        }
        (Self { nodes: nodes }, starting_keys)
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
    let test = false;

    let input = load_input(test);

    let mut p2: i64;

    let mut lines = input.lines();

    let instructions = Instructions::from(lines.next().unwrap());

    let list_len = instructions.list.len() as i32;
    
    let (map, mut current_nodes) = Graph::from(lines.clone());
    let branch_count = current_nodes.len();
    
    let mut last_zs: Vec<Option<i32>> = Vec::from([None].repeat(branch_count));
    let mut cycles: Vec<Option<i64>> = Vec::from([None].repeat(branch_count));
    let mut first_zs: Vec<Option<i32>> = Vec::from([None].repeat(branch_count));
    
    let mut ix = 0 as i32;
    let mut count = 0;

    while let direction = instructions.list[ix as usize] {
        let mut new_nodes: Vec<String> = Vec::new();
        for (branch_index, node_key) in current_nodes.iter().enumerate() {
            let node = map.get(node_key);
            let next_node = node.get_child(direction);
            if next_node.ends_with("Z") {
                // println!("Branch {branch_index}");
                // println!("Count: {count}");
                match last_zs[branch_index] {
                    Some(last_z) => {
                        let cycle_length = (count - last_z) / list_len;
                        let first_z = ((count - (ix + 1)) % (cycle_length * list_len)) / list_len; 
                        // println!("Branch: {:?} Count: {:?} Cycles = {:?}, Index = {:?}, First z = {:?}, list_len = {:?}", branch_index, count, cycle_length, ix, first_z, list_len);
                        cycles[branch_index] = Some((count - last_z) as i64);
                        if count - last_z == 0 {
                            if first_zs[branch_index].is_none() {
                                first_zs[branch_index] = Some(last_z);
                            }
                        }
                    }
                    None => {}
                };
                last_zs[branch_index] = Some(count)
            }
            new_nodes.push(next_node);
        }
        count += 1;
        ix = (ix + 1) % list_len;
        if cycles.iter().all(|cycle| cycle.is_some()) {
            println!("nodes: {:?}", new_nodes);
            break;
        } else if new_nodes.iter().any(|n| n.ends_with("Z")){
            println!("{:?}", new_nodes);
            println!("{:?}", cycles);
        }

        current_nodes = new_nodes;
    }
    println!("cycles {:?}", cycles);
    println!("first_zs {:?}", first_zs);
    println!("last_zs {:?}", last_zs);
    let mut p2: i64 = 1;
    let mut cycles = cycles.iter();
    while let Some(Some(number)) = cycles.next() {
        p2 = p2.lcm(number);
    }

    println!("p2: {}", p2);

    Ok(())
}
