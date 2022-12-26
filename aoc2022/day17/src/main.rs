use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

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
    last_block_pos: i64,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct State {
    block_idx: usize,
    jet_idx: usize,
    last_block_pos: i64
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
            last_block_pos: 0,
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

    fn state(&self) -> State {
        State {
            block_idx: self.block_idx,
            jet_idx: self.jet_idx,
            last_block_pos: self.last_block_pos,
        }
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
        self.last_block_pos = falling.x;
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
    //let part = 1;
    let part = 2;
    // Part 1 of sample.txt: 3068
    // Part 1 of my_input.txt: 3239
    // Part 2 of sample.txt: 1514285714288
    // Part 2 of my_input.txt: 1594842406882

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut chamber = Chamber::parse(&(lines.next().unwrap()?));

    println!("{:?}", chamber);
    //chamber.print();
    let mut history = HashMap::new();
    let mut heights = vec!();
    let mut states = vec!();

    let mut in_loop = false;
    let mut height_before_loop = 0;
    let mut iterations_before_loop = 0;
    let mut loop_height = 0;
    let mut loop_iterations = 0;

    //let one_by_one_iterations = 2022;
    let one_by_one_iterations = 10000;

    //for i in 0..2022 {
    for i in 0..one_by_one_iterations {
    //for i in 0..100 {
        chamber.drop_one();
        let s = chamber.state();
        let h = chamber.height - MAX_BLOCK_HEIGHT - BLOCK_Y_MARGIN;
        // TODO: This is cheating, but the first match is not actually a loop!
        // We'd need to check that the whole loop is repeating and continue if not.
        // The first 4 iterations match but the 5th is actually different!
        if history.contains_key(&s) {
            if in_loop {
                if states[(i-loop_iterations) as usize] != s {
                    in_loop = false;
                }
            } else {
                in_loop = true;
                println!("Looping for first time: {} -> height = {}", i, h);
                let (prev_i, prev_h) = history[&s];
                iterations_before_loop = prev_i + 1 as i64;
                //let iterations_end_loop = i as i64;
                let height_end_loop = h; //heights[(iterations_end_loop-1) as usize];
                height_before_loop = heights[prev_i as usize];
                loop_height = height_end_loop-height_before_loop;
                loop_iterations = i-prev_i;
                println!("Previous iteration: {} -> height = {} -> loop iterations = {}, loop height = {}", prev_i, prev_h, loop_iterations, loop_height);
            }
        }
        history.insert(s.clone(), (i, h));
        states.push(s);
        heights.push(h);
        println!("{}: {:?} - height: {}", i, chamber.state(), h);
        //chamber.print();
    }

    let mut iterations = 2022;
		if part == 2 {
				iterations = 1000000000000;
		}
    let nb_loops = (iterations-iterations_before_loop)/loop_iterations;
    let iterations_in_loops = nb_loops*loop_iterations;
    let height_in_loops = nb_loops*loop_height;
    let iterations_after_loops = iterations - iterations_in_loops - iterations_before_loop;
    let height_after_loops = heights[(iterations_after_loops + loop_iterations + iterations_before_loop - 1) as usize] - heights[(loop_iterations + iterations_before_loop - 1) as usize];
    let total_height = height_before_loop + height_in_loops + height_after_loops;

    println!("Iterations: {} before loops, {} loops of {} each for a total of {}, {} after loops", iterations_before_loop, nb_loops, loop_iterations, iterations_in_loops, iterations_after_loops);
    println!("Loop height: {}", loop_height);
    println!("Tower is {} tall ({} before loops, {} in loops and {} after loops)", total_height, height_before_loop, height_in_loops, height_after_loops);

    if iterations <= one_by_one_iterations {
        for i in [
           iterations_before_loop - 1,
           loop_iterations + iterations_before_loop - 1,
           nb_loops*loop_iterations + iterations_before_loop - 1,
           iterations_after_loops + nb_loops*loop_iterations + iterations_before_loop - 1,
           iterations_after_loops + loop_iterations + iterations_before_loop - 1,
        ] {
            println!("Height at {}: {}", i, heights[i as usize]);
        }
    }

    //chamber.print();


    Ok(())
}
