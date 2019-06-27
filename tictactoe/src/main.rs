extern crate clap;
extern crate sdl2;

mod board;
mod dc;
mod game;
mod human_player;
mod player;
mod q_learning_player;
mod random_player;
mod value_iteration_player;

use clap::{Arg,App};
use crate::dc::DrawingContext;
use crate::game::TicTacToe;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH : u32 = 600;
const SCREEN_HEIGHT : u32 = 660;
const DEFAULT_PLAYER1 : &str = "human";
const DEFAULT_PLAYER2 : &str = "value_iteration";

fn help_message() {
    println!("Use --help for command line arguments.");
    println!("You can use the following commands in the game:");
    println!(" - Click to play (when at least one human player is selected).");
    println!(" - R: Restart a new game.");
    println!(" - Escape: Quit the game.");
}

fn main() {
    let matches = App::new("Tic Tac Toe")
        .version("0.1")
        .author("Colin Pitrat")
        .about("Play Tic Tac Toe against the computer, or let it play against itself.")
        .arg(Arg::with_name("autoreset")
                .short("a")
                .long("autoreset")
                .help("When provided, automatically restart a new game after one finished."))
        .arg(Arg::with_name("player1")
                .short("1")
                .long("player1")
                .takes_value(true)
                .help(&format!("Algorithm to use for player 1 (human, q_learning, random, value_iteration). Default: {}", DEFAULT_PLAYER1)))
        .arg(Arg::with_name("player2")
                .short("2")
                .long("player2")
                .takes_value(true)
                .help(&format!("Algorithm to use for player 2 (human, q_learning, random, value_iteration). Default: {}.", DEFAULT_PLAYER2)))
        .get_matches();
    let autoreset = matches.is_present("autoreset");
    let player1_algorithm = matches.value_of("player1").unwrap_or(DEFAULT_PLAYER1).to_string();
    let player2_algorithm = matches.value_of("player2").unwrap_or(DEFAULT_PLAYER2).to_string();
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    let mut tictactoe = TicTacToe::new(SCREEN_WIDTH, SCREEN_HEIGHT, player1_algorithm, player2_algorithm);
    help_message();
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    tictactoe.reset();
                }
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    tictactoe.handle_click(x, y);
                },
                _ => {},
            }
        }

        if autoreset && tictactoe.game.finished {
            tictactoe.reset();
        }
        tictactoe.try_next_move();
        tictactoe.show(&mut dc);
        dc.canvas.present();
    }
}
