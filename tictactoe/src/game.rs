use crate::board::{Board,Square};
use crate::dc::DrawingContext;
use crate::human_player::HumanPlayer;
use crate::player::Player;
use crate::random_player::RandomPlayer;
use crate::value_iteration_player::VIPlayer;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use std::boxed::Box;
use std::cmp;
use std::mem;

pub struct Game {
    pub cell_width: u32,
    pub board: Board,
    pub next_to_play: Square,
    pub finished: bool,
    pub current_player: Box<Player>,
    pub other_player: Box<Player>,
}

impl Game {
    pub fn new(screen_width: u32, screen_height: u32) -> Game {
        let cell_width = cmp::min(screen_width, screen_height)/3;
        let board = Board::new();
        let next_to_play = Square::White;
        let finished = false;
        // TODO: Make it possible to choose on the command line which player uses which algorithm
        //let mut current_player = Box::new(RandomPlayer::new());
        let mut current_player = Box::new(HumanPlayer::new());
        //let mut current_player = Box::new(VIPlayer::new(Square::White));
        let other_player = Box::new(VIPlayer::new(Square::Black));
        //let other_player = Box::new(HumanPlayer::new());
        current_player.turn_starts(&board);
        Game {
            cell_width,
            board,
            next_to_play,
            finished,
            current_player,
            other_player,
        }
    }

    fn coord_to_pos(&self, x: i32, y: i32) -> (usize, usize) {
        ((x as u32/self.cell_width) as usize, (y as u32/self.cell_width) as usize)
    }

    pub fn handle_click(&mut self, x: i32, y: i32) {
        if !self.finished {
            let (i, j) = self.coord_to_pos(x, y);
            self.current_player.mouse_clicked((i, j));
            self.other_player.mouse_clicked((i, j));
        }
    }

    pub fn try_next_move(&mut self) {
        if !self.finished {
            if let Some((i, j)) = self.current_player.move_to_play() {
                if let Ok(_) = self.board.set_pos(i, j, self.next_to_play) {
                    self.next_to_play = self.next_to_play.next();
                    mem::swap(&mut self.current_player, &mut self.other_player);
                }
                if let Some(c) = self.board.winner() {
                    //println!("{:?} won !", c);
                    self.finished = true;
                } else {
                    self.current_player.turn_starts(&self.board);
                }
            }
        }
    }

    fn show_bg(&self, dc: &mut DrawingContext) {
        let black = Color::RGB(0, 0, 0);
        dc.canvas.set_draw_color(black);
        dc.canvas.fill_rect(sdl2::rect::Rect::new(0, 0, dc.width, dc.height)).unwrap();
    }

    fn show_board(&self, dc: &mut DrawingContext) {
        let cell_width = cmp::min(dc.width, dc.height)/3;
        let grey = Color::RGB(127, 127, 127);
        let red = Color::RGB(255, 0, 0);
        let blue = Color::RGB(0, 0, 255);
        for i in 0..3 {
            for j in 0..3 {
                dc.canvas.set_draw_color(grey);
                dc.canvas.draw_rect(sdl2::rect::Rect::new((i * cell_width) as i32, (j * cell_width) as i32, cell_width, cell_width)).unwrap();
                match self.board.at(i as usize, j as usize).unwrap() {
                    Square::White => {
                        dc.canvas.set_draw_color(red);
                        dc.canvas.fill_rect(sdl2::rect::Rect::new((i * cell_width + cell_width/5) as i32, (j * cell_width + cell_width/5) as i32, 3*cell_width/5, 3*cell_width/5)).unwrap()
                    },
                    Square::Black => {
                        dc.canvas.filled_circle((i * cell_width + cell_width/2) as i16, (j * cell_width + cell_width/2) as i16, (3*cell_width/10) as i16, blue).unwrap()
                    },
                    _ => {},
                }
            }
        }
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        self.show_bg(dc);
        self.show_board(dc);
    }

}

impl Drop for Game {
    fn drop(&mut self) {
        self.current_player.save_model();
        self.other_player.save_model();
    }
}
