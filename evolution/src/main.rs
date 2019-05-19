extern crate sdl2;

mod dc;
mod grid;
mod plant;

use dc::DrawingContext;
use grid::Grid;
use plant::Plants;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH : u32 = 2000;
const SCREEN_HEIGHT : u32 = 1400;
const CELL_WIDTH : u32 = 10;
const PLANTS_AT_START : u32 = 200;

fn main() {
    let mut dc = DrawingContext::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    let mut grid = Grid::new(SCREEN_WIDTH/CELL_WIDTH, SCREEN_HEIGHT/CELL_WIDTH, CELL_WIDTH);
    let mut plants = Plants::new(&mut grid, PLANTS_AT_START);

    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    'game_loop: loop {
        plants.to_grid(&mut grid);
        grid.show(&mut dc);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    plants.reproduce(&mut grid);
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new(SCREEN_WIDTH/CELL_WIDTH, SCREEN_HEIGHT/CELL_WIDTH, CELL_WIDTH);
                    plants = Plants::new(&mut grid, PLANTS_AT_START);
                },
                _ => {}
            }
        }

        dc.canvas.present();
    }
}
