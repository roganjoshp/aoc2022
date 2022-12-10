use std::{
    collections::HashMap,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn parse_to_num(lines: &Vec<String>) -> Vec<i32> {
    let as_int: Vec<i32> = lines
        .iter()
        .map(|x| x.parse::<i32>().unwrap_or(0))
        .collect();
    as_int
}

fn get_max_calories(calories: &Vec<i32>) -> HashMap<i32, i32> {
    // Identify individual elves
    let elf_counts: Vec<i32> = calories
        .iter()
        .map(|x| match x {
            0 => 1,
            _ => 0,
        })
        .collect();

    // Create a vec to map with each elf
    let mut count: i32 = 1;
    let mut elf_count: Vec<i32> = Vec::with_capacity(elf_counts.len());

    for i in 0..elf_counts.len() {
        if elf_counts[i] == 1 {
            count += 1;
        }
        elf_count.push(count);
    }

    // Now create a hashmap of the elves
    let mut count_map: HashMap<i32, i32> = HashMap::new();

    for (elf_num, cal) in elf_count.iter().zip(calories.iter()) {
        count_map
            .entry(*elf_num)
            .and_modify(|cur_cals| *cur_cals += cal)
            .or_insert(*cal);
    }

    count_map
}

fn main() {
    let lines = lines_from_file("./d1_input.txt");
    let as_int = parse_to_num(&lines);
    let max_cals = get_max_calories(&as_int);

    // Find the biggest value
    let max_single: &i32 = max_cals.values().max().unwrap();
    println!("{}", max_single);

    // Find the largest 3 values
    let mut all_vals: Vec<&i32> = max_cals.values().collect();
    all_vals.sort();
    all_vals.reverse();

    let top_3: i32 = all_vals.into_iter().take(3).sum();
    println!("{:?}", top_3)
}
