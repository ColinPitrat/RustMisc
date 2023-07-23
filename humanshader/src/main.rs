extern crate sdl2;

mod dc;
mod grid;

use dc::DrawingContext;
use grid::Grid;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn main() {
    let grid = Grid::new(71, 40, 20);
    let mut dc = DrawingContext::new(grid.display_width(), grid.display_height());
    let mut event_pump = dc.sdl_context.event_pump().unwrap();

    'game_loop: loop {
        grid.show(&mut dc);
        dc.blit_grid();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                _ => {},
            }
        }

        dc.canvas.present();
    }
}
