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
}

impl Grid {
    pub fn new(width: u32, height: u32, cell_size: u32) -> Grid {
        let mut cells = vec!();
        for x in 0..width {
            let mut cells_row = vec!();
            for y in 0..height {
                let color = Grid::empty_color();
                let cell = Cell{x, y, color};
                cells_row.push(cell);
            }
            cells.push(cells_row);
        }
        Grid{cells, width, height, cell_size}
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn empty_color() -> Color {
        Color::RGB(0, 0, 0)
    }

    pub fn empty(&self, x: u32, y: u32) -> bool {
        // TODO: Cell should have more logic for handling emptiness
        self.cells[x as usize][y as usize].color == Grid::empty_color()
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        self.cells[x as usize][y as usize].color = color;
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let light_grey = Color::RGB(192, 192, 192);
                dc.canvas.set_draw_color(cell.color);
                dc.canvas.fill_rect(sdl2::rect::Rect::new((cell.x * self.cell_size) as i32, (cell.y * self.cell_size) as i32, self.cell_size, self.cell_size)).unwrap();
                dc.canvas.set_draw_color(light_grey);
                dc.canvas.draw_rect(sdl2::rect::Rect::new((cell.x * self.cell_size) as i32, (cell.y * self.cell_size) as i32, self.cell_size, self.cell_size)).unwrap();
            }
        }
    }
}
