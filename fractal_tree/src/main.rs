extern crate sdl2; 
extern crate rand;

mod dc;
mod point;
mod branch;

use branch::Branch;
use dc::DrawingContext;
use point::Point;
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

#[derive(Debug)]
struct TreeParams {
    ratio: f64,
    angle: f64,
    size: f64,
    randomness: f64,
}

fn create_tree(tree_params: &TreeParams) -> Vec<Branch> {
    println!("Create tree with {:#?}", tree_params);
    let mid_x = f64::from(SCREEN_WIDTH/2);
    let mut branches = vec!(
            Branch::new(
                Point::new(mid_x, f64::from(SCREEN_HEIGHT)),
                Point::new(mid_x, f64::from(SCREEN_HEIGHT) - tree_params.size),
                tree_params.randomness)
    );
    loop {
        let mut to_add = vec!();
        for branch in branches.iter_mut() {
            to_add.extend(branch.children(tree_params.ratio, tree_params.angle));
        }
        if to_add.is_empty() {
            break;
        }
        branches.extend(to_add);
    }
    branches
}

fn main() {
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tree_params = TreeParams{
        ratio: 0.75,
        angle: PI/4.0,
        size: 200.0,
        randomness: 0.1,
    };
    let mut branches = create_tree(&tree_params);
    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    'game_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    if tree_params.ratio < 1.0 {
                        tree_params.ratio += 0.05;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    if tree_params.ratio > 0.05 {
                        tree_params.ratio -= 0.05;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    if tree_params.angle < PI {
                        tree_params.angle += 0.05;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    if tree_params.angle > 0.05 {
                        tree_params.angle -= 0.05;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    if tree_params.size < 500.0 {
                        tree_params.size *= 1.1;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if tree_params.size > 10.0 {
                        tree_params.size /= 1.1;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    if tree_params.randomness < 1.0 {
                        tree_params.randomness += 0.01;
                        branches = create_tree(&tree_params);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if tree_params.randomness > 0.0 {
                        tree_params.randomness -= 0.01;
                        branches = create_tree(&tree_params);
                    }
                },
                _ => {}
            }
        }

        {
            white_background(&mut dc.canvas);
        }
        for branch in branches.iter() {
            branch.display(&mut dc);
        }
        dc.canvas.present();
    }
}
