use png::HasParameters;
use sdl2::image::INIT_PNG;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Canvas;
use sdl2::render::TextureCreator;
use sdl2::surface::{Surface,SurfaceRef,SurfaceContext};
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

        let window = video_subsystem.window("Evolution simulation", width, height)
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

    #[allow(dead_code)]
    pub fn save_grid_bmp(&self, p: &Path) {
        self.grid_canvas.surface().save_bmp(p).unwrap();
    }

    fn save_png(&self, surface: &SurfaceRef, p: &Path) {
        //println!("Saving {}", p.to_str().unwrap());
        let file = File::create(p).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.width, self.height);
        let data = surface.without_lock().unwrap();
        //println!("SDL data size: {}", data.len());
        //println!("Pixel format: {:?}", surface.pixel_format_enum());
        let data : Vec<_> = match surface.pixel_format_enum() {
            PixelFormatEnum::ABGR8888 => {
                encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                data.into_iter().cloned().collect()
            },
            PixelFormatEnum::RGBA8888 => {
                encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                let mut tmp : Vec<_> = data.into_iter().cloned().collect();
                // Swap red and alpha and green and blue for each pixel
                (0..self.width*self.height).for_each(|x| {
                    tmp.swap((x*4) as usize, (x*4+3) as usize);
                    tmp.swap((x*4+1) as usize, (x*4+2) as usize);
                });
                tmp
            }
            PixelFormatEnum::ARGB8888 => {
                encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                let mut tmp : Vec<_> = data.into_iter().cloned().collect();
                // Swap red and blue for each pixel
                (0..self.width*self.height).for_each(|x| tmp.swap((x*4) as usize, (x*4+2) as usize));
                tmp
            }
            PixelFormatEnum::BGRA8888 => {
                encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                let mut tmp : Vec<_> = data.into_iter().cloned().collect();
                // Rotate components from ARGB to RGBA
                (0..self.width*self.height).for_each(|x| {
                    tmp.swap((x*4) as usize, (x*4+3) as usize);
                    tmp.swap((x*4) as usize, (x*4+2) as usize);
                    tmp.swap((x*4) as usize, (x*4+1) as usize);
                });
                tmp
            }
            PixelFormatEnum::RGBX8888 => {
                encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
                // Remove the X part (first u8 of each group of 4)
                let mut tmp : Vec<_> = data.into_iter().enumerate().filter(|&(i, _)| i%4 != 0).map(|(_, v)| v).cloned().collect();
                // Swap red and blue for each pixel
                (0..self.width*self.height).for_each(|x| tmp.swap((x*3) as usize, (x*3+2) as usize));
                tmp
            }
            PixelFormatEnum::BGRX8888 => {
                encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
                // Remove the X part (first u8 of each group of 4)
                data.into_iter().enumerate().filter(|&(i, _)| i%4 != 0).map(|(_, v)| v).cloned().collect()
            }
            PixelFormatEnum::RGB888 => {
                encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
                // Remove the padding (last u8 of each group of 4)
                let mut tmp : Vec<_> = data.into_iter().enumerate().filter(|&(i, _)| i%4 != 3).map(|(_, v)| v).cloned().collect();
                // Swap red and blue for each pixel
                (0..self.width*self.height).for_each(|x| tmp.swap((x*3) as usize, (x*3+2) as usize));
                tmp
            },
            PixelFormatEnum::BGR888 => {
                encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
                // Remove the padding (last u8 of each group of 4)
                data.into_iter().enumerate().filter(|&(i, _)| i%4 != 3).map(|(_, v)| v).cloned().collect()
            },
            _ => {
                panic!("Unsupported pixel format: {:?}", surface.pixel_format_enum());
            }
        };
        let mut writer = encoder.write_header().unwrap();
        //println!("PNG data size: {}", data.len());
        writer.write_image_data(&data[..]).unwrap();
    }

    pub fn save_graph_png(&self, p: &Path) {
        let graph_surface = self.graph_canvas.surface();
        self.save_png(&graph_surface, p);
    }

    pub fn save_grid_png(&self, p: &Path) {
        let grid_surface = self.grid_canvas.surface();
        self.save_png(&grid_surface, p);
    }

    pub fn blit_graph(&mut self) {
        let graph_texture = self.texture_creator.create_texture_from_surface(self.graph_canvas.surface()).unwrap();
        self.canvas.copy(&graph_texture, None, None).unwrap();
    }

    pub fn blit_grid(&mut self) {
        let grid_texture = self.texture_creator.create_texture_from_surface(self.grid_canvas.surface()).unwrap();
        self.canvas.copy(&grid_texture, None, None).unwrap();
    }
}
