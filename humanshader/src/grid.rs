extern crate sdl2;

use crate::dc::DrawingContext;
use sdl2::pixels::Color;
pub struct Cell {
    x: u32,
    y: u32,
    color: Color,
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    width: u32,
    height: u32,
    cell_size: u32,
    margin: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Grid {
        let mut cells = vec!();
        for x in 0..width {
            let mut cells_row = vec!();
            for y in 0..height {
                let color = Grid::content_color(x as i32, y as i32);
                let cell = Cell{x, y, color};
                cells_row.push(cell);
            }
            cells.push(cells_row);
        }
        Grid{cells, width, height, cell_size, margin: 1}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn display_width(&self) -> u32 {
        self.width()*(self.cell_size+self.margin)
    }

    pub fn display_height(&self) -> u32 {
        self.height()*(self.cell_size+self.margin)
    }

    pub fn content_color(x: i32, y: i32) -> Color {
        let (mut red, mut blue);
        // Center of the sphere (and height of horizon).
        let (cx, cy) = (36, 18);
        // Square of radius of the sphere.
        let radius2 = 200;
        let u = x - cx;
        let v = cy - y;
        let h = u*u + v*v;
        if h < radius2 {
            // Section B (sphere)
            red = 420;
            blue = 520;
            let t = 5000 + 8*h;
            let p = (t*u) / 100;
            let q = (t*v) / 100;
            let s = 2*q;
            let mut w = (1000 + p - s) / 100 + 8;
            if w > 0 {
                red = red + w*w;
            }
            let o = s+2200;
            red = (red*o)/10000;
            blue = (blue*o)/10000;
            if p > -q {
                w = (p+q)/10;
                red = red + w;
                blue = blue + w;
            }
        } else if v < 0 {
            // Section C (ground + shadow)
            red = 150 + 2*v;
            blue = 50;
            let p = h + 8*v*v;
            let c = -240*v-p;
            // Note that v < 0.
            // With v = -dy (distance from horizon)
            // And h is the square of the distance from the center of the sphere.
            // c = 240*dy - 8*dy² - h
            if c > 1200 {
                // This makes the shadow under the sphere darker.
                let mut o = (6*c)/10;
                o = c*(1500-o);
                o = o/100-8360;
                red = (red*o)/1000;
                blue = (blue*o)/1000;
            }
            // This makes the rest of the ground (outside of the shadow) lighter.
            // The 3200 will make the red saturate very quickly outside of the shadow.
            // The u*v is what gives the direction "to the left" to the shadow as that's where u
            // gets negative.
            let r = c + u*v;
            let d = 3200 - h - 2*r;
            if d > 0 {
                red = red + d;
            }
        } else {
            // Section D (sky)
            // Gradient of direction (4, 1)
            let c = x+4*y;
            red = 132 + c;
            blue = 192 + c;
        }
        // Section E
        if red > 255 {
            red = 255;
        }
        if blue > 255 {
            blue = 255;
        }
        let green = (7*red + 3*blue)/10;
        Color::RGB(red as u8, green as u8, blue as u8)
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        let light_grey = Color::RGB(192, 192, 192);
        dc.grid_canvas.set_draw_color(light_grey);
        dc.grid_canvas.fill_rect(sdl2::rect::Rect::new(0, 0, self.display_width().try_into().unwrap(), self.display_height().try_into().unwrap())).unwrap();
        for row in self.cells.iter() {
            for cell in row.iter() {
                dc.grid_canvas.set_draw_color(cell.color);
                dc.grid_canvas.fill_rect(sdl2::rect::Rect::new((cell.x * (self.cell_size + self.margin)) as i32, (cell.y * (self.cell_size + self.margin)) as i32, self.cell_size, self.cell_size)).unwrap();
            }
        }
    }
}
