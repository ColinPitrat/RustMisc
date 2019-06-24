extern crate sdl2;

use crate::animal::Animal;
use crate::dc::DrawingContext;
use crate::plant::Plant;
use crate::predator::Predator;
use rand::Rng;
use sdl2::pixels::Color;
use std::rc::Rc;

pub enum CellContent {
    Empty,
    Plant(Rc<Plant>),
    Animal(Rc<Animal>),
    Predator(Rc<Predator>),
}

pub struct Cell {
    x: u32,
    y: u32,
    content: CellContent,
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
                let content = CellContent::Empty;
                let cell = Cell{x, y, content, color};
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

    pub fn plant_color() -> Color {
        Color::RGB(0, 255, 0)
    }

    pub fn animal_color() -> Color {
        Color::RGB(255, 0, 0)
    }

    pub fn predator_color() -> Color {
        Color::RGB(0, 0, 255)
    }

    // TODO: Get rid of xxx_color and keep just this one
    pub fn content_color(content: &CellContent) -> Color {
        match content {
            CellContent::Empty => Grid::empty_color(),
            CellContent::Plant(_) => Grid::plant_color(),
            CellContent::Animal(_) => Grid::animal_color(),
            CellContent::Predator(_) => Grid::predator_color(),
        }
    }

    pub fn empty(&self, x: u32, y: u32) -> bool {
        match self.cells[x as usize][y as usize].content {
            CellContent::Empty => true,
            _ => false,
        }
    }

    pub fn at(&self, x: u32, y: u32) -> &CellContent {
        &self.cells[x as usize][y as usize].content
    }

    pub fn set_content(&mut self, x: u32, y: u32, content: CellContent) {
        self.cells[x as usize][y as usize].color = Grid::content_color(&content);
        self.cells[x as usize][y as usize].content = content;
    }

    pub fn get_empty_cell(&self) -> (u32, u32) {
        let (mut x, mut y);
        loop {
            x = rand::thread_rng().gen_range(0, self.width());
            y = rand::thread_rng().gen_range(0, self.height());
            if self.empty(x, y) {
                break;
            }
        }
        (x, y)
    }

    pub fn show(&self, dc: &mut DrawingContext) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                dc.grid_canvas.set_draw_color(cell.color);
                dc.grid_canvas.fill_rect(sdl2::rect::Rect::new((cell.x * self.cell_size) as i32, (cell.y * self.cell_size) as i32, self.cell_size, self.cell_size)).unwrap();
                // TODO: Draw the grid in a faster way (draw lines instead of squares)
                //let light_grey = Color::RGB(192, 192, 192);
                //dc.grid_canvas.set_draw_color(light_grey);
                //dc.grid_canvas.draw_rect(sdl2::rect::Rect::new((cell.x * self.cell_size) as i32, (cell.y * self.cell_size) as i32, self.cell_size, self.cell_size)).unwrap();
            }
        }
    }
}
