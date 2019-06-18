use png::HasParameters;
use sdl2::image::INIT_PNG;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::surface::Surface;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::video::WindowContext;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub struct DrawingContext<'a> {
    pub width: u32,
    pub height: u32,
    pub sdl_context: sdl2::Sdl,
    pub grid_canvas: Canvas<Surface<'a>>,
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl2TtfContext,
}

impl<'a> DrawingContext<'a> {
    pub fn new(width: u32, height: u32) -> DrawingContext<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();

        let window = video_subsystem.window("Evolution simulation", width, height)
            .position_centered()
            .build()
            .unwrap();
        //let grid = Surface::new(width, height, window.window_pixel_format()).unwrap();
        let grid = Surface::new(width, height, sdl2::pixels::PixelFormatEnum::ABGR8888).unwrap();
        let grid_canvas = Canvas::from_surface(grid).unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let ttf_context = ttf::init().unwrap();

        DrawingContext{ width, height, sdl_context, grid_canvas, canvas, texture_creator, ttf_context }
    }

    pub fn save_grid_bmp(&self, p: &Path) {
        self.grid_canvas.surface().save_bmp(p).unwrap();
    }

    pub fn save_grid_png(&self, p: &Path) {
        let file = File::create(p).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        // TODO: Adapt to the pixel format of the surface
        let grid_surface = self.grid_canvas.surface();
        println!("Pixel format: {:?}", grid_surface.pixel_format_enum());
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let data = self.grid_canvas.surface().without_lock().unwrap();
        //let data : [u8; self.width*self.height*3] = surface_data
        //println!("Data: {:?}", data);
        writer.write_image_data(data).unwrap();
        //panic!("Stop here !");
    }

    pub fn blit_grid(&mut self) {
        let grid_texture = self.texture_creator.create_texture_from_surface(self.grid_canvas.surface()).unwrap();
        self.canvas.copy(&grid_texture, None, None).unwrap();
    }
}
