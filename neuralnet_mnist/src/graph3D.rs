use crate::dc::DrawingContext;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const MARGIN : u32 = 20;
const LEGEND_MARGIN : i32 = 5;
const TITLE_HEIGHT : u32 = 50;
const TOP_MARGIN : u32 = 2*MARGIN + TITLE_HEIGHT; // Margin above and below the title
const DOWN_MARGIN : u32 = MARGIN;
const LEFT_MARGIN : u32 = MARGIN;
const RIGHT_MARGIN : u32 = MARGIN + 50;  // Add 50 for the legend
const PT_SIZE : u32 = 3;

pub struct Graph3D {
    title: String,
    legend: Vec<String>,
    data: Vec<Vec<Vec<(f64, f64, f64)>>>,
    projected_data: Vec<Vec<Vec<(f64, f64)>>>,
    additional_infos: Vec<String>,
    x_scale: f64,
    y_scale: f64,
    z_scale: f64,
}

impl Graph3D {
    pub fn new(title: String, legend: Vec<String>, data: Vec<Vec<Vec<(f64, f64, f64)>>>, additional_infos: Vec<String>) -> Graph3D {
        Graph3D {title, legend, data, projected_data: vec!(), additional_infos, x_scale: 1.0, y_scale: 1.0, z_scale: 1.0}
    }

    pub fn set_scale(&mut self, x_scale: f64, y_scale: f64, z_scale: f64) {
        self.x_scale = x_scale;
        self.y_scale = y_scale;
        self.z_scale = z_scale;
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

    fn project(&mut self) {
        let min_x = self.data.iter().cloned().fold(self.data[0][0][0].0,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::min(o, coords.0))));
        let max_x = self.data.iter().cloned().fold(self.data[0][0][0].0,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::max(o, coords.0))));
        let min_y = self.data.iter().cloned().fold(self.data[0][0][0].1,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::min(o, coords.1))));
        let max_y = self.data.iter().cloned().fold(self.data[0][0][0].1,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::max(o, coords.1))));
        let mid_x = (max_x + min_x)/2.0;
        let mid_y = (max_y + min_y)/2.0;
        //println!("Min x={}, Max x={}", min_x, max_x);
        //println!("Min y={}, Max y={}", min_y, max_y);
        // Depth of the screen (eye is at 0)
        let y0 = 10.0;
        // Shift of the closest edge of the graph
        let y_shift = 12.0;
        let z_shift = -50.0;
        self.projected_data = vec!();
        for (g, gs) in self.data.iter().enumerate() {
            self.projected_data.push(vec!());
            for (c, ds) in gs.iter().enumerate() {
                self.projected_data[g].push(vec!());
                for (i, v) in ds.iter().enumerate() {
                    let x = v.0 - mid_x;
                    let y = v.1 - mid_y;
                    let z = v.2 + z_shift;
                    let X = x*y0/(y+y_shift);
                    let Y = -z*y0/(y+y_shift);
                    self.projected_data[g][c].push((X, Y));
                }
            }
        }
    }

    fn draw_graph(&mut self, dc: &mut DrawingContext) {
        //println!("Graph with {} lines", self.data.len());
        if self.data.is_empty() || self.data[0].is_empty() || self.data[0][0].is_empty() {
            return
        }
        self.project();
        let mut min_x = self.projected_data.iter().cloned().fold(self.projected_data[0][0][0].0,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::min(o, coords.0))));
        let mut max_x = self.projected_data.iter().cloned().fold(self.projected_data[0][0][0].0,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::max(o, coords.0))));
        let mut min_y = self.projected_data.iter().cloned().fold(self.projected_data[0][0][0].1,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::min(o, coords.1))));
        let mut max_y = self.projected_data.iter().cloned().fold(self.projected_data[0][0][0].1,
                |m, subdata| subdata.iter().cloned().fold(m,
                    |n, subsubdata| subsubdata.iter().cloned().fold(n, 
                        |o, coords| f64::max(o, coords.1))));
        //println!("Min x={}, Max x={}", min_x, max_x);
        //println!("Min y={}, Max y={}", min_y, max_y);
        if max_x == min_x {
            max_x += 1.0;
            min_x -= 1.0;
        }
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
        let x_scale = (dc.width - LEFT_MARGIN - RIGHT_MARGIN) as f64/(max_x - min_x);
        let y_scale = (dc.height - TOP_MARGIN - DOWN_MARGIN) as f64/(max_y - min_y);
        let x_shift = LEFT_MARGIN as f64;
        let y_shift = TOP_MARGIN as f64;
        //println!("x_scale = {}, y_scale = {}, x_shift = {}, y_shift = {}", x_scale, y_scale, x_shift, y_shift);
        for (g, gs) in self.projected_data.iter().enumerate() {
            let color = colors[g % colors.len()];
            for (c, ds) in gs.iter().enumerate() {
                dc.graph_canvas.set_draw_color(color);
                let mut prev_x = 0;
                let mut prev_y = 0;
                for (i, v) in ds.iter().enumerate() {
                    let x = (x_shift + x_scale*(v.0-min_x)) as i32;
                    let y = (y_shift + y_scale*(v.1-min_y)) as i32;
                    //println!("Point {} at {}, {}", i, x, y);
                    // Avoid overflow if graph is out of bound. This shouldn't happen with proper
                    // scaling.
                    /*if x < PT_SIZE as i32 || y < PT_SIZE as i32 {
                        continue;
                    }*/
                    dc.graph_canvas.fill_rect(sdl2::rect::Rect::new(x-PT_SIZE as i32, y-PT_SIZE as i32, 2*PT_SIZE, 2*PT_SIZE)).unwrap();
                    if i > 0 {
                        dc.graph_canvas.draw_line(Point::new(prev_x, prev_y), Point::new(x, y)).unwrap();
                    }
                    if c > 0 {
                        let px = (x_shift + x_scale*(self.projected_data[g][c-1][i].0-min_x)) as i32;
                        let py = (y_shift + y_scale*(self.projected_data[g][c-1][i].1-min_y)) as i32;
                        dc.graph_canvas.draw_line(Point::new(px, py), Point::new(x, y)).unwrap();
                    }
                    prev_x = x;
                    prev_y = y;
                }
                if c == 0 {
                    self.draw_legend(dc, g, prev_x, prev_y, color);
                }
            }
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

    pub fn show(&mut self, dc: &mut DrawingContext) {
        self.draw_background(dc);
        self.draw_title(dc);
        self.draw_graph(dc);
        self.draw_additional_infos(dc);
    }
}
