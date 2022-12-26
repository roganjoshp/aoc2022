use std::char;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::FromIterator;

fn read_file(path: &str) -> Vec<Vec<char>> {
    let file_str = fs::read_to_string(&path).expect("Couldn't find file path");
    let split_str = file_str
        .split("\n")
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    split_str
}

fn build_priority_map() -> HashMap<char, u32> {
    let mut lowercase: Vec<char> = Vec::with_capacity(26);
    let mut uppercase: Vec<char> = Vec::with_capacity(26);

    for i in 0..=25 {
        lowercase.push(char::from_u32(i + 97).unwrap());
        uppercase.push(char::from_u32(i + 65).unwrap());
    }
    let letters: Vec<char> = lowercase.into_iter().chain(uppercase.into_iter()).collect();
    let mut mapper: HashMap<char, u32> = HashMap::new();
    for (i, c) in letters.iter().enumerate() {
        mapper.insert(c.to_owned(), i as u32 + 1);
    }
    mapper
}

fn check_rucksacks(checklists: &Vec<Vec<char>>, priority_map: &HashMap<char, u32>) -> () {
    let mut priorities: Vec<u32> = Vec::new();
    for checklist in checklists.iter() {
        let split_point: usize = checklist.len() / 2;
        let (bag1, bag2) = checklist.split_at(split_point);
        let b1hash: HashSet<char> = HashSet::from_iter(bag1.iter().cloned());
        let b2hash: HashSet<char> = HashSet::from_iter(bag2.iter().cloned());

        let common_item = b1hash.intersection(&b2hash).next().unwrap();
        let priority = *priority_map.get(common_item).unwrap();
        priorities.push(priority);
    }
    let total_priority: u32 = priorities.iter().sum();
    println!("Total day 1, {}", total_priority);
}

fn main() {
    let priority_map = build_priority_map();
    let input_data = read_file("./d3_input.txt");
    check_rucksacks(&input_data, &priority_map)
}
