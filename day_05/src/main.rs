use std::collections::{HashMap, VecDeque};
use std::fs;

struct Dock {
    stacks: HashMap<usize, Stack>,
}

impl Dock {
    fn new(stacks: HashMap<usize, Stack>) -> Self {
        Dock { stacks: stacks }
    }
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

fn parse_file(file: &str) -> (Vec<Vec<char>>, Vec<String>) {
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

    println!("{:?}", &moves[1..]);
    let moves: Vec<String> = moves.iter().map(|&x| x.split(" ").collect()).collect();
    return (stacks, moves);
}
fn build_dock(initial_stacks: &Vec<Vec<char>>) -> Dock {
    // First find each of the stacks
    let starter_crates: Vec<(usize, char)> = initial_stacks[1..]
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

    let all_stacks = starter_crates
        .iter()
        .fold(HashMap::<usize, Stack>::new(), |mut map, val| {
            map.entry(val.0)
                .and_modify(|stack| stack.add_crate(Crate::new(val.1)))
                .or_insert(Stack::start_new(Crate::new(val.1)));
            map
        });

    Dock::new(all_stacks)
}

// fn build_dock(stacks: )

fn main() {
    let (stacks, moves) = parse_file("./d5_input.txt");
    let dock = build_dock(&stacks);
}