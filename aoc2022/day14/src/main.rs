use itertools::Itertools;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::{thread,time};

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn parse(s: &str) -> Position {
            let (x, y) = s.split(',')
                .map(|x| x.parse().unwrap())
                .next_tuple().unwrap();
            Position{x, y}
    }

    fn path_to(&self, other: &Position) -> Vec<Position> {
        let mut result = vec!();
        if self.x == other.x {
            let y1 = std::cmp::min(self.y, other.y);
            let y2 = std::cmp::max(self.y, other.y);
            for y in y1..=y2 {
                result.push(Position{x: self.x, y});
            }
        } else {
            let x1 = std::cmp::min(self.x, other.x);
            let x2 = std::cmp::max(self.x, other.x);
            for x in x1..=x2 {
                result.push(Position{x, y: self.y});
            }
        }
        result
    }

    fn below(&self) -> Position {
        Position {
            x: self.x,
            y: self.y+1,
        }
    }

    fn below_left(&self) -> Position {
        Position {
            x: self.x-1,
            y: self.y+1,
        }
    }

    fn below_right(&self) -> Position {
        Position {
            x: self.x+1,
            y: self.y+1,
        }
    }
}

#[derive(PartialEq,Eq)]
enum Outcome {
    Rest,
    ReachedFloor,
    Blocking,
}

#[derive(Debug)]
struct Map {
    map: HashMap<Position, char>,
    x_min: i64, x_max: i64,
    y_min: i64, y_max: i64,
    y_floor: i64,
    falling: Option<Position>,
}

impl Map {
    fn new() -> Self {
        Map{
            map: HashMap::new(),
            x_min: 500, x_max: 500,
            y_min: 0, y_max: 0,
            y_floor: 0,
            falling: None,
        }
    }

    fn insert(&mut self, p: Position, c: char) {
        if p.x < self.x_min {
            self.x_min = p.x
        }
        if p.x > self.x_max {
            self.x_max = p.x
        }
        if p.y < self.y_min {
            self.y_min = p.y
        }
        if p.y > self.y_max {
            self.y_max = p.y
        }
        self.map.insert(p, c);
    }

    fn print(&self) {
        println!();
        for y in self.y_min-1..=self.y_max+1 {
            for x in self.x_min-1..=self.x_max+1 {
                let p = Position{x, y};
                if let Some(f) = self.falling.clone() {
                    if f == p {
                        print!("o");
                        continue;
                    }
                }
                let c = self.map.get(&p).or(Some(&'.')).unwrap();
                print!("{}", c);
            }
            println!();
        }
    }

    fn fall_one(&mut self) -> Outcome {
        self.falling = Some(Position{x: 500, y: 0});
        loop {
            //self.print();
            //thread::sleep(time::Duration::from_millis(10));
            let below = self.falling.clone().unwrap().below();
            let below_left = self.falling.clone().unwrap().below_left();
            let below_right = self.falling.clone().unwrap().below_right();
            if !self.map.contains_key(&below) {
                self.falling = Some(below);
            } else if !self.map.contains_key(&below_left) {
                self.falling = Some(below_left);
            } else if !self.map.contains_key(&below_right) {
                self.falling = Some(below_right);
            } else {
                if self.falling.clone().unwrap() == (Position{x: 500, y: 0}) {
                    return Outcome::Blocking;
                }
                self.insert(self.falling.clone().unwrap(), 'o');
                self.falling = None;
                break;
            }
            if self.falling.clone().unwrap().y == self.y_floor - 1 {
                self.insert(self.falling.clone().unwrap(), 'o');
                self.falling = None;
                return Outcome::ReachedFloor;
            }
        }
        //self.print();
        Outcome::Rest
    }

    fn part_one(&mut self) {
        let mut i = 0;
        self.y_floor = self.y_max + 2;
        loop {
            if self.fall_one() == Outcome::ReachedFloor{
                break
            }
            i += 1;
        }
        self.print();
        println!("Falling into the abyss after {} units", i);
    }

    fn part_two(&mut self) {
        let mut i = 0;
        self.y_floor = self.y_max + 2;
        loop {
            if self.fall_one() == Outcome::Blocking{
                break
            }
            i += 1;
        }
        self.print();
        println!("Blocking incoming sand after {} units", i+1);
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let part = 2;
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut m = Map::new();
    m.insert(Position{x: 500, y: 0}, '+');

    for l in lines {
        let l = l?;
        let positions = l.split(" -> ").collect::<Vec<_>>();
        for p in positions.as_slice().windows(2) {
            let p1 = Position::parse(p[0]);
            let p2 = Position::parse(p[1]);
            println!("From {:?} to {:?}", p1, p2);
            for pp in p1.path_to(&p2) {
                println!("  Adding {:?}", pp);
                m.insert(pp, '#');
            }
        }
    }

    println!("{:?}", m);

    m.print();
    // Run either part one or part two, not both in a row (the state is not reset between them)
    if part == 1 {
        m.part_one();
    } else {
        m.part_two();
    }

    Ok(())
}
