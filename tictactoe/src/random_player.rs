extern crate rand;

use crate::player::Player;
use crate::board::Board;
use rand::Rng;

pub struct RandomPlayer {
}

impl RandomPlayer {
    pub fn new() -> RandomPlayer {
        RandomPlayer{}
    }
}

impl Player for RandomPlayer {
    fn turn_starts(&mut self, _board: &Board) {}

    fn move_to_play(&mut self) -> Option<(usize, usize)> {
        let i = rand::thread_rng().gen_range(0, 3);
        let j = rand::thread_rng().gen_range(0, 3);
        Some((i, j))
    }
}
