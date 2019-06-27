// This is a computer player using the Q Learning algorithm to adapt its policy.
// Q Learning is similar to Value Iteration except that it affects a value to every state-action
// pairs instead of states only.
// The reason one may want to do that is that Value Iteration works poorly with non-deterministic
// Markov Decision Process (MDB), i.e cases where a same action from a same state can lead to 
// different states & rewards (probabilitstic transition). Example of such actions in a game would
// be drawing a card, rolling a die ... although Value Iteration can easily be adapted to cases
// where the probability are known. Q Learning is really useful when probabilities of the reward /
// next state are not known.
// This is clearly not the case of Tic Tac Toe which is 100% deterministic (so implementing here
// has no value other than pedagogical).
// 
// For each state (board layout) and for each action, we compute a Q-value which is:
//   Q(x_t) = r(x_t, u_t) + gamma*max_u_t+1(Q(x_t+1, u_t+1))
// Intuitively, the value of a state-action pair is the direct reward obtained by doing the action
// plus the maximum value of the actions availables in the resulting state discounted by a factor
// gamma.
// Here again, the reward r(x_t, u_t) is always 0 except when winning (+1) or loosing (-1).
// See value_iteration_player.rs for some discussion on gamma.
//
// We start with all states-action pairs having a value of 0. Each time the player has to choose
// what to play, it chooses a move (either randomly or following the policy depending on the
// exploration rate). From the resulting state, it look at which state each opponent would bring it
// and for each of these, takes the maximum Q-value of all the actions. It then take the minimum
// Q-value of all the opponent moves. This is the new Q-value for the state-action.

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
const INVALID_MOVE_SCORE: f64 = -10.0;
const GAMMA: f64 = 0.95;

#[derive(Serialize, Deserialize)]
pub struct QLPlayer {
    my_color: Square,
    q_values: HashMap<u32, f64>,
    to_play: Option<(usize, usize)>
}

impl QLPlayer {
    pub fn new(color: Square) -> QLPlayer {
        let filename = Self::model_name(color);
        let path = Path::new(filename.as_str());
        if path.exists() {
            serde_json::from_str(&fs::read_to_string(path).expect(&format!("Unable to read file {:?}.", path))).unwrap()
        } else {
            QLPlayer{
              my_color: color,
              q_values: HashMap::new(),
              to_play: None,
            }
        }
    }

    pub fn model_name(color: Square) -> String {
        format!("models/QL_{:?}.json", color)
    }

    pub fn color_val(color: Square) -> u32 {
        match color {
            Square::Empty => 0,
            Square::White => 1,
            Square::Black => 2,
        }
    }

    pub fn state_action_hash(&self, board: &Board, action: (usize, usize)) -> u32 {
        let mut result = 0;
        for i in 0..3 {
            for j in 0..3 {
                result *= 3;
                result += Self::color_val(board.at(i, j).unwrap());
            }
        }
        result*9 + action.0 as u32*3 + action.1 as u32
    }

    pub fn get_value_or_insert(&mut self, state_action: u32) -> f64 {
        if let Some(v) = self.q_values.get(&state_action) {
            *v
        } else {
            self.q_values.insert(state_action, 0.0);
            0.0
        }
    }

    pub fn choose_move_to_play(&mut self, board: &Board) -> Option<(usize, usize)> {
        // We would loop infinitely here if the board is full.
        if let Some(_) = board.winner() {
            panic!("choose_move_to_play called with full board:\n{}", board.to_string());
        }
        loop {
            let action = if rand::thread_rng().gen::<f64>()*100.0 < EXPLORATION_PERCENT {
                // Choose randomly
                let i = rand::thread_rng().gen_range(0, 3);
                let j = rand::thread_rng().gen_range(0, 3);
                let mut chosen = (i, j);
                // If a move has no score yet, choose it
                for i in 0..3 {
                    for j in 0..3 {
                        let this_value = self.get_value_or_insert(self.state_action_hash(board, (i, j)));
                        if this_value == 0.0 {
                            chosen = (i, j);
                        }
                    }
                }
                chosen
            } else {
                let mut best_move = (0, 0);
                for i in 0..3 {
                    for j in 0..3 {
                        let this_value = self.get_value_or_insert(self.state_action_hash(board, (i, j)));
                        let best_value = self.get_value_or_insert(self.state_action_hash(board, best_move));
                        if  this_value > best_value {
                            best_move = (i, j);
                        }
                    }
                }
                best_move
            };
            if let Square::Empty = board.at(action.0, action.1).unwrap() {
                // This is a valid move
                break Some(action);
            }
            // This is not a valid move, penalize it
            let bad_state_action = self.state_action_hash(board, action);
            self.q_values.insert(bad_state_action, INVALID_MOVE_SCORE);
        }
    }
}

impl Player for QLPlayer {
    fn turn_starts(&mut self, board: &Board) {
        self.to_play = self.choose_move_to_play(board);
        // In theory, all that follows would be done after the actual move is done as this would be
        // the only way to know the result of it. Here, as it's determinist, we can guess what the
        // result will be although the move will only be done after the call to move_to_play.
        // A "proper" implementation would require a callback after the move, or handling the
        // previous move here instead of the next one.
        let opponent_color = self.my_color.next();
        let current_state_action = self.state_action_hash(board, self.to_play.unwrap());
        let mut board_copy = board.clone();
        board_copy.set_pos(self.to_play.unwrap().0, self.to_play.unwrap().1, self.my_color).unwrap();
        // If the game ends after this move, no need to do anything ...
        if let None = board_copy.winner() {
            // Try all possible opponent moves
            let mut min_q_value = None;
            for i in 0..3 {
                for j in 0..3 {
                    if let Square::Empty = board_copy.at(i, j).unwrap() {
                        let mut board_copy = board_copy.clone();
                        board_copy.set_pos(i, j, opponent_color).unwrap();
                        let state_value = match board_copy.winner() {
                            None => {
                                // And for each one, take the max Q-value
                                let mut max_q_value = None;
                                for i2 in 0..3 {
                                    for j2 in 0..3 {
                                        if let Square::Empty = board_copy.at(i2, j2).unwrap() {
                                            let next_state_action = self.state_action_hash(&board_copy, (i2, j2));
                                            let mut board_copy = board_copy.clone();
                                            board_copy.set_pos(i2, j2, self.my_color).unwrap();
                                            let next_state_action_value = match board_copy.winner() {
                                                None => {
                                                    Some(GAMMA*self.get_value_or_insert(next_state_action))
                                                },
                                                // It's a tie, worth nothing.
                                                Some(Square::Empty) => {
                                                    self.q_values.insert(next_state_action, TIE_SCORE);
                                                    Some(GAMMA*TIE_SCORE)
                                                },
                                                Some(c) => {
                                                    // If we're not winning, it means the opponent won before this move which
                                                    // shouldn't happen!
                                                    assert_eq!(c, self.my_color);
                                                    self.q_values.insert(next_state_action, WIN_SCORE);
                                                    Some(GAMMA*WIN_SCORE)
                                                },
                                            };
                                            if let None = max_q_value {
                                                max_q_value = next_state_action_value;
                                            } else if next_state_action_value.unwrap() > max_q_value.unwrap() {
                                                max_q_value = next_state_action_value;
                                            }
                                        }
                                    }
                                }
                                Some(GAMMA*max_q_value.unwrap())
                            },
                            // It's a tie, worth nothing.
                            Some(Square::Empty) => {
                                Some(GAMMA*TIE_SCORE)
                            },
                            // If we lost, then it's a bad move!
                            Some(c) => {
                                // If we're of the winning color, it means we won before reaching here,
                                // it shouldn't happen!
                                assert_eq!(c, opponent_color, "Current board:\n{}", board.to_string());
                                Some(GAMMA*LOOSE_SCORE)
                            }
                        };
                        if let None = min_q_value {
                            min_q_value = state_value;
                        } else if state_value.unwrap() < min_q_value.unwrap() {
                            min_q_value = state_value;
                        }
                    }
                }
            }
            self.q_values.insert(current_state_action, min_q_value.unwrap());
        }
    }

    fn move_to_play(&mut self) -> Option<(usize, usize)> {
        self.to_play
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
    fn state_action_hash() {
        let mut board = Board::new();
        let vip = QLPlayer::new(Square::White);
        assert_eq!(0, vip.state_action_hash(&board, (0, 0)));
        assert_eq!(1, vip.state_action_hash(&board, (0, 1)));
        assert_eq!(2, vip.state_action_hash(&board, (0, 2)));
        assert_eq!(3, vip.state_action_hash(&board, (1, 0)));
        assert_eq!(6, vip.state_action_hash(&board, (2, 0)));
        assert_eq!(8, vip.state_action_hash(&board, (2, 2)));
        board.set_pos(2, 2, Square::White).unwrap();
        assert_eq!(9, vip.state_action_hash(&board, (0, 0)));
        assert_eq!(13, vip.state_action_hash(&board, (1, 1)));
        assert_eq!(17, vip.state_action_hash(&board, (2, 2)));
        board.set_pos(2, 1, Square::Black).unwrap();
        assert_eq!(63, vip.state_action_hash(&board, (0, 0)));
        assert_eq!(68, vip.state_action_hash(&board, (1, 2)));
        assert_eq!(70, vip.state_action_hash(&board, (2, 1)));
        board.set_pos(0, 0, Square::White).unwrap();
        assert_eq!(59112, vip.state_action_hash(&board, (0, 0)));
    }
}
