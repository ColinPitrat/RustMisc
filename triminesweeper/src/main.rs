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

const SCREEN_WIDTH : u32 = 2000;
//const SCREEN_WIDTH : u32 = 1000;
const SCREEN_HEIGHT : u32 = 1000;
const CELL_WIDTH : u32 = 50;
//const CELL_WIDTH : u32 = 100;
const NB_MINES : u32 = 320;
//const NB_MINES : u32 = 40;

#[derive(PartialEq)]
enum GamePhase {
    Playing,
    Won,
    Lost,
}

struct Cell {
    x: u32,
    y: u32,
    revealed: bool,
    marked: bool,
    mine: bool,
    neighbours: usize,
    over: bool,
}

struct Grid {
    cells: Vec<Vec<Cell>>,
    width: u32,
    height: u32,
    phase: GamePhase,
    colors: [Color; 8]
}

impl Grid {
    fn new() -> Grid {
        let as2 = (f64::from(CELL_WIDTH) / f64::from(2).sqrt()) as u32;
        let width : u32 = 2*SCREEN_WIDTH / CELL_WIDTH - 1;
        let height : u32 = SCREEN_HEIGHT / as2;
        let mines_pos = Grid::gen_mines_pos(NB_MINES, width, height);
        let mut x = 0;
        let mut cells = vec!();
        let colors = [
            Color::RGB(0, 0, 255),
            Color::RGB(0, 192, 0),
            Color::RGB(255, 0, 0),
            Color::RGB(0, 208, 208),
            Color::RGB(255, 0, 255),
            Color::RGB(255, 128, 0),
            Color::RGB(128, 0, 255),
            Color::RGB(0, 64, 128),
        ];
        let mut result = Grid {
            cells: vec!(), width, height, phase: GamePhase::Playing, colors
        };
        while x < width {
            let mut y = 0;
            let mut cells_col = vec!();
            while y < height {
                let cell = Cell {
                    x, y,
                    revealed: false,
                    marked: false,
                    mine: mines_pos.contains(&(x, y)),
                    neighbours: result.count_neighbours(x, y, &mines_pos),
                    over: false,
                };
                cells_col.push(cell);
                y += 1;
            }
            cells.push(cells_col);
            x += 1;
        }
        result.cells = cells;
        result
    }

    fn gen_mines_pos(nb: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
        let mut x = 0;
        let mut candidates = vec!();
        while x < width {
            let mut y = 0;
            while y < height {
                candidates.push((x, y));
                y += 1;
            }
            x += 1;
        }

        let mut result = vec!();
        let mut n = 0;
        while n < nb {
            let r = rand::thread_rng().gen_range(0, candidates.len());
            result.push(candidates[r]);
            candidates.remove(r);
            n += 1;
        }
        result
    }

    fn neighbours(&self, x: u32, y:u32) -> Vec<(u32, u32)> {
        let mut result = vec!();
        if x > 0 {
            result.push((x-1, y));
        }
        if x+1 < self.width {
            result.push((x+1, y));
        }
        if (x + y) % 2 == 0 {
            if y+1 < self.height {
                result.push((x, y+1));
            }
        } else if y > 0 {
            result.push((x, y-1));
        }
        // Larger neighbourhood (cells touching just by a point)
        // Common to both cases:
        // (x-1, y-1), (x+1, y-1)
        // (x-2, y), (x+2, y),
        // (x-1, y+1), (x+1, y+1)
        if y > 0 {
            if x > 0 {
                result.push((x-1, y-1));
            }
            if x+1 < self.width {
                result.push((x+1, y-1));
            }
        }
        if x > 1 {
            result.push((x-2, y));
        }
        if x+2 < self.width {
            result.push((x+2, y));
        }
        if y+1 < self.height {
            if x > 0 {
                result.push((x-1, y+1));
            }
            if x+1 < self.width {
                result.push((x+1, y+1));
            }
        }
        if (x + y) % 2 == 0 {
            // Need also:
            // (x, y-1)
            // (x-2, y+1), (x+2, y+1)
            if y > 0 {
                result.push((x, y-1));
            }
            if y+1 < self.height {
                if x > 1 {
                    result.push((x-2, y+1));
                }
                if x+2 < self.width {
                    result.push((x+2, y+1));
                }
            }
        } else {
            // Need also:
            // (x-2, y-1), (x+2, y-1)
            // (x, y+1)
            if y > 0 {
                if x > 1 {
                    result.push((x-2, y-1));
                }
                if x+2 < self.width {
                    result.push((x+2, y-1));
                }
            }
            if y+1 < self.height {
                result.push((x, y+1));
            }
        }
        result
    }

    fn count_neighbours(&self, x: u32, y: u32, mines_pos: &[(u32, u32)]) -> usize {
        let mut nb = 0;
        for pos in self.neighbours(x, y) {
            if mines_pos.contains(&pos) {
                nb += 1;
            }
        }
        nb
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
            for line in self.cells.iter() {
                for cell in line.iter() {
                    let grey = Color::RGB(127, 127, 127);
                    let bg_color = if cell.marked {
                        Color::RGB(255, 0, 0)
                    } else if cell.revealed {
                        Color::RGB(255, 255, 255)
                    } else {
                        Color::RGB(192, 192, 192)
                    };
                    let as2 = (f64::from(CELL_WIDTH) / f64::from(2).sqrt()) as u32;
                    let offset = if (cell.x + cell.y) % 2 == 0 {
                        draw_up_triangle(dc, (cell.x*CELL_WIDTH/2) as i16, (cell.y*as2) as i16, CELL_WIDTH as i16, grey, Some(bg_color));
                        CELL_WIDTH/4
                    } else {
                        draw_down_triangle(dc, (cell.x*CELL_WIDTH/2) as i16, (cell.y*as2) as i16, CELL_WIDTH as i16, grey, Some(bg_color));
                        0
                    };
                    // TODO: Use cute pictures for mines & marks
                    if cell.mine && (cell.revealed || self.phase == GamePhase::Lost) {
                        let black = Color::RGB(0, 0, 0);
                        dc.canvas.set_draw_color(black);
                        let dy = CELL_WIDTH/8;
                        dc.canvas.fill_rect(sdl2::rect::Rect::new((cell.x * CELL_WIDTH/2 + 3*CELL_WIDTH/8) as i32, (cell.y * as2 + dy + offset) as i32, CELL_WIDTH/4, CELL_WIDTH/4)).unwrap();
                    }
                    if cell.neighbours > 0 && cell.revealed && !cell.mine {
                        let nb = font.render(&cell.neighbours.to_string()).solid(self.colors[cell.neighbours-1]).unwrap();
                        let nb = dc.texture_creator.create_texture_from_surface(nb).unwrap();
                        dc.canvas.copy(&nb, None, sdl2::rect::Rect::new((cell.x * CELL_WIDTH/2 + CELL_WIDTH/4) as i32, (cell.y * as2 + offset) as i32, CELL_WIDTH/2, CELL_WIDTH/2)).expect("Rendering number failed");
                    }
                }
            }
            for line in self.cells.iter() {
                for cell in line.iter() {
                    if cell.over {
                        let over_color = Color::RGB(255, 0, 255);
                        let bg_over_color = Some(Color::RGBA(255, 0, 255, 64));
                        let as2 = (f64::from(CELL_WIDTH) / f64::from(2).sqrt()) as u32;
                        if (cell.x + cell.y) % 2 == 0 {
                            draw_up_triangle(dc, (cell.x*CELL_WIDTH/2) as i16, (cell.y*as2) as i16, CELL_WIDTH as i16, over_color, bg_over_color);
                        } else {
                            draw_down_triangle(dc, (cell.x*CELL_WIDTH/2) as i16, (cell.y*as2) as i16, CELL_WIDTH as i16, over_color, bg_over_color);
                        }
                    }
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
        for line in self.cells.iter() {
            for cell in line.iter() {
                if ! cell.revealed && ! cell.mine {
                    return
                }
            }
        }
        self.phase = GamePhase::Won;
    }

    fn reset_over(&mut self) {
        for line in self.cells.iter_mut() {
            for cell in line.iter_mut() {
                cell.over = false;
            }
        }
    }

    fn over(&mut self, x: usize, y: usize) {
        if (x as u32) < self.width && (y as u32) < self.height {
            self.cells[x][y].over = true;
        }
    }

    fn mark(&mut self, x: usize, y: usize) {
        let cell = &mut self.cells[x][y];
        if ! cell.revealed {
            cell.marked = !cell.marked;
        }
    }

    fn reveal(&mut self, x: usize, y: usize) {
        let (cx, cy) = {
            let cell = &mut self.cells[x][y];
            if cell.mine {
                self.phase = GamePhase::Lost;
            }
            if ! cell.marked {
                cell.revealed = true;
            }
            if cell.neighbours != 0 {
                return;
            }
            (cell.x, cell.y)
        };
        for (nx, ny) in self.neighbours(cx, cy) {
            let (nx, ny) = (nx as usize, ny as usize);
            if !self.cells[nx][ny].revealed && !self.cells[nx][ny].marked {
                self.reveal(nx, ny);
            }
        }
    }
}

fn draw_up_triangle(dc: &DrawingContext, x: i16, y: i16, a: i16, color: Color, bg_color: Option<Color>) {
    let as2 = (f64::from(a) / f64::from(2).sqrt()) as i16;
    let xs = [x, x+a, x+a/2];
    let ys = [y+as2, y+as2, y];
    if let Some(bg_color) = bg_color {
        dc.canvas.filled_polygon(&xs, &ys, bg_color).unwrap();
    }
    dc.canvas.polygon(&xs, &ys, color).unwrap();
}

fn draw_down_triangle(dc: &DrawingContext, x: i16, y: i16, a: i16, color: Color, bg_color: Option<Color>) {
    let as2 = (f64::from(a) / f64::from(2).sqrt()) as i16;
    let xs = [x, x+a, x+a/2];
    let ys = [y, y, y+as2];
    if let Some(bg_color) = bg_color {
        dc.canvas.filled_polygon(&xs, &ys, bg_color).unwrap();
    }
    dc.canvas.polygon(&xs, &ys, color).unwrap();
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
 
    let window = video_subsystem.window("Tri-minesweeper", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
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
    sdl2::rect::Rect::new(x, y, inner.w as u32, inner.h as u32)
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

// TODO: More accurate way to compute coord from mouse pos
fn coord_to_pos(x: i32, y: i32) -> Option<(usize, usize)> {
    if x < (CELL_WIDTH as i32) / 4 || y < 0 {
        return None;
    }
    let as2 = (f64::from(CELL_WIDTH) / f64::from(2).sqrt()) as u32;
    let i = ((2*x as u32 - CELL_WIDTH/2) / CELL_WIDTH) as usize;
    let j = (y as u32 / as2) as usize;
    Some((i, j))
}

// TODO: timer
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
                        if let Some((i, j)) = coord_to_pos(x, y) {
                            grid.reveal(i, j);
                            grid.check_win();
                        }
                    }
                },
                Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Right, x, y, .. } => {
                    if ! grid.finished() {
                        if let Some((i, j)) = coord_to_pos(x, y) {
                            grid.mark(i, j);
                        }
                    }
                },
                _ => {}
            }
        }

        let state = event_pump.mouse_state();
        grid.reset_over();
        if let Some((i, j)) = coord_to_pos(state.x(), state.y()) {
            grid.over(i, j);
            for (x, y) in grid.neighbours(i as u32, j as u32) {
                grid.over(x as usize, y as usize);
            }
        }

        dc.canvas.present();
    }
}
