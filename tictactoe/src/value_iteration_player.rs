// This is a computer player using the Value Iteration algorithm to adapt its policy.
// Value iteration is equivalent to TD(O) (Temporal Difference with lambda = 0).
// Each state is associated with a value. At each step, we update the state's value:
//   V(x_t) = max_u(r(x_t, u) + gamma*V(x_t+1))
// Intuitively, the value of a state is the maximum for all possible action of the direct reward of
// the action plus the value of the state after the action discounted by a factor gamma.
// Here, the reward r(x_t, u) is always 0 except when winning (+1) or loosing (-1).
// We could also use gamma = 1 to simplify.
// Using a value of gamma is important in some cases, for example so that loops (like invalid action),
// which lead to stay in the current state, are not preventing the state value from converging. Or
// when the path from state to state can be infinite with rewards between them, in which case the
// value can end up being infinite.
// We shouldn't need it here as we'll only consider actions that make the state progress.
//
// We start with all states having a value of 0. Each time the player has to choose what to play,
// it checks in which state each move would bring it (first level states), and from each of these 
// states, in which state each opponent move would bring it (second level states). For each of the
// 'first level' state, it keeps a value being the min of the corresponding second level states
// (assuming the opponent will play its best move). Then it takes the max of the 'first level' 
// states values and updates the current state's value with it.

extern crate rand;

use crate::board::{Board,Square};
use crate::player::Player;
use rand::Rng;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const EXPLORATION_PERCENT: f64 = 0.0;
const WIN_SCORE: f64 = 10.0;
const TIE_SCORE: f64 = 1.0;
const LOOSE_SCORE: f64 = -10.0;
const GAMMA: f64 = 0.95;

#[derive(Serialize, Deserialize)]
pub struct VIPlayer {
    my_color: Square,
    values: HashMap<u32, f64>,
    to_play: Option<(usize, usize)>
}

impl VIPlayer {
    pub fn new(color: Square) -> VIPlayer {
        let filename = Self::model_name(color);
        let path = Path::new(filename.as_str());
        if path.exists() {
            serde_json::from_str(&fs::read_to_string(path).expect(&format!("Unable to read file {:?}.", path))).unwrap()
        } else {
            VIPlayer{
              my_color: color,
              values: HashMap::new(),
              to_play: None,
            }
        }
    }

    pub fn model_name(color: Square) -> String {
        format!("models/VI_{:?}.json", color)
    }

    pub fn color_val(color: Square) -> u32 {
        match color {
            Square::Empty => 0,
            Square::White => 1,
            Square::Black => 2,
        }
    }

    pub fn state_hash(&self, board: &Board) -> u32 {
        let mut result = 0;
        for i in 0..3 {
            for j in 0..3 {
                result *= 3;
                result += Self::color_val(board.at(i, j).unwrap());
            }
        }
        result
    }

    pub fn get_value_or_insert(&mut self, state: u32) -> f64 {
        let value = self.values.get(&state);
        if let Some(v) = value {
            *v
        } else {
            self.values.insert(state, 0.0);
            0.0
        }
    }
}

impl Player for VIPlayer {
    fn turn_starts(&mut self, board: &Board) {
        self.to_play = None;
        let opponent_color = self.my_color.next();
        let current_state = self.state_hash(board);
        //println!("Current board {}:\n{}", current_state, board.to_string());
        // Try all possible moves
        let mut max_value = None;
        for i in 0..3 {
            for j in 0..3 {
                if let Square::Empty = board.at(i, j).unwrap() {
                    let mut board_copy = board.clone();
                    board_copy.set_pos(i, j, self.my_color).unwrap();
                    let new_state = self.state_hash(&board_copy);
                    //println!("After playing, board {}:\n{}", new_state, board_copy.to_string());
                    let state_value = match board_copy.winner() {
                        None => {
                            // And for each one, try all possible opponent moves
                            let mut min_value = None;
                            for i2 in 0..3 {
                                for j2 in 0..3 {
                                    if let Square::Empty = board_copy.at(i2, j2).unwrap() {
                                        let mut board_copy = board.clone();
                                        board_copy.set_pos(i, j, self.my_color).unwrap();
                                        board_copy.set_pos(i2, j2, opponent_color).unwrap();
                                        let opponent_state = self.state_hash(&board_copy);
                                        //println!("After opponent playing, board {}:\n{}", opponent_state, board_copy.to_string());
                                        let opponent_state_value = match board_copy.winner() {
                                            None => {
                                                let opponent_value = self.get_value_or_insert(opponent_state);
                                                Some(GAMMA*opponent_value)
                                            },
                                            // It's a tie, worth nothing.
                                            Some(Square::Empty) => {
                                                self.values.insert(opponent_state, TIE_SCORE);
                                                Some(GAMMA*TIE_SCORE)
                                            },
                                            Some(c) => {
                                                // If the opponent is not winning, it means win won before this move which
                                                // shouldn't happen!
                                                assert_eq!(c, self.my_color.next());
                                                self.values.insert(opponent_state, LOOSE_SCORE);
                                                Some(GAMMA*LOOSE_SCORE)
                                            },
                                        };
                                        //println!("After opponent playing, board {}:\n{} has value {}", opponent_state, board_copy.to_string(), opponent_state_value.unwrap());
                                        if let None = min_value {
                                            min_value = opponent_state_value;
                                        } else if opponent_state_value.unwrap() < min_value.unwrap() {
                                            min_value = opponent_state_value;
                                        }
                                    }
                                }
                            }
                            Some(GAMMA*min_value.unwrap())
                        },
                        // It's a tie, worth nothing.
                        Some(Square::Empty) => {
                            self.values.insert(new_state, TIE_SCORE);
                            Some(GAMMA*TIE_SCORE)
                        },
                        // If we won, then it's a great move !
                        Some(c) => {
                            // If we're not of the winning color, it means the opponent won before this move which
                            // shouldn't happen!
                            assert_eq!(c, self.my_color, "Current board:\n{}", board.to_string());
                            self.values.insert(new_state, WIN_SCORE);
                            Some(GAMMA*WIN_SCORE)
                        }
                    };
                    //println!("After playing, board {}:\n{} has score {}", new_state, board_copy.to_string(), state_value.unwrap());
                    if let None = max_value {
                        max_value = state_value;
                        self.to_play = Some((i as usize, j as usize));
                    } else if state_value.unwrap() > max_value.unwrap() {
                        max_value = state_value;
                        self.to_play = Some((i as usize, j as usize));
                    }
                }
            }
        }
        //println!("Current board {}:\n{} has score {}", current_state, board.to_string(), max_value.unwrap());
        self.values.insert(current_state, max_value.unwrap());
    }

    fn move_to_play(&mut self) -> Option<(usize, usize)> {
        if rand::thread_rng().gen::<f64>()*100.0 < EXPLORATION_PERCENT {
            let i = rand::thread_rng().gen_range(0, 3);
            let j = rand::thread_rng().gen_range(0, 3);
            Some((i, j))
        } else {
            self.to_play
        }
    }

    fn save_model(&mut self) {
        self.to_play = None;
        let filename = Self::model_name(self.my_color);
        fs::write(&filename, serde_json::to_string_pretty(&self).unwrap()).expect(&format!("Unable to write file {:?}.", filename));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_hash() {
        let mut board = Board::new();
        let vip = VIPlayer::new(Square::White);
        assert_eq!(0, vip.state_hash(&board));
        board.set_pos(2, 2, Square::White).unwrap();
        assert_eq!(1, vip.state_hash(&board));
        board.set_pos(2, 1, Square::Black).unwrap();
        assert_eq!(7, vip.state_hash(&board));
        board.set_pos(0, 0, Square::White).unwrap();
        assert_eq!(6568, vip.state_hash(&board));
    }
}
