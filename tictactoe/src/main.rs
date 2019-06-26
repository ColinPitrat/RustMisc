extern crate sdl2;

mod board;
mod dc;
mod game;
mod human_player;
mod player;
mod random_player;
mod value_iteration_player;

use crate::dc::DrawingContext;
use crate::game::Game;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH : u32 = 600;
const SCREEN_HEIGHT : u32 = 600;

fn main() {
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    let mut game = Game::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    // TODO: Factorize this and the initialization (in a Game struct?)
                    game = Game::new(SCREEN_WIDTH, SCREEN_HEIGHT);
                }
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    game.handle_click(x, y);
                },
                _ => {},
            }
        }

        // TODO: Make this a command line option
        if game.finished {
            game = Game::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        }
        game.try_next_move();
        game.show(&mut dc);
        dc.canvas.present();
    }
}
