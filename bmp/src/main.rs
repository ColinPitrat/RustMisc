use argh::FromArgs;
use std::path::{Path, PathBuf};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

mod bitreader;
mod bmp;

use bmp::*;
use std::fs;

#[derive(FromArgs)]
/// Bmp parsing demo
struct Args {
  #[argh(subcommand)]
  subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Display(BmpDisplay),
    Parse(BmpParse),
    MyTest(BmpMyTest),
}

impl Subcommand {
    fn run(self) {
        match self {
            Subcommand::Display(x) => x.run(),
            Subcommand::Parse(x) => x.run(),
            Subcommand::MyTest(x) => x.run(),
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name = "test")]
/// Various tests to debug stuff
pub struct BmpMyTest {
  #[argh(positional)]
  test: String,
  #[argh(positional)]
  a: f64,
  #[argh(positional)]
  b: f64,
  #[argh(positional)]
  c: f64,
}

impl BmpMyTest {
    fn run(self) {
        match self.test.as_str() {
            "srgb" => self.test_srgb(),
            "xyy" => self.test_xyy(),
            "xyz" => self.test_xyz(),
            "pal8v4" => self.test_pal8v4(),
            _ => println!("Unsupported command: {}", self.test)
        }
    }

    fn test_xyz(self) {
        let xyz = bmp::Colorf{ red: self.a, green: self.b, blue: self.c, alpha: 0.0};
        let srgb = bmp::xyz_to_srgb(xyz);
        let color = bmp::denormalize(srgb);
        println!("XYZ: {:?}\n -> SRGB: {:?}\n -> SRGB255: {:?}", xyz, srgb, color);
    }

    fn test_xyy(self) {
        let xyy = bmp::Colorf{ red: self.a, green: self.b, blue: self.c, alpha: 0.0};
        let xyz = bmp::xyY_to_xyz(xyy);
        let srgb = bmp::xyz_to_srgb(xyz);
        let color = bmp::denormalize(srgb);
        println!("xyY: {:?}\n -> XYZ: {:?}\n -> SRGB: {:?}\n -> SRGB255: {:?}", xyy, xyz, srgb, color);
    }

    fn test_srgb(self) {
        let srgb_endpoints = bmp::Endpoints{
            red: bmp::Endpoint {
                x: 0.4123,
                y: 0.2126,
                z: 0.0193,
            },
            green: bmp::Endpoint {
                x: 0.3576,
                y: 0.7152,
                z: 0.1192,
            },
            blue: bmp::Endpoint {
                x: 0.1805,
                y: 0.0722,
                z: 0.9506,
            },
        };
        let srgb = bmp::Colorf{ red: self.a, green: self.b, blue: self.c, alpha: 0.0 };
        let xyz = bmp::to_xyz(srgb, &srgb_endpoints);
        let srgb2 = bmp::xyz_to_srgb(xyz);
        let color = bmp::denormalize(srgb2);
        println!("SRGB: {:?}\n -> XYZ: {:?}\n -> SRGB: {:?}\n -> SRGB255: {:?}", srgb, xyz, srgb2, color);
    }

    fn test_pal8v4(self) {
        let srgb_endpoints = bmp::Endpoints{
            red: bmp::Endpoint {
                x: 0.64,
                y: 0.33,
                z: 0.03,
            },
            green: bmp::Endpoint {
                x: 0.3,
                y: 0.6,
                z: 0.1,
            },
            blue: bmp::Endpoint {
                x: 0.15,
                y: 0.06,
                z: 0.79,
            },
        };
        let srgb = bmp::Colorf{ red: self.a, green: self.b, blue: self.c, alpha: 0.0 };
        let xyz = bmp::to_xyz(srgb, &srgb_endpoints);
        let srgb2 = bmp::xyz_to_srgb(xyz);
        let color = bmp::denormalize(srgb2);
        println!("PAL8v4: {:?}\n -> XYZ: {:?}\n -> SRGB: {:?}\n -> SRGB255: {:?}", srgb, xyz, srgb2, color);
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name = "display")]
/// Display a BMP file
pub struct BmpDisplay {
  #[argh(positional)]
  input_paths: Vec<PathBuf>,
}

const SCREEN_WIDTH : u32 = 600;
const SCREEN_HEIGHT : u32 = 600;

impl BmpDisplay {
    fn fill_background(&self, canvas: &mut Canvas<Window>, color: Color) {
        canvas.set_draw_color(color);
        canvas.clear();
    }

    fn draw_bitmap_on_canvas(&self, bitmap: BmpFile, canvas: &mut Canvas<Window>) {
        for x in 0..bitmap.width as usize {
            for y in 0..bitmap.height.abs() as usize {
                canvas.pixel(x as i16, y as i16, bitmap.pixels[x][y]).unwrap();
            }
        }
    }

    fn run(self) {
        let sdl_context = sdl2::init().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("BMP Viewer", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();

        let mut idx = 0;
        let mut bg_color = Color::RGB(0, 0, 0);
        'display_loop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'display_loop
                    },
                    Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                        if bg_color.r == 0 {
                            bg_color = Color::RGB(255, 255, 255);
                        } else {
                            bg_color = Color::RGB(0, 0, 0);
                        }
                    },
                    Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                        if idx < self.input_paths.len() - 1 {
                            idx = idx + 1;
                        }
                        println!("{:?}", self.input_paths[idx]);
                    },
                    Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                        if idx > 0 {
                            idx = idx - 1;
                        }
                        println!("{:?}", self.input_paths[idx]);
                    },
                    _ => {}
                }
            }
            {
                self.fill_background(&mut canvas, bg_color);
            }
            // TODO: Do not reload the file at each loop, do this only when pressing n or p.
            let input_path = self.input_paths[idx].as_path();
            // TODO: Remove this hack, it's just to avoid ooming ...
            if input_path == Path::new("resources/reallybig.bmp") {
                println!("Skip loading {:?} to avoid OOMing", input_path);
                continue;
            }
            let input = fs::read(input_path);
            match input {
                Ok(inp) => {
                    let file = BmpFile::parse(&inp);
                    match file {
                        Ok((_, bitmap)) => {
                            self.draw_bitmap_on_canvas(bitmap, &mut canvas);
                        },
                            Err(e) => println!("Couldn't parse {:?}: {}", input_path, e),
                    }
                }
                Err(e) => println!("Couldn't open {:?}: {}", input_path, e),
            }
            canvas.present();
        }
    }
}

#[derive(FromArgs)]
#[argh(subcommand, name = "parse")]
/// Parse a BMP file and print content
pub struct BmpParse {
  #[argh(positional)]
  input_path: PathBuf,
}

impl BmpParse {
    fn run(self) {
        let input_path = self.input_path.as_path();
        // TODO: Remove this hack, it's just to avoid ooming ...
        if input_path == Path::new("resources/reallybig.bmp") {
            println!("Skip loading {:?} to avoid OOMing", input_path);
            return;
        }
        let input = fs::read(input_path);
        match input {
            Ok(inp) => {
                let file = BmpFile::parse(&inp);
                match file {
                    Ok((_, bitmap)) => {
                        println!("{:?}", input_path);
                        println!("{:#?}", bitmap);
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
