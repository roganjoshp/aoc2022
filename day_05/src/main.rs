use std::collections::{HashMap, VecDeque};
use std::fs;

struct Dock {
    stacks: HashMap<usize, Stack>,
}

#[derive(Debug)]
struct Stack {
    crates: VecDeque<Crate>,
}

impl Stack {
    fn start_new(new_crate: Crate) -> Self {
        Stack {
            crates: VecDeque::from(vec![new_crate]),
        }
    }

    fn add_crate(&mut self, new_crate: Crate) -> () {
        self.crates.push_back(new_crate);
    }
}

#[derive(Debug)]
struct Crate {
    id: char,
}

impl Crate {
    fn new(id: char) -> Self {
        Crate { id: id }
    }
}

fn parse_file(file: &str) -> () {
    let contents = fs::read_to_string(file).expect("Cannot open file");
    let split_contents: Vec<&str> = contents.split("\n").collect();
    let split_point = split_contents
        .iter()
        .position(|&search| search == "")
        .unwrap();
    let (stacks, moves) = split_contents.split_at(split_point);
    let mut stacks: Vec<Vec<char>> = stacks
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    // Build from the bottom up
    stacks.reverse();

    // First find each of the stacks
    let starter: Vec<(usize, char)> = stacks[1..]
        .iter()
        .flat_map(|layer| {
            layer
                .chunks(4)
                .into_iter()
                .enumerate()
                .map(|(i, item)| (i + 1, item[1]))
                .filter(|(_, crate_name)| !crate_name.is_ascii_whitespace())
                .collect::<Vec<(usize, char)>>()
        })
        .collect();

    // Now build them up

    let next_level = starter
        .iter()
        .fold(HashMap::<usize, Stack>::new(), |mut map, val| {
            map.entry(val.0)
                .and_modify(|stack| stack.crates.push_back(Crate::new(val.1)))
                .or_insert(Stack::start_new(Crate::new(val.1)));
            map
        });

    println!("{:?}", &next_level);
}

fn main() {
    let input = parse_file("./d5_input.txt");
}
