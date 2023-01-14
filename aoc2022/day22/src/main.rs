use eframe::egui::{self,Align,RichText};
use egui::{Color32,Rect,Sense};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Duration;

#[derive(Clone,Copy,Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(c: char) -> Direction {
        match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => unimplemented!("Unknown direction '{}'", c),
        }
    }
}

#[derive(Clone,Copy,Debug)]
enum Instruction {
    Move(i64),
    Rotate(Direction),
}

impl Instruction {
    fn parse(line: String) -> Vec<Instruction> {
        let mut result = vec![];
        let mut nb = 0;
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                nb *= 10;
                nb += c as i64 - '0' as i64;
            } else if c == 'R' || c == 'L' {
                result.push(Instruction::Move(nb));
                nb = 0;
                result.push(Instruction::Rotate(Direction::parse(c)));
            }
        }
        if nb != 0 {
            result.push(Instruction::Move(nb));
        }
        result
    }
}

#[derive(Debug)]
enum Cell {
    Outside,
    Wall,
    Inside,
    Visited,
}

#[derive(Debug)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn rotate_right(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn rotate_left(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        }
    }

    fn facing(&self) -> i64 {
        match self {
            Orientation::Up => 3,
            Orientation::Right => 0,
            Orientation::Down => 1,
            Orientation::Left => 2,
        }
    }
}

#[derive(Debug,PartialEq)]
enum Speed {
    X0,
    X1,
    X2,
    X4,
    X8,
    X16,
    X32,
    X64,
    X128,
    X256,
}

impl Speed {
    fn value(&self) -> i64 {
        match self {
            Speed::X0 => 0,
            Speed::X1 => 1,
            Speed::X2 => 2,
            Speed::X4 => 4,
            Speed::X8 => 8,
            Speed::X16 => 16,
            Speed::X32 => 32,
            Speed::X64 => 64,
            Speed::X128 => 128,
            Speed::X256 => 256,
        }
    }

    fn name(&self) -> String {
        String::from(
        match self {
            Speed::X0 => "x0",
            Speed::X1 => "x1",
            Speed::X2 => "x2",
            Speed::X4 => "x4",
            Speed::X8 => "x8",
            Speed::X16 => "x16",
            Speed::X32 => "x32",
            Speed::X64 => "x64",
            Speed::X128 => "x128",
            Speed::X256 => "x256",
        })
    }
}

#[derive(Debug)]
struct MyApp {
		width: usize,
	  height: usize,
    instructions: Vec<Instruction>,
    map: Vec<Vec<Cell>>,
    pos: (usize, usize),
    orientation: Orientation,
    step_idx: usize,
    speed: Speed,
}

impl MyApp {
    fn parse(lines: &mut Lines<BufReader<File>>) -> Self {
        let mut result = MyApp {
						width: 0,
					  height: 0,
            map: vec![],
            instructions: vec![],
            pos: (0, 0), 
            orientation: Orientation::Right,
            step_idx: 0,
            speed: Speed::X0,
        };
        let mut finished_map = false;
        for l in lines {
            let l = l.unwrap();
            if l.is_empty() {
                finished_map = true;
                continue;
            }
            if finished_map {
                result.instructions = Instruction::parse(l);
            } else {
                // Read map line
                // TODO: Use an enum rather than strings for cells.
                result.map.push(vec![]);
                for c in l.chars() {
                    match c {
                        ' ' => {
                            result.map.last_mut().unwrap().push(Cell::Outside);
                        }
                        '.' => {
                            result
                                .map
                                .last_mut()
                                .unwrap()
                                .push(Cell::Inside);
                        }
                        '#' => {
                            result
                                .map
                                .last_mut()
                                .unwrap()
                                .push(Cell::Wall);
                        }
                        _ => todo!("Unsupported char in map: {}", c),
                    }
                }
            }
        }
				result.width = result.map.iter().map(|x| x.len()).max().unwrap();
				result.height = result.map.len();
        result.pos = (result.map[0].iter().position(|x| matches!(x, Cell::Inside)).unwrap(), 0);
        result
    }

    fn do_move(&mut self) {
        let (mut new_x, mut new_y) = self.pos;
        println!("Moving {:?} from {}, {}", self.orientation, self.pos.0, self.pos.1);
        // The problem is losely specified in the case the map has discontinuities (inside,
        // outside, inside) horizontally or vertically. Should we move to the next inside section
        // or loop in the current one? Fortunately, this doesn't happen.
        // In this code, we opt for the next section.
        match self.orientation {
            Orientation::Up => {
                if self.pos.1 == 0 {
                    new_y = self.height-1;
                    while self.pos.0 >= self.map[new_y].len() || matches!(self.map[new_y][self.pos.0], Cell::Outside) {
                        new_y -= 1;
                    }
                } else {
                    new_y -= 1;
                    while self.pos.0 >= self.map[new_y].len() || matches!(self.map[new_y][self.pos.0], Cell::Outside) {
                        new_y -= 1;
                        if new_y == 0 {
                            new_y = self.height-1;
                        }
                    }
                }
            },
            Orientation::Down => {
                if self.pos.1 == self.height-1 {
                    new_y = 0;
                    while self.pos.0 >= self.map[new_y].len() || matches!(self.map[new_y][self.pos.0], Cell::Outside) {
                        new_y += 1;
                    }
                } else {
                    new_y += 1;
                    while self.pos.0 >= self.map[new_y].len() || matches!(self.map[new_y][self.pos.0], Cell::Outside) {
                        new_y += 1;
                        if new_y == self.height {
                            new_y = 0
                        }
                    }
                }
            },
            Orientation::Left => {
                if self.pos.0 == 0 {
                    new_x = self.map[new_y].len()-1;
                } else {
                    new_x = self.pos.0 - 1;
                    let mut loopback = false;
                    while matches!(self.map[new_y][new_x], Cell::Outside) {
                        if new_x == 0 {
                            loopback = true;
                            break;
                        }
                        new_x -= 1;
                    }
                    if loopback {
                        new_x = self.map[new_y].len()-1;
                    }
                }
            },
            Orientation::Right => {
                if self.pos.0 == self.map[self.pos.1].len()-1 {
                    new_x = self.map[self.pos.1].iter().position(|x| !matches!(x, Cell::Outside)).unwrap();
                } else {
                    new_x += 1
                    // We don't have to handle the case where there's an Outside after an Inside in
                    // practice. If we had this would need to be the same logic as for Down above.
                }
            },
        }
        if matches!(self.map[new_y][new_x], Cell::Inside) || matches!(self.map[new_y][new_x], Cell::Visited) {
            self.pos = (new_x, new_y);
        }
    }

    fn step(&mut self) {
        if self.step_idx < self.instructions.len() {
            let ins = self.instructions[self.step_idx].clone();
            self.map[self.pos.1][self.pos.0] = Cell::Visited;
            match ins {
                Instruction::Move(n) => {
                    for _ in 0..n {
                        self.do_move();
                        self.map[self.pos.1][self.pos.0] = Cell::Visited;
                        println!("Moved to {}, {}", self.pos.0, self.pos.1);
                    }
                },
                    Instruction::Rotate(Direction::Right) => self.orientation = self.orientation.rotate_right(),
                    Instruction::Rotate(Direction::Left) => self.orientation = self.orientation.rotate_left(),
            }
            self.step_idx += 1;
        }
    }

    fn reset(&mut self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if let Cell::Visited = self.map[y][x] {
                    self.map[y][x] = Cell::Inside;
                }
            }
        }
        self.step_idx = 0;
        self.pos = (self.map[0].iter().position(|x| matches!(x, Cell::Inside)).unwrap(), 0);
        self.orientation = Orientation::Right;
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let margin = 10.0;
        let window_size = frame.info().window_info.size;
        let map_size = window_size.y.min(window_size.x - 300.0) - 2.0*margin;
        egui::SidePanel::right("instructions").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Step")).clicked() {
                        self.step();
                        ctx.request_repaint_after(Duration::from_millis(1));
                }
                if ui.add(egui::Button::new("Reset")).clicked() {
                        self.reset();
                        ctx.request_repaint_after(Duration::from_millis(1));
                }
            });
            ui.label("Speed:");
            ui.selectable_value(&mut self.speed, Speed::X0, "x0");
            ui.selectable_value(&mut self.speed, Speed::X1, "x1");
            ui.selectable_value(&mut self.speed, Speed::X2, "x2");
            ui.selectable_value(&mut self.speed, Speed::X4, "x4");
            ui.selectable_value(&mut self.speed, Speed::X8, "x8");
            ui.selectable_value(&mut self.speed, Speed::X16, "x16");
            ui.selectable_value(&mut self.speed, Speed::X32, "x32");
            ui.selectable_value(&mut self.speed, Speed::X64, "x64");
            ui.selectable_value(&mut self.speed, Speed::X128, "x128");
            ui.selectable_value(&mut self.speed, Speed::X256, "x256");
            ui.separator();
            let row = (self.pos.1 + 1) as i64;
            let column = (self.pos.0 + 1) as i64;
            let facing = self.orientation.facing();
            ui.label(format!("1000 * {} + 4 * {} + {} = {}", row, column, facing, 1000*row + 4*column + facing));
            egui::ScrollArea::both().show(ui, |ui| {
                // The separator is just here to force a larger minimum width
                ui.separator();
                ui.heading("Instructions");
                let mut offset = None;
                for (idx, ins) in self.instructions.iter().enumerate() {
                    let color = match idx == self.step_idx {
                        true => Color32::RED,
                        false => Color32::GRAY,
                    };
                    let res = ui.label(RichText::new(match ins {
                        Instruction::Move(i) => i.to_string(),
                        Instruction::Rotate(Direction::Right) => String::from("➡"),
                        Instruction::Rotate(Direction::Left) => String::from("⬅"),
                    }).color(color));
                    if idx == self.step_idx {
                        offset = Some(res.rect.min.y);
                    }
                }
                // TODO: Dedupe this from code for regular instructions
                let mut color = Color32::GRAY;
                if self.step_idx == self.instructions.len() {
                    color = Color32::RED;
                }
                let res = ui.label(RichText::new("Finished!").color(color));
                if self.step_idx == self.instructions.len() {
                        offset = Some(res.rect.min.y);
                }
                if let Some(offset) = offset {
                    ui.scroll_to_rect(Rect::from_min_size(egui::Pos2::new(0.0, offset), egui::Vec2::new(1.0, 1.0)), Some(Align::Center));
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter_size = egui::vec2(map_size, map_size);
            let (_, painter) = ui.allocate_painter(painter_size, Sense::hover());

            let side: f32 = std::cmp::max(self.width, self.height) as f32;
            let to_panel_pos = |pos: (usize, usize)| {
                (egui::vec2(margin + pos.0 as f32 * painter_size.x / side, margin + pos.1 as f32 * painter_size.y / side)).to_pos2()
            };

            //painter.rect_filled(Rect::EVERYTHING, 0.0, Color32::WHITE);
            for x in 0..self.width {
                for y in 0..self.height {
                    if self.map[y].len() <= x {
                        continue;
                    }
                    let mut color = match self.map[y][x] {
                        Cell::Outside => Color32::BLACK,
                        Cell::Wall => Color32::DARK_RED,
                        Cell::Inside => Color32::LIGHT_GRAY,
                        Cell::Visited => Color32::GREEN,
                    };
                    if (x, y) == self.pos {
                        color = Color32::BLUE;
                    }
										let rect = Rect::from_min_max(to_panel_pos((x, y)), to_panel_pos((x+1, y+1)));
                    painter.rect_filled(rect, 0.0, color);
                }
            }
        });
        for _ in 0..self.speed.value() {
            self.step();
        }
        ctx.request_repaint_after(Duration::from_millis(1000));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let myapp = MyApp::parse(&mut lines);

    let options = eframe::NativeOptions {
        maximized: true,
        initial_window_size: Some(egui::vec2(900.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 — Day 21",
        options,
        Box::new(|_cc| Box::new(myapp)),
    );

    Ok(())
}
