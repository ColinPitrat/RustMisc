extern crate sdl2; 

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::image::INIT_PNG;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color,PixelFormatEnum};
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::path::Path;

const SCREEN_WIDTH : u32 = 1360;
const SCREEN_HEIGHT : u32 = 1360;
const RADIUS : f64 = 50.0;
const POINT_RADIUS : f64 = 5.0;
const MARGIN : f64 = 20.0;
const SPEED_STEP : f64 = 0.01;
const DELTA_STEP : f64 = 0.01;

struct DrawingContext<'a> {
    sdl_context: sdl2::Sdl,
    back_canvas: Canvas<Surface<'a>>,
    front_canvas: Canvas<Surface<'a>>,
    canvas: Canvas<Window>,
    texture_creator: TextureCreator<WindowContext>,
    draw_lines: bool,
}

impl<'a> DrawingContext<'a> {
    fn new() -> DrawingContext<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();

        let window = video_subsystem.window("Lissajous curves", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();

        let back = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, window.window_pixel_format()).unwrap();
        let front = Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::RGBA8888).unwrap();
        let back_canvas = Canvas::from_surface(back).unwrap();
        let front_canvas = Canvas::from_surface(front).unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        DrawingContext{ sdl_context, back_canvas, front_canvas, canvas, texture_creator, draw_lines: true }
    }

    // TODO: I suspect the canvas -> surface -> texture -> copy is what makes it so slow.
    // I think there's a way to draw on the texture directly which should be faster.
    fn blit(&mut self) {
        self.back_canvas.surface().save_bmp(Path::new("test.bmp")).unwrap();
        let background = self.texture_creator.create_texture_from_surface(self.back_canvas.surface()).unwrap();
        let foreground = self.texture_creator.create_texture_from_surface(self.front_canvas.surface()).unwrap();
        self.canvas.copy(&background, None, None).unwrap();
        self.canvas.copy(&foreground, None, None).unwrap();
    }
}

struct Circle {
    r: f64,
    x: f64,
    y: f64,
    t: f64,
    speed: f64,
    color: Color,
    horizontal: bool,
}

impl Circle {
    fn new(x: f64, y: f64, t: f64, speed: f64, color: Color, horizontal: bool) -> Circle {
        Circle {
            r: RADIUS,
            t, x, y, speed,
            color,
            horizontal,
        }
    }

    fn show(&self, dc: &mut DrawingContext) {
        dc.front_canvas.filled_circle(self.point_x() as i16, self.point_y() as i16, POINT_RADIUS as i16, self.color).unwrap();
        if dc.draw_lines {
            let x1 = if self.horizontal { self.point_x() as i32 } else { 0 };
            let x2 = if self.horizontal { self.point_x() as i32 } else { SCREEN_WIDTH as i32 };
            let y1 = if self.horizontal { 0 } else { self.point_y() as i32 };
            let y2 = if self.horizontal { SCREEN_HEIGHT as i32 } else { self.point_y() as i32 };
            dc.front_canvas.draw_line(Point::new(x1, y1), Point::new(x2, y2)).unwrap();
        }
        dc.back_canvas.filled_circle(self.point_x() as i16, self.point_y() as i16, 1 as i16, self.color).unwrap();
    }

    fn update(&mut self) {
        self.t += self.speed;
    }

    fn point_x(&self) -> f64 {
        self.x + self.r * self.t.cos()
    }

    fn point_y(&self) -> f64{
        self.y + self.r * self.t.sin()
    }
}

fn fill_background(canvas: &mut Canvas<Surface>, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
}

fn black_background(canvas: &mut Canvas<Surface>) {
    fill_background(canvas, Color::RGB(0, 0, 0));
}

fn empty_background(canvas: &mut Canvas<Surface>) {
    fill_background(canvas, Color::RGBA(0, 0, 0, 0));
}

struct Lissajous {
    x_circles: Vec<Circle>,
    y_circles: Vec<Circle>,
}

impl Lissajous {
    fn new(delta : f64) -> Lissajous {
        Lissajous { 
            x_circles: Lissajous::init_x_circles(delta),
            y_circles: Lissajous::init_y_circles(0.0),
        }
    }

    fn circles_colors() -> Vec<Color> {
        vec!(
                Color::RGB(255, 0, 0),
                Color::RGB(0, 255, 0),
                Color::RGB(0, 0, 255),
                Color::RGB(0, 255, 255),
                Color::RGB(255, 0, 255),
                Color::RGB(255, 255, 0),
                Color::RGB(127, 0, 0),
                Color::RGB(0, 127, 0),
                Color::RGB(0, 0, 127),
                Color::RGB(0, 127, 127),
                Color::RGB(127, 0, 127),
                Color::RGB(127, 127, 0),
                Color::RGB(0, 255, 127),
                Color::RGB(0, 127, 255),
                Color::RGB(255, 0, 127),
                Color::RGB(127, 0, 255),
                Color::RGB(255, 127, 0),
                Color::RGB(127, 255, 0),
            )
    }

    // TODO: Deduplciate between init_x_circles and init_y_circles
    fn init_x_circles(delta: f64) -> Vec<Circle> {
        let mut circles = vec!();
        let colors = Lissajous::circles_colors();

        let mut x = 3.0*RADIUS + 2.0*MARGIN;
        let y = RADIUS + MARGIN;
        let mut s = SPEED_STEP;
        let mut i = 0;
        while x + RADIUS + MARGIN <= f64::from(SCREEN_WIDTH) {
            circles.push(Circle::new(x, y, delta, s, colors[i], true));
            x += 2.0*RADIUS + MARGIN;
            s += SPEED_STEP;
            i += 1;
            if i >= colors.len() {
                i -= colors.len();
            }
        }

        circles
    }

    fn init_y_circles(delta: f64) -> Vec<Circle> {
        let mut circles = vec!();
        let colors = Lissajous::circles_colors();

        let x = RADIUS + MARGIN;
        let mut y = 3.0*RADIUS + 2.0*MARGIN;
        let mut s = SPEED_STEP;
        let mut i = 0;
        while y + RADIUS + MARGIN <= f64::from(SCREEN_HEIGHT) {
            circles.push(Circle::new(x, y, delta, s, colors[i], false));
            y += 2.0*RADIUS + MARGIN;
            s += SPEED_STEP;
            i += 1;
            if i >= colors.len() {
                i -= colors.len();
            }
        }

        circles
    }

    fn show(&self, dc: &mut DrawingContext) {
        let white = Color::RGB(255, 255, 255);
        for c1 in self.x_circles.iter() {
            for c2 in self.y_circles.iter() {
                dc.front_canvas.filled_circle(c1.point_x() as i16, c2.point_y() as i16, POINT_RADIUS as i16, white).unwrap();
                dc.back_canvas.filled_circle(c1.point_x() as i16, c2.point_y() as i16, 1 as i16, white).unwrap();
            }
        }
        for c in self.x_circles.iter() {
            c.show(dc);
        }
        for c in self.y_circles.iter() {
            c.show(dc);
        }
        dc.blit();
        dc.canvas.present();
    }

    fn update(&mut self) {
        for c in self.x_circles.iter_mut() {
            c.update();
        }
        for c in self.y_circles.iter_mut() {
            c.update();
        }
    }

}

fn main() {
    let mut dc = DrawingContext::new();

    let mut delta = 0.0;
    let mut lissajous = Lissajous::new(delta);
    black_background(&mut dc.back_canvas);
    'game_loop: loop {
        empty_background(&mut dc.front_canvas);

        let mut event_pump = dc.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    dc.draw_lines = !dc.draw_lines;
                }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    lissajous = Lissajous::new(delta);
                    black_background(&mut dc.back_canvas);
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    delta += DELTA_STEP;
                    println!("Delta = {}", delta);
                    lissajous = Lissajous::new(delta);
                    black_background(&mut dc.back_canvas);
                }
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    delta -= DELTA_STEP;
                    println!("Delta = {}", delta);
                    lissajous = Lissajous::new(delta);
                    black_background(&mut dc.back_canvas);
                }
                _ => {}
            }
        }

        lissajous.show(&mut dc);
        lissajous.update();
    }
}
