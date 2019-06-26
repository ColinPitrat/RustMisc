extern crate sdl2;

mod board;
mod dc;

use crate::board::{Board,Cell};
use crate::dc::DrawingContext;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::cmp;

const SCREEN_WIDTH : u32 = 600;
const SCREEN_HEIGHT : u32 = 600;

fn show_board(dc: &mut DrawingContext, board: &Board) {
    let cell_width = cmp::min(dc.width, dc.height)/3;
    let black = Color::RGB(0, 0, 0);
    let grey = Color::RGB(127, 127, 127);
    let red = Color::RGB(255, 0, 0);
    let blue = Color::RGB(0, 0, 255);
    dc.canvas.set_draw_color(black);
    dc.canvas.fill_rect(sdl2::rect::Rect::new(0, 0, 3*cell_width, 3*cell_width)).unwrap();
    for i in 0..3 {
        for j in 0..3 {
            dc.canvas.set_draw_color(grey);
            dc.canvas.draw_rect(sdl2::rect::Rect::new((i * cell_width) as i32, (j * cell_width) as i32, cell_width, cell_width)).unwrap();
            match board.at(i as usize, j as usize).unwrap() {
                Cell::White => {
                    dc.canvas.set_draw_color(red);
                    dc.canvas.fill_rect(sdl2::rect::Rect::new((i * cell_width + cell_width/5) as i32, (j * cell_width + cell_width/5) as i32, 3*cell_width/5, 3*cell_width/5)).unwrap()
                },
                Cell::Black => {
                    dc.canvas.set_draw_color(blue);
                    dc.canvas.fill_rect(sdl2::rect::Rect::new((i * cell_width + cell_width/5) as i32, (j * cell_width + cell_width/5) as i32, 3*cell_width/5, 3*cell_width/5)).unwrap()
                },
                _ => {},
            }
        }
    }
}

fn coord_to_pos(dc: &DrawingContext, x: i32, y: i32) -> (usize, usize) {
    let cell_width = cmp::min(dc.width, dc.height)/3;
    ((x as u32/cell_width) as usize, (y as u32/cell_width) as usize)
}

fn main() {
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    let mut board = Board::new();
    let mut next_to_play = Cell::White;
    let mut finished = false;
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    board = Board::new();
                    next_to_play = Cell::White;
                    finished = false;
                }
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    if !finished {
                        let (i, j) = coord_to_pos(&dc, x, y);
                        // TODO: This could be outside of the grid if display is not square
                        board.set_pos(i, j, next_to_play).unwrap();
                        next_to_play = next_to_play.next();
                    }
                    if let Some(c) = board.winner() {
                        println!("{:?} won !", c);
                        finished = true;
                    }
                },
                _ => {},
            }
        }

        show_board(&mut dc, &board);
        dc.canvas.present();
    }
}
