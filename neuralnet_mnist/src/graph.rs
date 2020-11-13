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

pub struct Graph {
    title: String,
    legend: Vec<String>,
    data: Vec<Vec<f64>>,
    additional_infos: Vec<String>,
}

impl Graph {
    pub fn new(title: String, legend: Vec<String>, data: Vec<Vec<f64>>, additional_infos: Vec<String>) -> Graph {
        Graph {title, legend, data, additional_infos}
    }

/*
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn set_legend(&mut self, legend: Vec<String>) {
        self.legend = legend;
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>) {
        self.data = data;
    }
    */

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

    fn draw_graph(&self, dc: &mut DrawingContext) {
        //println!("Graph with {} lines", self.data.len());
        if self.data.is_empty() || self.data[0].is_empty() {
            return
        }
        // TODO: Support scale in computing min/max Y
        let mut max_y = self.data.iter().cloned().fold(self.data[0][0],
                |m, subdata| subdata.iter().cloned().fold(m, f64::max));
        let mut min_y = self.data.iter().cloned().fold(max_y,
                |m, subdata| subdata.iter().cloned().fold(m, f64::min));
        if max_y == min_y {
            max_y += 1.0;
            min_y -= 1.0;
        }
        let colors = vec![
            Color::RGB(0, 255, 0),
            Color::RGB(255, 0, 0),
            Color::RGB(0, 0, 255),
            Color::RGB(255, 255, 0),
            Color::RGB(255, 0, 255),
            Color::RGB(0, 255, 255),
        ];
        let dy = (dc.height - TOP_MARGIN - DOWN_MARGIN) as f64 / (max_y - min_y);
        for (c, ds) in self.data.iter().enumerate() {
            //println!("Curve {} with {} points", c, ds.len());
            let dx = (dc.width - LEFT_MARGIN - RIGHT_MARGIN) as f64 / ds.len() as f64;
            let color = colors[c % colors.len()];
            dc.graph_canvas.set_draw_color(color);
            let mut prev_x = 0;
            let mut prev_y = 0;
            for (i, v) in ds.iter().enumerate() {
                let x = (LEFT_MARGIN as f64 + i as f64*dx) as i32;
                let y = (TOP_MARGIN as f64 + (max_y-*v)*dy) as i32;
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

    fn draw_additional_infos(&self, dc: &mut DrawingContext) {
        let font = dc.ttf_context.load_font("./resources/DejaVuSans.ttf", 20).unwrap();
        let white = Color::RGB(255, 255, 255);
        for (i, info) in self.additional_infos.iter().enumerate() {
            let text = font.render(info).solid(white).unwrap();
            let mut r = text.rect();
            r.x = MARGIN as i32;
            r.y = MARGIN as i32 + (i*30) as i32;
            let text = dc.graph_texture_creator.create_texture_from_surface(text).unwrap();
            dc.graph_canvas.copy(&text, None, r).expect("Rendering graph text failed");
        }
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        self.draw_background(dc);
        self.draw_title(dc);
        self.draw_graph(dc);
        self.draw_additional_infos(dc);
    }
}
