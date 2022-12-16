use derive_more::Display;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Display, Clone)]
struct MoveCodeError(String);

impl Error for MoveCodeError {}

enum Move {
    Rock,
    Paper,
    Scissor,
}

fn decode_move(encoded: &str) -> Result<Move, Box<dyn Error>> {
    match encoded {
        "A" => Ok(Move::Rock),
        "B" => Ok(Move::Paper),
        "C" => Ok(Move::Scissor),
        "X" => Ok(Move::Rock),
        "Y" => Ok(Move::Paper),
        "Z" => Ok(Move::Scissor),
        _ => Err(Box::new(MoveCodeError(format!("Unknown encoded move '{}'", encoded))))
    }
}

fn decode_strategy(encoded: &str, opponent_move: &Move) -> Result<Move, Box<dyn Error>> {
    match encoded {
        "X" => { // Lose
            match opponent_move {
                Move::Rock => Ok(Move::Scissor),
                Move::Paper => Ok(Move::Rock),
                Move::Scissor => Ok(Move::Paper),
            }
        },
        "Y" => { // Draw
            match opponent_move {
                Move::Rock => Ok(Move::Rock),
                Move::Paper => Ok(Move::Paper),
                Move::Scissor => Ok(Move::Scissor),
            }
        },
        "Z" => { // Win
            match opponent_move {
                Move::Rock => Ok(Move::Paper),
                Move::Paper => Ok(Move::Scissor),
                Move::Scissor => Ok(Move::Rock),
            }
        },
        _ => Err(Box::new(MoveCodeError(format!("Unknown encoded move '{}'", encoded))))
    }
}

fn score_outcome(my_move: &Move, opponent_move: &Move) -> i64 {
    match my_move {
        Move::Rock => match opponent_move {
            Move::Rock => 3,
            Move::Paper => 0,
            Move::Scissor => 6,
        },
        Move::Paper => match opponent_move {
            Move::Rock => 6,
            Move::Paper => 3,
            Move::Scissor => 0,
        },
        Move::Scissor => match opponent_move {
            Move::Rock => 0,
            Move::Paper => 6,
            Move::Scissor => 3,
        },
    }
}

fn score_move(my_move: &Move) -> i64 {
    match my_move {
        Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;
    for l in lines {
        let l = l?;
        if l.is_empty() {
            continue
        }
        let moves = l.split(' ').collect::<Vec<_>>();
        let opponent_move = decode_move(moves[0])?;
        let my_move = decode_strategy(moves[1], &opponent_move)?;
        score += score_move(&my_move);
        score += score_outcome(&my_move, &opponent_move);
    }

    println!("Total score: {}", score);

    Ok(())
}
