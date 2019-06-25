use crate::dc::DrawingContext;
use sdl2::pixels::Color;
use sdl2::rect::Point;
//use std::convert::TryFrom;

const MARGIN : u32 = 20;
const LEGEND_MARGIN : i32 = 5;
const TITLE_HEIGHT : u32 = 50;
const TOP_MARGIN : u32 = 2*MARGIN + TITLE_HEIGHT; // Margin above and below the title
const DOWN_MARGIN : u32 = MARGIN;
const LEFT_MARGIN : u32 = MARGIN;
const RIGHT_MARGIN : u32 = MARGIN + 50;  // Add 50 for the legend
const PT_SIZE : u32 = 3;

pub struct Graph<T> {
    title: String,
    legend: Vec<String>,
    data: Vec<Vec<T>>,
    scale: Vec<f64>,
}

impl <T> Graph<T> where
    T: std::marker::Copy,
    T: std::clone::Clone,
    T: std::cmp::Ord,
    T: std::ops::AddAssign,
    T: std::ops::SubAssign,
    T: std::ops::Add<Output=T>,
    T: std::ops::Div<Output=T>,
    T: std::ops::Mul<Output=T>,
    T: std::ops::Sub<Output=T>,
    T: std::convert::From<u32>,
    T: std::convert::TryFrom<i32>,
    i32: std::convert::TryFrom<T>,
    f64: std::convert::From<T>,
    <T as std::convert::TryFrom<i32>>::Error : std::fmt::Debug, 
    <i32 as std::convert::TryFrom<T>>::Error : std::fmt::Debug {
    pub fn new(title: String, legend: Vec<String>, data: Vec<Vec<T>>) -> Graph<T> {
        let mut scale = vec!();
        for _ in data.iter() {
            scale.push(1.0);
        }
        Graph {title, legend, data, scale}
    }

/*
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_legend(&mut self, legend: Vec<String>) {
        self.legend = legend;
    }

    pub fn set_data(&mut self, data: Vec<Vec<T>>) {
        self.data = data;
    }
    */

    pub fn set_scale(&mut self, scale: Vec<f64>) {
        self.scale = scale;
    }

    fn draw_background(&self, dc: &mut DrawingContext) {
        let black = Color::RGB(0, 0, 0);
        dc.graph_canvas.set_draw_color(black);
        dc.graph_canvas.fill_rect(sdl2::rect::Rect::new(0, 0, dc.width, dc.height)).unwrap();
    }

    fn draw_title(&self, dc: &mut DrawingContext) {
        let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 50).unwrap();
        let white = Color::RGB(255, 255, 255);
        let title = font.render(&self.title).solid(white).unwrap();
        //let mut r = centered_rect(&title.rect(), &screen_rect);
        let r = sdl2::rect::Rect::new((dc.width as i32 - title.rect().w)/2, LEFT_MARGIN as i32, title.rect().w as u32, title.rect().h as u32);
        let title = dc.graph_texture_creator.create_texture_from_surface(title).unwrap();
        dc.graph_canvas.copy(&title, None, r).expect("Rendering graph title failed");
    }

    fn draw_legend(&self, dc: &mut DrawingContext, c: usize, x: i32, y: i32, color: Color) {
        let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 20).unwrap();
        let legend = font.render(&self.legend[c]).solid(color).unwrap();
        let mut r = legend.rect();
        r.x = x + LEGEND_MARGIN;
        r.y = y - r.h/2;
        let legend = dc.graph_texture_creator.create_texture_from_surface(legend).unwrap();
        dc.graph_canvas.copy(&legend, None, r).expect("Rendering graph legend failed");
    }

    fn draw_gen(&self, dc: &mut DrawingContext) {
        let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 30).unwrap();
        let white = Color::RGB(255, 255, 255);
        let text = font.render(&format!("round={}", self.data[0].len())).solid(white).unwrap();
        let mut r = text.rect();
        r.x = dc.width as i32 - r.w;
        let text = dc.graph_texture_creator.create_texture_from_surface(text).unwrap();
        dc.graph_canvas.copy(&text, None, r).expect("Rendering graph gen text failed");
    }

    fn draw_scale(&self, dc: &mut DrawingContext) {
        if self.scale.iter().filter(|s| **s != 1.0).count() != 0 {
            let scales_text = self.scale.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
            let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 30).unwrap();
            let white = Color::RGB(255, 255, 255);
            let text = font.render(&format!("scale={}", scales_text)).solid(white).unwrap();
            let mut r = text.rect();
            r.y = 30; // Same as gen font size
            r.x = dc.width as i32 - r.w;
            let text = dc.graph_texture_creator.create_texture_from_surface(text).unwrap();
            dc.graph_canvas.copy(&text, None, r).expect("Rendering graph gen text failed");
        }
    }

    fn draw_graph(&self, dc: &mut DrawingContext) {
        //println!("Graph with {} lines", self.data.len());
        if self.data.is_empty() || self.data[0].is_empty() {
            return
        }
        // TODO: Support scale in computing min/max Y
        let mut max_y = self.data.iter().cloned().fold(self.data[0][0],
                |m, subdata| subdata.iter().cloned().fold(m, T::max));
        let mut min_y = self.data.iter().cloned().fold(max_y,
                |m, subdata| subdata.iter().cloned().fold(m, T::min));
        if max_y == min_y {
            max_y += T::try_from(1).unwrap();
            min_y -= T::try_from(1).unwrap();
        }
        let colors = vec![
            Color::RGB(0, 255, 0),
            Color::RGB(255, 0, 0),
            Color::RGB(0, 0, 255),
            Color::RGB(255, 255, 0),
            Color::RGB(255, 0, 255),
            Color::RGB(0, 255, 255),
        ];
        let dy = (dc.height - TOP_MARGIN - DOWN_MARGIN) as f64 / f64::from(max_y - min_y);
        for (c, ds) in self.data.iter().enumerate() {
            let scale = self.scale[c];
            //println!("Curve {} with {} points", c, ds.len());
            let dx = (dc.width - LEFT_MARGIN - RIGHT_MARGIN) as f64 / ds.len() as f64;
            let color = colors[c % colors.len()];
            dc.graph_canvas.set_draw_color(color);
            let mut prev_x = 0;
            let mut prev_y = 0;
            for (i, v) in ds.iter().enumerate() {
                let x = (LEFT_MARGIN as f64 + i as f64*dx) as i32;
                let y = (TOP_MARGIN as f64 + (f64::from(max_y)-f64::from(*v)*scale)*dy) as i32;
                //println!("Point {} at {}, {} ({})", i, x, y, i32::try_from(*v).unwrap());
                dc.graph_canvas.fill_rect(sdl2::rect::Rect::new(x-PT_SIZE as i32, y-PT_SIZE as i32, 2*PT_SIZE, 2*PT_SIZE)).unwrap();
                if i > 0 {
                    dc.graph_canvas.draw_line(Point::new(prev_x, prev_y), Point::new(x, y)).unwrap();
                }
                prev_x = x;
                prev_y = y;
            }
            self.draw_legend(dc, c, prev_x, prev_y, color);
        }
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        self.draw_background(dc);
        self.draw_title(dc);
        self.draw_graph(dc);
        self.draw_gen(dc);
        self.draw_scale(dc);
    }
}
