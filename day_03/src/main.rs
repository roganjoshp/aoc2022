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

fn elf_groups(checklists: &Vec<Vec<char>>, priority_map: &HashMap<char, u32>) -> () {
    // Oops, I didn't need a HashMap but keeping it anyway for posterity for a vec of counters
    // let chunked: Vec<Vec<char>> = checklists
    //     .chunks(3)
    //     .into_iter()
    //     .map(|x| x.iter().flatten().cloned().collect())
    //     .collect();

    // let counters: Vec<HashMap<&char, i32>> = chunked
    //     .iter()
    //     .map(|x| {
    //         x.iter().fold(HashMap::new(), |mut map, val| {
    //             map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
    //             map
    //         })
    //     })
    //     .collect();

    // First just get sets of letters
    let uniques: Vec<HashSet<&char>> = checklists
        .iter()
        .map(|x| {
            x.into_iter().fold(HashSet::new(), |mut letters, val| {
                letters.insert(val);
                letters
            })
        })
        .collect();

    // Now chunk, flatten and count
    let grouped: Vec<Vec<char>> = uniques
        .chunks(3)
        .into_iter()
        .map(|x| {
            x.into_iter().fold(Vec::new(), |mut flat_vec, val| {
                flat_vec.extend(Vec::from_iter(val.clone()));
                flat_vec
            })
        })
        .collect();

    let counts: Vec<HashMap<&char, i32>> = grouped
        .iter()
        .map(|x| {
            x.iter().fold(HashMap::new(), |mut map, val| {
                map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
                map
            })
        })
        .collect();

    let badges = counts
        .iter()
        .map(|&dict| dict.iter().filter(|&(&key, &val)| val == 3))
        .collect();

    // let badges = counts
    //     .iter()
    //     .map(|&x| x.iter().filter(|(&k, &v)| v == 3).)
    //     .collect();

    println!("{:?}", counts);
}

fn main() {
    let priority_map = build_priority_map();
    let input_data = read_file("./d3_input.txt");
    check_rucksacks(&input_data, &priority_map);
    elf_groups(&input_data, &priority_map);
}
