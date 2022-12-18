use derive_more::Display;
use itertools::Itertools;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

#[derive(Debug, Display, Clone)]
struct NotAnElevation(String);

impl Error for NotAnElevation {}

#[derive(Clone,Copy,Debug,Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position{x, y}
    }

    fn left(&self) -> Option<Position> {
        if self.x >= 1 {
            Some(Position{x: self.x-1, y: self.y})
        } else {
            None
        }
    }

    fn right(&self) -> Option<Position> {
        Some(Position{x: self.x+1, y: self.y})
    }

    fn up(&self) -> Option<Position> {
        if self.y >= 1 {
            Some(Position{x: self.x, y: self.y-1})
        } else {
            None
        }
    }

    fn down(&self) -> Option<Position> {
        Some(Position{x: self.x, y: self.y+1})
    }
}

#[derive(Debug)]
struct Elevation(i64);

impl TryFrom<char> for Elevation {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' ..= 'z' => {
                Ok(Elevation(c as i64 - 'a' as i64))
            }
            'S' => Ok(Elevation(0)),
            'E' => Ok(Elevation(25)),
            _ => Err(format!("Unsupported elevation: '{}'", c))
        }
    }
}


#[derive(Debug)]
struct Place {
    elevation: Elevation,
    distance: i64
}

impl TryFrom<char> for Place {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let p = Place{
            elevation: Elevation::try_from(c)?,
            distance: i64::MAX,
        };
        Ok(p)
    }
}

impl Place {
    fn repr(&self) -> char {
        (self.elevation.0 as u8 + 'a' as u8) as char
    }
}

#[derive(Debug,Default)]
struct Map {
    map: Vec<Vec<Place>>,
    start: Position,
    end: Position,
}

impl Map {
    fn parse(lines: &mut Lines<BufReader<File>>) -> Result<Map, Box<dyn Error>> {
        let mut m = Map{..Default::default()};
        for (y, l) in lines.enumerate() {
            let l = l?;
            let mut row = vec!();
            for (x, c) in l.chars().enumerate() {
                row.push(Place::try_from(c)?);
                if c == 'S' {
                    m.start = Position::new(x, y);
                }
                if c == 'E' {
                    m.end = Position::new(x, y);
                }
            }
            m.map.push(row);
        }
        Ok(m)
    }

    fn width(&self) -> usize {
        self.map.last().unwrap().len()
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn print(&self) {
        for l in self.map.iter() {
            for p in l {
                print!("{:02} ", p.elevation.0);
            }
            println!("");
        }
    }

    fn print_repr(&self) {
        for l in self.map.iter() {
            for p in l {
                print!("{} ", p.repr());
            }
            println!("");
        }
    }


    fn print_distances(&self) {
        for l in self.map.iter() {
            for p in l {
                print!("{:02} ", p.distance);
            }
            println!("");
        }
    }

    fn elevation_at(&self, pos: Position) -> i64 {
        self.map[pos.y][pos.x].elevation.0
    }

    fn distance_at(&self, pos: Position) -> i64 {
        self.map[pos.y][pos.x].distance
    }

    fn set_distance(&mut self, pos: Position, d: i64) {
        self.map[pos.y][pos.x].distance = d;
    }

    fn valid_pos(&self, pos: Position) -> bool {
        pos.y < self.map.len() && pos.x < self.map.last().unwrap().len()
    }

    fn update_distance(&mut self, pos: Position, next_pos: Position) -> bool {
        if !self.valid_pos(next_pos) {
            // Out of map
            return false
        }
        if self.elevation_at(next_pos) > self.elevation_at(pos) + 1 {
            // Can't go up by more than 1
            return false
        }
        let d = self.distance_at(pos) + 1;
        if self.distance_at(next_pos) > d {
            // Found shorter path to next_pos
            self.set_distance(next_pos, d);
            return true;
        }
        false
    }

    fn reset_distances(&mut self) {
        for row in self.map.iter_mut() {
            for p in row {
                p.distance = i64::MAX;
            }
        }
    }

    fn resolve_path_from(&mut self, start: Position) {
        self.reset_distances();
        self.set_distance(start, 0);
        let mut queue = VecDeque::from([start]);
        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if let Some(next_pos) = pos.left() {
                if self.update_distance(pos, next_pos) {
                    queue.push_back(next_pos);
                }
            }
            if let Some(next_pos) = pos.right() {
                if self.update_distance(pos, next_pos) {
                    queue.push_back(next_pos);
                }
            }
            if let Some(next_pos) = pos.up() {
                if self.update_distance(pos, next_pos) {
                    queue.push_back(next_pos);
                }
            }
            if let Some(next_pos) = pos.down() {
                if self.update_distance(pos, next_pos) {
                    queue.push_back(next_pos);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut m = Map::parse(&mut lines)?;

    //println!("Map: {:?}", m);

    m.print_repr();
    //m.print();
    //m.print_distances();

    m.resolve_path_from(m.start);

    //println!("Map: {:?}", m);

    //m.print_distances();

    // Part 1
    println!("Distance to end: {}", m.distance_at(m.end));

    // Part 2
    let mut min_d = i64::MAX;
    let starting_positions = Itertools::cartesian_product(0..m.width(), 0..m.height())
        .map(|(x,y)| Position::new(x, y))
        .filter(|p| m.elevation_at(*p) == 0)
        .collect::<Vec<_>>();

    for start in starting_positions {
            m.resolve_path_from(start);
            let d = m.distance_at(m.end);
            if d < min_d {
                min_d = d;
            }
    }

    println!("Minimum distance to end: {}", min_d);

    Ok(())
}
