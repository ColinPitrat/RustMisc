extern crate sdl2; 
extern crate rand;

use rand::Rng;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::image::INIT_PNG;
use sdl2::pixels::Color;
use sdl2::render::TextureCreator;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::cmp;

// Must be (3k+2)*CELL_WIDTH for a nice fit
const SCREEN_WIDTH : u32 = 1040;
// Must be n*sqrt(3)*CELL_WIDTH for a nice fit
const SCREEN_HEIGHT : u32 = 1020;
const CELL_WIDTH : u32 = 40;
const NB_MINES : u32 = 40;

#[derive(PartialEq)]
enum GamePhase {
    Playing,
    Won,
    Lost,
}

struct Cell {
    id: usize,
    x: u32,
    y: u32,
    revealed: bool,
    marked: bool,
    mine: bool,
    neighbours: usize,
}

impl Cell {
    fn contains(&self, x: i32, y: i32) -> bool {
        let s3a = ((3 as f64).sqrt() * (CELL_WIDTH as f64)) as u32;
        // First check bounding rect
        if x < self.x as i32 {
            return false;
        }
        if y < self.y as i32 {
            return false;
        }
        if x > (self.x + 2*CELL_WIDTH) as i32 {
            return false;
        }
        if y > (self.y + s3a) as i32 {
            return false;
        }
        // Then check corners
        let dx = (x - self.x as i32) as f64;
        let dy = (y - self.y as i32) as f64;
        let s3a = (3 as f64).sqrt() * (CELL_WIDTH as f64);
        // Above y=-sqrt(3)*x + a*sqrt(3)/2 is not this hexagon (top-left corner)
        if dy + (3 as f64).sqrt()*dx < s3a/2.0 {
            return false;
        }
        // Neither is above y=sqrt(3)*x - 3*a*sqrt(3)/2 (top-right corner)
        if dy - (3 as f64).sqrt()*dx < -3.0*s3a/2.0 {
            return false;
        }
        // Nor below y=sqrt(3)*x + a*sqrt(3)/2 (bottom-left corner)
        if dy - (3 as f64).sqrt()*dx > s3a/2.0 {
            return false;
        }
        // Nor below y=-sqrt(3)*x + 5*a*sqrt(3)/2 (bottom-right corner)
        if dy + (3 as f64).sqrt()*dx > 5.0*s3a/2.0 {
            return false;
        }
        return true;
    }
}

struct Grid {
    cells: Vec<Cell>,
    phase: GamePhase,
    colors: [Color; 6]
}

impl Grid {
    fn new() -> Grid {
        let colors = [
            Color::RGB(0, 0, 255),
            Color::RGB(0, 192, 0),
            Color::RGB(255, 0, 0),
            Color::RGB(0, 208, 208),
            Color::RGB(255, 0, 255),
            Color::RGB(255, 128, 0),
        ];
        let mut grid = Grid {
            cells: vec!(),
            phase: GamePhase::Playing,
            colors,
        };
        grid.create_cells();
        grid.place_mines();
        grid.count_neighbours();
        grid
    }

    fn create_cells(&mut self) {
        let s3a = ((3 as f64).sqrt() * (CELL_WIDTH as f64)) as u32;
        let mut y = 0;
        let mut offset_x = 0;
        let dx = 3*CELL_WIDTH;
        let dy = s3a/2;
        let mut id = 0;
        while y + 2*dy <= SCREEN_HEIGHT {
            let mut x = offset_x;
            while x + 2*CELL_WIDTH <= SCREEN_WIDTH {
                let cell = Cell {
                    id, x, y, revealed: false, marked: false, mine: false, neighbours: 0
                };
                self.cells.push(cell);
                x += dx;
                id += 1;
            }
            y += dy;
            if offset_x > 0 {
                offset_x = 0;
            } else {
                offset_x = 3*CELL_WIDTH/2;
            }
        }
    }

    fn place_mines(&mut self) {
        let mut n = 0;
        let mut looped = false;
        while n < NB_MINES {
            let mut r = rand::thread_rng().gen_range(0, self.cells.len());
            while self.cells[r].mine {
                r += 1;
                if r >= self.cells.len() {
                    if looped {
                        panic!("More mines than available cells !");
                    }
                    r = 0;
                    looped = true;
                }
            }
            self.cells[r].mine = true;
            n += 1;
        }
    }

    fn count_neighbours(&mut self) {
        let mut neighbours = vec!();
        for _ in self.cells.iter() {
            neighbours.push(0);
        }
        for cell in self.cells.iter() {
            if cell.mine {
                for ncell in self.cells.iter() {
                    let dx = (cell.x as i32 - ncell.x as i32).abs();
                    let dy = (cell.y as i32 - ncell.y as i32).abs();
                    // Neighbours are cells that are at less than sqrt(3)*a from this cell, so square
                    // distance is <= 3. Plus some margin ...
                    if dx*dx + dy*dy <= 310*CELL_WIDTH as i32 {
                        neighbours[ncell.id] += 1;
                    }
                }
            }
        }
        for cell in self.cells.iter_mut() {
            cell.neighbours = neighbours[cell.id];
        }
    }

    fn end_screen(&self, dc: &mut DrawingContext, text: &str) {
        let screen_rect = sdl2::rect::Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
        let blue = Color::RGB(0, 0, 255);
        let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 50).unwrap();
        let big_font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 100).unwrap();
        let lost = big_font.render(text).solid(blue).unwrap();
        let mut r1 = centered_rect(&lost.rect(), &screen_rect);
        let lost = dc.texture_creator.create_texture_from_surface(lost).unwrap();
        let help = font.render("Press R to play again.").solid(blue).unwrap();
        let mut r2 = centered_rect(&help.rect(), &screen_rect);
        let help = dc.texture_creator.create_texture_from_surface(help).unwrap();
        r1.y -= r1.h / 2;
        r2.y += r2.h / 2;
        // TODO: Understand why alpha doesn't work
        let text_bg = Color::RGBA(128, 128, 128, 64);
        let mut text_rect = bounding_rect(&r1, &r2);
        text_rect.x -= 10;
        text_rect.y -= 10;
        text_rect.w += 20;
        text_rect.h += 20;
        dc.canvas.set_draw_color(text_bg);
        dc.canvas.fill_rect(text_rect).unwrap();
        dc.canvas.copy(&lost, None, r1).expect("Rendering text failed");
        dc.canvas.copy(&help, None, r2).expect("Rendering text failed");
    }

    fn show(&self, dc: &mut DrawingContext) {
        {
            // TODO: loading the font at each call to show & end_screen is ugly ... Find a way to
            // put it in the dc.
            let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 50).unwrap();
            for cell in self.cells.iter() {
                let mut color = Color::RGB(192, 192, 192);
                if cell.revealed {
                    color = Color::RGB(255, 255, 255);
                }
                if cell.marked {
                    color = Color::RGB(255, 0, 0);
                }
                draw_hex(dc, cell.x as i16, cell.y as i16, CELL_WIDTH as i16, color);
                if cell.mine && (cell.revealed || self.phase == GamePhase::Lost) {
                    let black = Color::RGB(0, 0, 0);
                    dc.canvas.set_draw_color(black);
                    dc.canvas.fill_rect(sdl2::rect::Rect::new((cell.x + 3*CELL_WIDTH/4) as i32, (cell.y + 62*CELL_WIDTH/100) as i32, CELL_WIDTH/2, CELL_WIDTH/2)).unwrap();
                }
                if cell.neighbours > 0 && cell.revealed && !cell.mine {
                    let nb = font.render(&cell.neighbours.to_string()).solid(self.colors[cell.neighbours-1]).unwrap();
                    let nb = dc.texture_creator.create_texture_from_surface(nb).unwrap();
                    dc.canvas.copy(&nb, None, sdl2::rect::Rect::new((cell.x + CELL_WIDTH/2) as i32, (cell.y + 32*CELL_WIDTH/100) as i32, CELL_WIDTH, CELL_WIDTH)).expect("Rendering number failed");
                }
            }
        }
        match self.phase {
            GamePhase::Lost => {
                self.end_screen(dc, "You died !");
            },
            GamePhase::Won => {
                self.end_screen(dc, "Well done !");
            },
            GamePhase::Playing => {
                // TODO: Display timer - nb remaining mines
            },
        }
    }

    fn finished(&self) -> bool {
        match self.phase {
            GamePhase::Lost => true,
            GamePhase::Won => true,
            GamePhase::Playing => false,
        }
    }

    fn check_win(&mut self) {
        for cell in self.cells.iter() {
            if ! cell.revealed && ! cell.mine {
                return
            }
        }
        self.phase = GamePhase::Won;
    }

    fn reveal(&mut self, x: i32, y: i32) {
        let mut id_revealed: Option<usize> = None;
        for cell in self.cells.iter_mut() {
            if cell.contains(x, y) && !cell.marked && !cell.revealed {
                if cell.mine {
                    self.phase = GamePhase::Lost;
                }
                cell.revealed = true;
                if cell.neighbours == 0 {
                    id_revealed = Some(cell.id);
                }
                break;
            }
        }
        if let Some(id) = id_revealed {
            let mut to_reveal = vec!();
            for cell in self.cells.iter() {
                if cell.id == id {
                    for ncell in self.cells.iter() {
                        let dx = (cell.x as i32 - ncell.x as i32).abs();
                        let dy = (cell.y as i32 - ncell.y as i32).abs();
                        // Neighbours are cells that are at less than sqrt(3)*a from this cell, so square
                        // distance is <= 3. Plus some margin ...
                        if dx*dx + dy*dy <= 310*CELL_WIDTH as i32 {
                            to_reveal.push(((ncell.x + CELL_WIDTH/2) as i32, (ncell.y + CELL_WIDTH/2) as i32));
                        }
                    }
                }
            }
            for pos in to_reveal {
                self.reveal(pos.0, pos.1);
            }
        }
    }

    fn mark(&mut self, x: i32, y: i32) {
        for cell in self.cells.iter_mut() {
            if cell.contains(x, y) && !cell.revealed {
                cell.marked = !cell.marked;
                break;
            }
        }
    }
}

fn draw_hex(dc: &DrawingContext, x: i16, y: i16, a: i16, color: Color) {
    let s3a = ((3 as f64).sqrt() * (a as f64)) as i16;
    let xs = [x+a/2, x+3*a/2, x+2*a, x+3*a/2, x+a/2, x];
    let ys = [y, y, y+s3a/2, y+s3a, y+s3a, y+s3a/2];
    dc.canvas.filled_polygon(&xs, &ys, color).unwrap();
    let grey = Color::RGB(127, 127, 127);
    dc.canvas.polygon(&xs, &ys, grey).unwrap();
}

struct DrawingContext {
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    ttf_context: Sdl2TtfContext,
}

fn init_dc() -> DrawingContext {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(INIT_PNG).unwrap();
 
    let window = video_subsystem.window("AlThreat", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();
 
    let canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    
    let ttf_context = ttf::init().unwrap();

    DrawingContext{ sdl_context, canvas, texture_creator, ttf_context }
}

fn centered_rect(inner: &sdl2::rect::Rect, outer: &sdl2::rect::Rect) -> sdl2::rect::Rect {
    let x = (outer.w - inner.w) / 2;
    let y = (outer.h - inner.h) / 2;
    let result = sdl2::rect::Rect::new(x, y, inner.w as u32, inner.h as u32);
    result
}

fn bounding_rect(r1: &sdl2::rect::Rect, r2: &sdl2::rect::Rect) -> sdl2::rect::Rect {
    let x = cmp::min(r1.x, r2.x);
    let y = cmp::min(r1.y, r2.y);
    let left_x = cmp::max(r1.x + r1.w, r2.x + r2.w);
    let bottom_y = cmp::max(r1.y + r1.h, r2.y + r2.h);
    sdl2::rect::Rect::new(x, y, (left_x-x) as u32, (bottom_y-y) as u32)
}

fn grey_background(canvas: &mut Canvas<Window>) {
    let grey = Color::RGB(64, 64, 64);
    canvas.set_draw_color(grey);
    canvas.clear();
}

fn main() {
    let mut dc = init_dc();
    let mut grid = Grid::new();

    let mut event_pump = dc.sdl_context.event_pump().unwrap();
    'game_loop: loop {
        grey_background(&mut dc.canvas);
        grid.show(&mut dc);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new();
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    if ! grid.finished() {
                        grid.reveal(x, y);
                        grid.check_win();
                    }
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Right, x, y, .. } => {
                    if ! grid.finished() {
                        grid.mark(x, y);
                    }
                },
                _ => {}
            }
        }

        dc.canvas.present();
    }
}
