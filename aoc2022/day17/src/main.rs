use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

// Width = 7
// Block appears: 
//   left edge two units away from the left wall 
//   bottom edge is three units above the highest point
// Each round:
//   be pushed by next jet - collision blocks movement
//   go down one unit - collision stops block, next block starts to fall
//
// Shapes
//
// ####
//
// .#.
// ###
// .#.
//
// ..#
// ..#
// ###
//
// #
// #
// #
// #
//
// ##
// ##

#[derive(Clone,Copy,Debug)]
struct Position {
    x: i64,
    y: i64,
}

const MAX_BLOCK_HEIGHT: i64 = 4;
const BLOCK_Y_MARGIN: i64 = 3;

#[derive(Clone,Debug)]
struct Block {
    width: i64,
    height: i64,
    cells: Vec<Position>,
}

impl Block {
    fn block1() -> Block {
        Block {
            width: 4,
            height: 1,
            cells: vec!(
               Position{x: 0, y: 0},
               Position{x: 1, y: 0},
               Position{x: 2, y: 0},
               Position{x: 3, y: 0},
            ),
        }
    }

    fn block2() -> Block {
        Block {
            width: 3,
            height: 3,
            cells: vec!(
                Position{x: 1, y: 0},
                Position{x: 0, y: -1},
                Position{x: 1, y: -1},
                Position{x: 2, y: -1},
                Position{x: 1, y: -2},
            ),
        }
    }

    fn block3() -> Block {
        Block {
            width: 3,
            height: 3,
            cells: vec!(
                Position{x: 2, y: 0},
                Position{x: 2, y: -1},
                Position{x: 0, y: -2},
                Position{x: 1, y: -2},
                Position{x: 2, y: -2},
            ),
        }
    }

    fn block4() -> Block {
        Block {
            width: 1,
            height: 4,
            cells: vec!(
                Position{x: 0, y: 0},
                Position{x: 0, y: -1},
                Position{x: 0, y: -2},
                Position{x: 0, y: -3},
            ),
        }
    }

    fn block5() -> Block {
        Block {
            width: 2,
            height: 2,
            cells: vec!(
                Position{x: 0, y: 0},
                Position{x: 0, y: -1},
                Position{x: 1, y: 0},
                Position{x: 1, y: -1},
            ),
        }
    }
}

#[derive(Clone,Debug)]
struct Falling {
    x: i64,
    y: i64,
    block: Block,
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err(format!("Unsupported direction '{}'", c)),
        }
    }
}

#[derive(Debug)]
enum Cell {
    Empty,
    Temporary,
    Fixed,
}

#[derive(Debug)]
struct Chamber {
    width: i64,
    height: i64,
    blocks: Vec<Block>,
    block_idx: usize,
    jets: Vec<Direction>,
    jet_idx: usize,
    grid: Vec<Vec<Cell>>,
}

impl Chamber {
    fn parse(s: &str) -> Chamber {
        let height = 7;
        let width = 7;
        let mut jets = vec!();
        for c in s.chars() {
            jets.push(Direction::try_from(c).unwrap());
        }
        let mut grid = vec!();
        for y in 0..height {
            grid.push(vec!());
            for x in 0..width {
                grid.iter_mut().last().unwrap().push(Cell::Empty);
            }
        }
        Chamber {
            width,
            height,
            blocks: vec!(
                    Block::block1(),
                    Block::block2(),
                    Block::block3(),
                    Block::block4(),
                    Block::block5(),
            ),
            block_idx: 0,
            jets,
            jet_idx: 0,
            grid,
        }
    }

    fn print(&self) {
        println!("");
        for y in 0..self.height {
            print!("|");
            for x in 0..self.width {
                match self.grid[(self.height-y-1) as usize][x as usize] {
                    Cell::Empty => print!("."),
                    Cell::Temporary => print!("@"),
                    Cell::Fixed => print!("#"),
                }
            }
            println!("|");
        }
        print!("+");
        for x in 0..self.width {
            print!("-");
        }
        println!("+");
    }

    fn drop_one(&mut self) {
        // Drop new block
        let block = self.blocks[self.block_idx].clone();
        self.block_idx += 1;
        if self.block_idx >= self.blocks.len() {
            self.block_idx = 0;
        }
        let mut falling = Falling {
            x: 2,
            y: self.height - MAX_BLOCK_HEIGHT + block.height - 1,
            block,
        };
        'outer: loop {
            // Print grid with falling object
            /*
            {
                for p in &falling.block.cells {
                    self.grid[(falling.y + p.y) as usize][(falling.x + p.x) as usize] = Cell::Temporary;
                }
                self.print();
                for p in &falling.block.cells {
                    self.grid[(falling.y + p.y) as usize][(falling.x + p.x) as usize] = Cell::Empty;
                }
            }
            */
            // Move left or right
            {
                //println!("Move {:?}", self.jets[self.jet_idx]);
                let new_x = match self.jets[self.jet_idx] {
                    Direction::Left => falling.x - 1,
                    Direction::Right => falling.x + 1,
                };
                self.jet_idx += 1;
                if self.jet_idx >= self.jets.len() {
                    self.jet_idx = 0;
                }
                let mut move_ok = true;
                // Test collision
                if new_x < 0 || new_x + falling.block.width > self.width {
                    //println!(" blocked by wall!");
                    move_ok = false;
                } else {
                    for p in &falling.block.cells {
                        if let Cell::Fixed = self.grid[(falling.y + p.y) as usize][(new_x + p.x) as usize] {
                            //println!(" blocked by block!");
                            move_ok = false;
                            break;
                        }
                    }
                }
                if move_ok {
                    falling.x = new_x;
                }
            }
            // Print grid with falling object
            /*
            {
                for p in &falling.block.cells {
                    self.grid[(falling.y + p.y) as usize][(falling.x + p.x) as usize] = Cell::Temporary;
                }
                self.print();
                for p in &falling.block.cells {
                    self.grid[(falling.y + p.y) as usize][(falling.x + p.x) as usize] = Cell::Empty;
                }
            }
            */
            // Move down
            {
                //println!("Falling");
                let new_y = falling.y - 1;
                if new_y - falling.block.height + 1 < 0 {
                    //println!(" touching ground");
                    break 'outer;
                } else {
                    for p in &falling.block.cells {
                        if let Cell::Fixed = self.grid[(new_y + p.y) as usize][(falling.x + p.x) as usize] {
                            //println!(" touching block");
                            break 'outer;
                        }
                    }
                }
                falling.y = new_y;
            }
        }
        //self.print();
        let mut max_y = 0;
        for p in falling.block.cells {
            self.grid[(falling.y + p.y) as usize][(falling.x + p.x) as usize] = Cell::Fixed;
            if falling.y + p.y >= max_y {
                max_y = falling.y + p.y + 1;
            }
        }
        //println!("Should add lines? {} + {} + {} > {}?", max_y, MAX_BLOCK_HEIGHT, BLOCK_Y_MARGIN, self.height);
        while max_y + MAX_BLOCK_HEIGHT + BLOCK_Y_MARGIN > self.height {
            //println!("Adding one line");
            self.height += 1;
            self.grid.push(vec!());
            for x in 0..self.width {
                self.grid.iter_mut().last().unwrap().push(Cell::Empty);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut chamber = Chamber::parse(&(lines.next().unwrap()?));

    println!("{:?}", chamber);
    //chamber.print();

    for i in 0..2022 {
        chamber.drop_one();
        //chamber.print();
    }
    //chamber.print();

    println!("Tower is {} tall", chamber.height - MAX_BLOCK_HEIGHT - BLOCK_Y_MARGIN);

    Ok(())
}
