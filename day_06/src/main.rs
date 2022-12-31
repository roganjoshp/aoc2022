use std::collections::HashSet;
use std::fs;

fn read_input(filename: &str) -> Vec<char> {
    let file_str = fs::read_to_string(filename).expect("Cannot read file");
    let signal: Vec<char> = file_str.chars().collect();
    signal
}

fn find_uniques(signal: &Vec<char>, unique_no: usize) -> usize {
    let windows = signal.windows(unique_no);
    for (i, window) in windows.enumerate() {
        let set: HashSet<&char> = HashSet::from_iter(window.iter());
        if set.len() == unique_no {
            return i + unique_no;
        }
    }
    return 0;
}

fn main() {
    let input = read_input("./d6_input.txt");
    // println!("{:?}", input);
    let start_point: usize = find_uniques(&input, 4);
    println!("Start char: {:?}", start_point);
    let message: usize = find_uniques(&input, 14);
    println!("Start char: {:?}", message);
}
