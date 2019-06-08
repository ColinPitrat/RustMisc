use sdl2::image::INIT_PNG;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::video::WindowContext;

pub struct DrawingContext {
    pub width: u32,
    pub height: u32,
    pub sdl_context: sdl2::Sdl,
    pub canvas: Canvas<Window>,
    pub texture_creator: TextureCreator<WindowContext>,
    pub ttf_context: Sdl2TtfContext,
}

impl DrawingContext {
    pub fn new(width: u32, height: u32) -> DrawingContext {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let _image_context = sdl2::image::init(INIT_PNG).unwrap();

        let window = video_subsystem.window("Evolution simulation", width, height)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let ttf_context = ttf::init().unwrap();

        DrawingContext{ width, height, sdl_context, canvas, texture_creator, ttf_context }
    }
}
