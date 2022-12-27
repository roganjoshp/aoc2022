use std::fs;

#[derive(Debug)]
struct Section {
    lower_id: i32,
    upper_id: i32,
}

impl Section {
    fn new(bounds: &str) -> Self {
        let (lower, upper) = bounds.split_once("-").unwrap();
        Section {
            lower_id: lower.parse::<i32>().unwrap(),
            upper_id: upper.parse::<i32>().unwrap(),
        }
    }

    fn overlapping_pair(&self, other: &Self) -> i32 {
        if self.lower_id <= other.lower_id && self.upper_id >= other.upper_id {
            return 1;
        } else if self.lower_id >= other.lower_id && self.upper_id <= other.upper_id {
            return 1;
        }
        0
    }
}

fn read_input(input_file: &str) -> Vec<Vec<Section>> {
    let file = fs::read_to_string(input_file).expect("Cannot read input file");
    let rows: Vec<Vec<Section>> = file
        .split("\n")
        .map(|x| x.split(",").map(|y| Section::new(y)).collect())
        .collect();
    rows
}

fn find_contained_pairs(rows: &Vec<Vec<Section>>) -> () {
    let overlapping_pairs: i32 = rows
        .iter()
        .map(|row| row[0].overlapping_pair(&row[1]))
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    println!("Overlapping: {}", overlapping_pairs);
}

fn main() {
    let sections = read_input("./d4_input.txt");
    find_contained_pairs(&sections);
}
