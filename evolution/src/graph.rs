use crate::dc::DrawingContext;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::convert::TryFrom;

const MARGIN : u32 = 20;
const PT_SIZE : u32 = 3;

pub struct Graph<T> {
    data: Vec<Vec<T>>
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
    pub fn new(data: Vec<Vec<T>>) -> Graph<T> {
        Graph {data}
    }

    pub fn set_data(&mut self, data: Vec<Vec<T>>) {
        self.data = data;
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        println!("Graph with {} lines", self.data.len());
        if self.data.is_empty() || self.data[0].is_empty() {
            return
        }
        let mut max_y = self.data.iter().cloned().fold(self.data[0][0],
                |m, subdata| subdata.iter().cloned().fold(m, T::max));
        let mut min_y = self.data.iter().cloned().fold(max_y,
                |m, subdata| subdata.iter().cloned().fold(m, T::min));
        if max_y == min_y {
            max_y += T::try_from(1).unwrap();
            min_y -= T::try_from(1).unwrap();
        }
        let color = vec![
            Color::RGB(0, 255, 0),
            Color::RGB(255, 0, 0),
            Color::RGB(0, 255, 255),
        ];
        let dy = (dc.height - 2*MARGIN) as f64 / f64::from(max_y - min_y);
        let black = Color::RGB(0, 0, 0);
        dc.canvas.set_draw_color(black);
        dc.canvas.fill_rect(sdl2::rect::Rect::new(0, 0, dc.width, dc.height)).unwrap();
        for (c, ds) in self.data.iter().enumerate() {
            println!("Curve {} with {} points", c, ds.len());
            let dx = (dc.width - 2*MARGIN) / ds.len() as u32;
            dc.canvas.set_draw_color(color[c % color.len()]);
            let mut prev_x = 0;
            let mut prev_y = 0;
            for (i, v) in ds.iter().enumerate() {
                let x = (MARGIN + i as u32*dx) as i32;
                let y = (MARGIN as f64 + f64::from(max_y-*v)*dy) as i32;
                println!("Point {} at {}, {} ({})", i, x, y, i32::try_from(*v).unwrap());
                dc.canvas.fill_rect(sdl2::rect::Rect::new(x-PT_SIZE as i32, y-PT_SIZE as i32, 2*PT_SIZE, 2*PT_SIZE)).unwrap();
                if i > 0 {
                    dc.canvas.draw_line(Point::new(prev_x, prev_y), Point::new(x, y)).unwrap();
                }
                prev_x = x;
                prev_y = y;
            }
        }
    }
}
