use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn read_input(filename: &str) -> Forest {
    let file: String = fs::read_to_string(filename).expect("Cannot read file");
    let lines: Vec<u32> = file
        .split("\n")
        .flat_map(|row| row.chars().map(|character| character.to_digit(10).unwrap()))
        .collect();

    let num_cols = file.split("\n").next().unwrap().len();

    Forest::new(lines, num_cols)
}

#[derive(Debug)]
struct Forest {
    trees: Vec<u32>,
    num_columns: usize,
    num_rows: usize,
}

impl Forest {
    fn new(trees: Vec<u32>, num_columns: usize) -> Self {
        let num_rows = &trees.len() / num_columns;
        Forest {
            trees: trees,
            num_columns: num_columns,
            num_rows: num_rows,
        }
    }

    fn get_tree_height(&self, row: usize, column: usize) -> u32 {
        self.trees[(row * self.num_columns) + column]
    }

    fn view_from_top(&self, row: usize, column: usize, height: u32) -> bool {
        self.trees[column..(row * self.num_columns)]
            .iter()
            .step_by(self.num_columns)
            .all(|&x| x < height)
    }

    fn view_from_bottom(&self, row: usize, column: usize, height: u32) -> bool {
        self.trees[((row + 1) * self.num_columns) + column..(self.num_rows * self.num_columns)]
            .iter()
            .step_by(self.num_columns)
            .all(|&x| x < height)
    }

    fn view_from_left(&self, row: usize, column: usize, height: u32) -> bool {
        let start = row * self.num_columns;
        let end = (row * self.num_columns) + column;
        self.trees[start..end].iter().all(|&x| x < height)
    }

    fn view_from_right(&self, row: usize, column: usize, height: u32) -> bool {
        let start = (row * self.num_columns) + column + 1;
        let end = (row + 1) * self.num_columns;
        self.trees[start..end].iter().all(|&x| x < height)
    }
}

fn count_visible_trees(forest: &Forest) -> usize {
    let combos = (1..forest.num_rows - 1).cartesian_product(1..forest.num_columns - 1);

    combos
        .filter(|&(row, col)| {
            let height = forest.get_tree_height(row, col);
            forest.view_from_top(row, col, height)
                || forest.view_from_bottom(row, col, height)
                || forest.view_from_left(row, col, height)
                || forest.view_from_right(row, col, height)
        })
        .collect::<HashSet<_>>()
        .len()
        + 2 * (forest.num_rows - 1)
        + 2 * (forest.num_columns - 1)
}
fn main() {
    let forest = read_input("./d8_input.txt");
    println!(
        "{:?}",
        forest.view_from_bottom(97, 10, forest.get_tree_height(97, 10))
    );
    let count_visible = count_visible_trees(&forest);
    println!("{count_visible}");
}
