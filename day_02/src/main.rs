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

fn build_outcome_map() -> HashMap<Move, i32> {
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

fn play_game_two(all_rows: &Vec<Vec<String>>) -> () {
    let mut my_plays: Vec<&str> = Vec::with_capacity(all_rows.len());
    let mut scores: Vec<i32> = Vec::with_capacity(all_rows.len());

    // TODO how can this be generalised? This is gross
    for row in all_rows {
        if row[0] == "A" {
            if row[1] == "X" {
                my_plays.push("scissors");
                scores.push(0);
            } else if row[1] == "Y" {
                my_plays.push("rock");
                scores.push(3);
            } else {
                my_plays.push("paper");
                scores.push(6);
            }
        } else if row[0] == "B" {
            if row[1] == "X" {
                my_plays.push("rock");
                scores.push(0);
            } else if row[1] == "Y" {
                my_plays.push("paper");
                scores.push(3);
            } else {
                my_plays.push("scissors");
                scores.push(6);
            }
        } else {
            if row[1] == "X" {
                my_plays.push("paper");
                scores.push(0);
            } else if row[1] == "Y" {
                my_plays.push("scissors");
                scores.push(3);
            } else {
                my_plays.push("rock");
                scores.push(6);
            }
        }
    }
    let total_score: i32 = scores.iter().sum();
    let hand_scores: i32 = my_plays
        .iter()
        .map(|&play| match play {
            "rock" => 1,
            "paper" => 2,
            "scissors" => 3,
            _ => 0,
        })
        .collect::<Vec<i32>>()
        .iter()
        .sum();
    let soln: i32 = total_score + hand_scores;
    println!("Game 2, {}", soln);
}

fn main() {
    let all_rows = read_file("./d2_input.txt");
    play_game_one(&all_rows);
    play_game_two(&all_rows);
}
