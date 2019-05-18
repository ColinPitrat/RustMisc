extern crate sdl2; 
extern crate rand;

mod branch;
mod leaf;
mod dc;
mod point;
mod tree;

use dc::DrawingContext;
use point::Point;
use tree::{Tree,TreeParams};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::f64::consts::PI;

const SCREEN_WIDTH : u32 = 2000;
const SCREEN_HEIGHT : u32 = 1400;

fn white_background(canvas: &mut Canvas<Window>) {
    let white = Color::RGB(255, 255, 255);
    canvas.set_draw_color(white);
    canvas.clear();
}

fn create_tree(tree_params: &TreeParams) -> Tree {
    let x = f64::from(SCREEN_WIDTH/2);
    let y = f64::from(SCREEN_HEIGHT);
    Tree::new(tree_params, Point{x, y})
}

fn help() {
    println!("Commands available:");
    println!(" - H: help: display this in the console");
    println!(" - P: plus: increase the ratio of child size over parent size");
    println!(" - M: minus: decrease the ratio of child size over parent size");
    println!(" - W: wider: increase the angle");
    println!(" - N: narrower: decrease the angle");
    println!(" - L: larger: increase branches size");
    println!(" - S: smaller: decrease branches size");
    println!(" - R: randomer: increase randomness");
    println!(" - D: determinister: decrease randomness");
}

fn main() {
    help();
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tree_params = TreeParams{
        ratio: 0.75,
        angle: PI/4.0,
        size: 200.0,
        randomness: 0.1,
    };
    let mut tree = create_tree(&tree_params);
    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::H), .. } => {
                    help();
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    if tree_params.ratio < 1.0 {
                        tree_params.ratio += 0.05;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    if tree_params.ratio > 0.05 {
                        tree_params.ratio -= 0.05;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    if tree_params.angle < PI {
                        tree_params.angle += 0.05;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    if tree_params.angle > 0.05 {
                        tree_params.angle -= 0.05;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    if tree_params.size < 500.0 {
                        tree_params.size *= 1.1;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if tree_params.size > 10.0 {
                        tree_params.size /= 1.1;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if tree_params.randomness < 1.0 {
                        tree_params.randomness += 0.01;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if tree_params.randomness > 0.0 {
                        tree_params.randomness -= 0.01;
                        tree = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                    tree.falling_leaves();
                },
                _ => {}
            }
        }

        tree.animate();
        {
            white_background(&mut dc.canvas);
        }
        tree.display(&mut dc);
        dc.canvas.present();
    }
}
