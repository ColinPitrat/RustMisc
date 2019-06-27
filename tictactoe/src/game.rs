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
use std::collections::HashMap;
use std::mem;

pub struct TicTacToe {
    pub cell_width: u32,
    pub score_height: u32,
    pub game: Game,
    pub stats: HashMap<Square, u32>,
}

pub struct Game {
    pub board: Board,
    pub next_to_play: Square,
    pub finished: bool,
    pub current_player: Box<Player>,
    pub other_player: Box<Player>,
}

impl Game {
    pub fn new() -> Game {
        let board = Board::new();
        let next_to_play = Square::White;
        let finished = false;

        // TODO: Make it possible to choose on the command line which player uses which algorithm
        //let mut current_player = Box::new(HumanPlayer::new());
        //let mut current_player = Box::new(RandomPlayer::new());
        let mut current_player = Box::new(VIPlayer::new(Square::White));

        //let other_player = Box::new(HumanPlayer::new());
        //let other_player = Box::new(RandomPlayer::new());
        let other_player = Box::new(VIPlayer::new(Square::Black));

        current_player.turn_starts(&board);
        Game {
            board,
            next_to_play,
            finished,
            current_player,
            other_player,
        }
    }
}

impl TicTacToe {
    pub fn new(screen_width: u32, screen_height: u32) -> TicTacToe {
        let cell_width = cmp::min(screen_width, screen_height)/3;
        let score_height = cmp::max(0, screen_height as i32 - 3*cell_width as i32) as u32;
        let game = Game::new();
        let mut stats = HashMap::new();
        stats.insert(Square::Empty, 0);
        stats.insert(Square::White, 0);
        stats.insert(Square::Black, 0);
        TicTacToe {
            cell_width,
            score_height,
            game,
            stats,
        }
    }

    pub fn reset(&mut self) {
        self.game = Game::new();
    }

    fn coord_to_pos(&self, x: i32, y: i32) -> (usize, usize) {
        ((x as u32/self.cell_width) as usize, (y as u32/self.cell_width) as usize)
    }

    pub fn handle_click(&mut self, x: i32, y: i32) {
        if !self.game.finished {
            let (i, j) = self.coord_to_pos(x, y);
            self.game.current_player.mouse_clicked((i, j));
            self.game.other_player.mouse_clicked((i, j));
        }
    }

    pub fn try_next_move(&mut self) {
        if !self.game.finished {
            if let Some((i, j)) = self.game.current_player.move_to_play() {
                if let Ok(_) = self.game.board.set_pos(i, j, self.game.next_to_play) {
                    self.game.next_to_play = self.game.next_to_play.next();
                    mem::swap(&mut self.game.current_player, &mut self.game.other_player);
                }
                if let Some(c) = self.game.board.winner() {
                    self.stats.insert(c, self.stats.get(&c).unwrap() + 1);
                    self.game.finished = true;
                } else {
                    self.game.current_player.turn_starts(&self.game.board);
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
                match self.game.board.at(i as usize, j as usize).unwrap() {
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

    fn show_score(&self, dc: &mut DrawingContext) {
        if self.score_height > 0 {
            // Background
            let black = Color::RGB(0, 0, 0);
            dc.canvas.set_draw_color(black);
            dc.canvas.fill_rect(sdl2::rect::Rect::new(0, (3*self.cell_width) as i32, 3*self.cell_width, self.score_height)).unwrap();
            // Scores
            let white = Color::RGB(255, 255, 255);
            let red = Color::RGB(255, 0, 0);
            let blue = Color::RGB(0, 0, 255);
            let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 50).unwrap();
            let nb_white = font.render(&self.stats.get(&Square::White).unwrap().to_string()).solid(red).unwrap();
            let nb_tie =  font.render(&self.stats.get(&Square::Empty).unwrap().to_string()).solid(white).unwrap();
            let nb_black = font.render(&self.stats.get(&Square::Black).unwrap().to_string()).solid(blue).unwrap();
            let r_white = sdl2::rect::Rect::new(0, (3*self.cell_width) as i32, nb_white.rect().w as u32, nb_white.rect().h as u32);
            let r_tie = sdl2::rect::Rect::new((dc.width as i32 - nb_tie.rect().w)/2, (3*self.cell_width) as i32, nb_tie.rect().w as u32, nb_tie.rect().h as u32);
            let r_black = sdl2::rect::Rect::new(dc.width as i32 - nb_black.rect().w, (3*self.cell_width) as i32, nb_black.rect().w as u32, nb_white.rect().h as u32);
            let nb_white = dc.texture_creator.create_texture_from_surface(nb_white).unwrap();
            let nb_tie = dc.texture_creator.create_texture_from_surface(nb_tie).unwrap();
            let nb_black = dc.texture_creator.create_texture_from_surface(nb_black).unwrap();
            dc.canvas.copy(&nb_white, None, r_white).expect("Rendering score failed");
            dc.canvas.copy(&nb_tie, None, r_tie).expect("Rendering score failed");
            dc.canvas.copy(&nb_black, None, r_black).expect("Rendering score failed");
        }
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        self.show_bg(dc);
        self.show_board(dc);
        self.show_score(dc);
    }
}

impl Drop for TicTacToe {
    fn drop(&mut self) {
        self.game.current_player.save_model();
        self.game.other_player.save_model();
        let whites = self.stats.get(&Square::White).unwrap();
        let blacks = self.stats.get(&Square::Black).unwrap();
        let ties = self.stats.get(&Square::Empty).unwrap();
        let total = whites + blacks + ties;
        println!("Total games: {}", total);
        println!("White wins: {} ({}%)", whites, (f64::from(100*whites)/f64::from(total)));
        println!("Black wins: {} ({}%)", blacks, (f64::from(100*blacks)/f64::from(total)));
        println!("Ties: {} ({}%)", ties, (f64::from(100*ties)/f64::from(total)));
    }
}
