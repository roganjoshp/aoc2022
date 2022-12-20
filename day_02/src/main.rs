use hashbrown::HashMap;
use std::fs;

fn read_file(file_name: &str) -> Vec<Vec<String>> {
    let file_str = fs::read_to_string(file_name).unwrap();
    let rows: Vec<&str> = file_str.split("\n").collect();
    let split_rows = rows
        .iter()
        .map(|row| row.split(" ").map(|x| x.to_owned()).collect())
        .collect();
    split_rows
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Move {
    them: String,
    me: String,
}

impl Move {
    fn new(them: &str, me: &str) -> Self {
        Move {
            them: them.to_owned(),
            me: me.to_owned(),
        }
    }
}

fn build_outcome_map<'a>() -> HashMap<Move, i32> {
    let my_move = vec!["X", "X", "X", "Y", "Y", "Y", "Z", "Z", "Z"];
    let their_move = vec!["A", "B", "C", "A", "B", "C", "A", "B", "C"];
    let win_loss = vec![3, 0, 6, 6, 3, 0, 0, 6, 3];

    let mut move_outcomes: HashMap<Move, i32> = HashMap::new();

    for (i, (&them, &me)) in their_move.iter().zip(&my_move).enumerate() {
        move_outcomes.insert(Move::new(them, me), win_loss[i]);
    }
    move_outcomes
}

fn play_game_one(all_rows: &Vec<Vec<String>>) -> () {
    let move_outcomes = build_outcome_map();

    // First get the win/lose outcomes
    let scores: i32 = all_rows
        .iter()
        .map(|row| *move_outcomes.get(&Move::new(&row[0], &row[1])).unwrap())
        .collect::<Vec<i32>>()
        .iter()
        .sum();

    let mut hand_scores: i32 = 0;

    for hand in all_rows.iter() {
        let this_score = match hand[1].as_str() {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        hand_scores += this_score;
    }

    let total_score = scores + hand_scores;
    println!("Part 1 output: {}", total_score);
}

fn main() {
    let all_rows = read_file("./d2_input.txt");
    play_game_one(&all_rows);
}
