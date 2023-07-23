use sdl2::image::INIT_PNG;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::surface::{Surface,SurfaceContext};
use sdl2::ttf::{self, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::video::WindowContext;

pub struct DrawingContext<'a> {
    pub width: u32,
    pub height: u32,
    pub sdl_context: sdl2::Sdl,
    pub graph_canvas: Canvas<Surface<'a>>,
    pub graph_texture_creator: TextureCreator<SurfaceContext<'a>>,
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

        let window = video_subsystem.window("Human shader", width, height)
            .position_centered()
            .build()
            .unwrap();
        //let grid = Surface::new(width, height, PixelFormatEnum::ABGR8888).unwrap();
        let grid = Surface::new(width, height, window.window_pixel_format()).unwrap();
        let grid_canvas = Canvas::from_surface(grid).unwrap();
        let graph = Surface::new(width, height, window.window_pixel_format()).unwrap();
        let graph_canvas = Canvas::from_surface(graph).unwrap();
        let graph_texture_creator = graph_canvas.texture_creator();

        let canvas = window.into_canvas().build().unwrap();
        let texture_creator = canvas.texture_creator();

        let ttf_context = ttf::init().unwrap();

        DrawingContext{ width, height, sdl_context, graph_canvas, graph_texture_creator, grid_canvas, canvas, texture_creator, ttf_context }
    }

    pub fn blit_grid(&mut self) {
        let grid_texture = self.texture_creator.create_texture_from_surface(self.grid_canvas.surface()).unwrap();
        self.canvas.copy(&grid_texture, None, None).unwrap();
    }
}

