use argh::FromArgs;
use std::path::PathBuf;
use std::fs;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

mod ttf;

use ttf::*;

#[derive(FromArgs)]
/// Ttf parsing demo
struct Args {
  #[argh(subcommand)]
  subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Info(TtfInfo),
    Display(TtfDisplay),
}

impl Subcommand {
    fn run(self) {
        match self {
            Subcommand::Info(x) => x.run(),
            Subcommand::Display(x) => x.run(),
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name = "info")]
/// Display information about a TTF file
pub struct TtfInfo {
  #[argh(positional)]
  input_path: PathBuf,
}

impl TtfInfo {
    fn run(self) {
        let input_path = self.input_path.as_path();
        let input = fs::read(input_path);
        match input {
            Ok(inp) => {
                let file = TtfFile::parse(&inp);
                match file {
                    Ok((_, ttf)) => {
                        println!("{:?}", input_path);
                        println!("{:#?}", ttf);
                    },
                    Err(e) => println!("Couldn't parse {:?}: {}", input_path, e),
                }
            }
            Err(e) => println!("Couldn't open {:?}: {}", input_path, e),
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name = "display")]
/// Display glyphs from a TTF file
pub struct TtfDisplay {
  #[argh(positional)]
  input_path: PathBuf,
}

const SCREEN_WIDTH : u32 = 800;
const SCREEN_HEIGHT : u32 = 800;
const MARGIN : u32 = 50;
const PT_SIZE : u32 = 4;

impl TtfDisplay {
    fn fill_background(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.clear();
    }

    fn quadratic(&self, a1: f64, a2: f64, a3: f64, t: f64) -> f64 {
        (a3 - 2.0*a2 + a1)*t*t + 2.0*(a2-a3)*t + a3
    }

    fn draw_bezier_quadratic(&self, p1: &Point, p2: &Point, p3: &Point, color: Color, canvas: &mut Canvas<Window>) {
        let (x1, x2, x3) = (p1.x as f64, p2.x as f64, p3.x as f64);
        let (y1, y2, y3) = (p1.y as f64, p2.y as f64, p3.y as f64);
        let mut t = 0.0;
        let mut dt = 0.1;
        let mut first = true;
        let (mut px, mut py) = (0.0, 0.0);
        let (mut x, mut y);
        loop {
            if t > 1.0 {
                t = 1.0;
            }
            // Reduce the value of dt if it's too big to have adjacent pixels
            x = self.quadratic(x1, x2, x3, t);
            y = self.quadratic(y1, y2, y3, t);
            canvas.pixel(x as i16, y as i16, color).unwrap();
            if t == 1.0 {
                break;
            }
            px = x;
            py = y;
            loop {
                let new_t = t + dt;
                x = self.quadratic(x1, x2, x3, new_t);
                y = self.quadratic(y1, y2, y3, new_t);
                if (x - px).abs() + (y - py).abs() <= 1.0 {
                    break;
                }
                dt = dt / 2.0;
            }
            t += dt;
        }
    }

    // TODO: Draw bezier curves. For points x1, x2 and x3, the curve is for t in [0,1]:
    // (x3 - 2*x2 + x1)*t^2 + 2*(x2 - x3)*t + x3
    fn draw_glyph_on_canvas(&self, info: &str, glyph: &Glyph, canvas: &mut Canvas<Window>) {
        let sdl_ttf_context = sdl2::ttf::init().unwrap();
        let font = sdl_ttf_context.load_font("./resources/DejaVuSans.ttf", 20).unwrap();
        let text_color = Color::RGB(255, 255, 255);
        let oncurve_color = Color::RGB(255, 0, 0);
        let offcurve_color = Color::RGB(0, 0, 255);
        let first_color = Color::RGB(255, 255, 0);
        let line_color = Color::RGB(128, 128, 128);

        // Testing Bezier curves
        /*
        let x1 = 100;
        let y1 = 100;
        let x2 = 100;
        let y2 = 500;
        let x3 = 500;
        let y3 = 300;
        canvas.set_draw_color(oncurve_color);
        canvas.fill_rect(Rect::new(x1, y1, 4, 4)).unwrap();
        canvas.fill_rect(Rect::new(x2, y2, 4, 4)).unwrap();
        canvas.fill_rect(Rect::new(x3, y3, 4, 4)).unwrap();
        self.draw_bezier_quadratic(&Point::new(x1, y1), &Point::new(x2, y2), &Point::new(x3, y3), line_color, canvas);
        return;
        */

        match &glyph.glyph_data {
            GlyphData::SimpleGlyph{xs, ys, contours, ..} => {
                let x_min = xs.iter().min().unwrap();
                let x_max = xs.iter().max().unwrap();
                let y_min = ys.iter().min().unwrap();
                let y_max = ys.iter().max().unwrap();
                for c in contours.iter() {
                    let mut ppx = 0;
                    let mut ppy = 0;
                    let mut pppx = 0;
                    let mut pppy = 0;
                    let mut bezier = false;
                    let mut s = 0;
                    let mut on_curve = true;
                    for p in c.points.iter() {
                        let px = MARGIN as i32 + (SCREEN_WIDTH - 2*MARGIN) as i32 * (p.x - x_min) / (x_max - x_min);
                        let py = MARGIN as i32 + (SCREEN_HEIGHT - 2*MARGIN) as i32 * (y_max - p.y) / (y_max - y_min);
                        if p.on_curve {
                            canvas.set_draw_color(oncurve_color);
                        } else {
                            canvas.set_draw_color(offcurve_color);
                        }
                        if s == 0 {
                            canvas.set_draw_color(first_color);
                        }
                        canvas.fill_rect(Rect::new(px - PT_SIZE as i32/2, py - PT_SIZE as i32/2, PT_SIZE, PT_SIZE)).unwrap();
                        if s == 0 {
                            s = 1;
                        } else if s == 1 {
                            if p.on_curve {
                                canvas.set_draw_color(line_color);
                                canvas.draw_line(Point::new(ppx, ppy), Point::new(px, py)).unwrap();
                            } else {
                                s = 2;
                            }
                        } else if s == 2 {
                            if p.on_curve {
                                self.draw_bezier_quadratic(&Point::new(pppx, pppy), &Point::new(ppx, ppy), &Point::new(px, py), line_color, canvas);
                                s = 1;
                            } else {
                                let nx = (ppx + px) / 2;
                                let ny = (ppy + py) / 2;
                                self.draw_bezier_quadratic(&Point::new(pppx, pppy), &Point::new(ppx, ppy), &Point::new(nx, ny), line_color, canvas);
                                ppx = nx;
                                ppy = ny;
                            }
                        }
                        pppx = ppx;
                        pppy = ppy;
                        ppx = px;
                        ppy = py;
                        on_curve = p.on_curve;
                    }
                    // Close the contour
                    canvas.set_draw_color(line_color);
                    if let Some(p) = c.points.first() {
                        let px = MARGIN as i32 + (SCREEN_WIDTH - 2*MARGIN) as i32 * (p.x - x_min) / (x_max - x_min);
                        let py = MARGIN as i32 + (SCREEN_HEIGHT - 2*MARGIN) as i32 * (y_max - p.y) / (y_max - y_min);
                        if on_curve {
                            canvas.draw_line(Point::new(ppx, ppy), Point::new(px, py)).unwrap();
                        } else {
                            self.draw_bezier_quadratic(&Point::new(pppx, pppy), &Point::new(ppx, ppy), &Point::new(px, py), line_color, canvas);
                        }
                    }
                }
            }
            _ => {},
        };
        let help = font.render(&format!("{}, {} contours", info, glyph.nb_contours)).solid(text_color).unwrap();
        let r = help.rect();
        let texture_creator = canvas.texture_creator();
        let help = texture_creator.create_texture_from_surface(help).unwrap();
        canvas.copy(&help, None, r).expect("Rendering text failed");
    }

    fn run(self) {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("TTF Viewer", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        let input_path = self.input_path.as_path();
        let input = fs::read(input_path);
        match input {
            Ok(inp) => {
                let file = TtfFile::parse(&inp);
                match file {
                    Ok((_, ttf)) => {
                        println!("{:?}", input_path);
                        //println!("{:#?}", ttf);
                        let mut glyph_keys = ttf.glyph_table.as_ref().unwrap().glyphs.keys().collect::<Vec<_>>();
                        glyph_keys.sort();
                        let mut idx = 0;
                        'display_loop: loop {
                            for event in event_pump.poll_iter() {
                                match event {
                                    Event::Quit {..} |
                                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                            break 'display_loop
                                        },
                                        Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                                            if idx < (glyph_keys.len() - 1) as u32 {
                                                idx = idx + 1;
                                            }
                                        },
                                        Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                                            if idx > 0 {
                                                idx = idx - 1;
                                            }
                                        },
                                        _ => {}
                                }
                            }
                            {
                                let bg_color = Color::RGB(0, 0, 0);
                                self.fill_background(&mut canvas, bg_color);
                            }
                            let key = glyph_keys[idx as usize];
                            let glyph = &ttf.glyph_table.as_ref().unwrap().glyphs[key];
                            let info = format!("{} - {}", key, idx);
                            self.draw_glyph_on_canvas(&info, glyph, &mut canvas);
                            canvas.present();
                        }
                    },
                    Err(e) => println!("Couldn't parse {:?}: {}", input_path, e),
                }
            }
            Err(e) => println!("Couldn't open {:?}: {}", input_path, e),
        }
    }
}

fn main() {
    argh::from_env::<Args>().subcommand.run();
}
