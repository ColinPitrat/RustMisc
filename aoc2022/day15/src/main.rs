use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn distance(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone,Debug)]
struct Rect {
    top: i64,
    left: i64,
    right: i64,
    bottom: i64,
}

fn extract_val(t: &str) -> i64 {
    //println!("Extracting val from {}", t);
    let t = t.to_string().replace(",", "").replace(":", "");
    t.split('=').collect::<Vec<_>>()[1].parse::<i64>().unwrap()
}

fn parse_line(l: &str) -> (Position, Position) {
    let tokens = l.split(' ').collect::<Vec<_>>();
    let x1 = extract_val(tokens[2]);
    let y1 = extract_val(tokens[3]);
    let x2 = extract_val(tokens[8]);
    let y2 = extract_val(tokens[9]);
    (Position{
        x: x1,
        y: y1,
    }, Position {
        x: x2,
        y: y2,
    })
}

struct Map {
    map: HashMap<Position, char>,
    // Sensor -> Beacon 
    closest: Vec<(Position, Position)>,
    x_min: i64, x_max: i64,
    y_min: i64, y_max: i64,
    longest_dist: i64,
}

impl Map {
    fn new() -> Self {
        Map{
            map: HashMap::new(),
            closest: vec!(),
            x_min: 0, x_max: 0,
            y_min: 0, y_max: 0,
            longest_dist: 0,
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

    fn insert_pair(&mut self, sensor: Position, beacon: Position) {
        let d = sensor.distance(&beacon);
        if d > self.longest_dist {
            self.longest_dist = d;
        }
        self.insert(sensor.clone(), 'S');
        self.insert(beacon.clone(), 'B');
        self.closest.push((sensor, beacon));
    }

    fn print(&self) {
        println!();
        for i in (0..6).rev() {
            print!("      ");
            for x in self.x_min-1..=self.x_max+2 {
                let d = (x % 10_i64.pow(i+1)) / 10_i64.pow(i);
                if (x == 0 && i == 0) || x >= 10_i64.pow(i) {
                    print!("{}", d);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        for y in self.y_min-1..=self.y_max+1 {
            print!("{:6} ", y);
            for x in self.x_min-1..=self.x_max+1 {
                let p = Position{x, y};
                let c = self.map.get(&p).or(Some(&'.')).unwrap();
                print!("{}", c);
            }
            println!();
        }
    }

    fn part1(&mut self, y: i64) {
        let mut no_beacons = 0;
        let mut to_insert = vec!();
        'outer: for x in self.x_min-self.longest_dist..=self.x_max+self.longest_dist {
            for (s, b) in self.closest.iter() {
                let p = Position{x, y};
                // Do not count positions where there is a beacon
                if let Some('B') = self.map.get(&p) {
                    continue;
                }
                if p.distance(s) <= b.distance(s) {
                    //println!("No beacon at {:?}", p);
                    no_beacons += 1;
                    to_insert.push(p.clone());
                    continue 'outer;
                }
            }
        }
        for p in to_insert {
            self.insert(p, '#');
        }
        //self.print();
        println!("Places with no beacons in row {}: {}", y, no_beacons);
    }

    fn is_covered(&self, p: &Position) -> bool {
        for (s, b) in self.closest.iter() {
            let d = s.distance(b);
            let d1 = s.distance(p);
            if d1 <= d {
                return true;
            }
        }
        false
    }

    // Because we know that there's a single point, we can look only at the points that are just at
    // the border of all balls.
    fn part2(&self, min: i64, max: i64) -> Option<Position> {
        for (s, b) in self.closest.iter() {
            let d = s.distance(b) + 1;
            println!("Sensor = {:?}, Beacon = {:?}, distance = {}", s, b, d); 
            for i in 0..=d {
                // TODO: Look at (x+d-i,y+i) (x+d-i,y-i), (x-d+i,y+i) and (x-d+i,y-i)
                let p1 = Position{
                    x: s.x+d-i,
                    y: s.y+i,
                };
                let p2 = Position{
                    x: s.x+d-i,
                    y: s.y-i,
                };
                let p3 = Position{
                    x: s.x-d+i,
                    y: s.y+i,
                };
                let p4 = Position{
                    x: s.x-d+i,
                    y: s.y-i,
                };
                //println!("  Look at {:?}, {:?}, {:?}, {:?}", p1, p2, p3, p4); 
                for p in [p1, p2, p3, p4] {
                    if p.x < min || p.y < min || p.x > max || p.y > max {
                        continue
                    }
                    if !self.is_covered(&p) {
                        println!("Found {:?} uncovered: {}", p, 4000000*p.x+p.y);
                        return Some(p);
                    }
                }
            }
        }
        None
    }

    fn part2_slow(&mut self, min: i64, max: i64) {
        for y in min..=max {
            println!("Looking at y = {}", y);
            'pos: for x in min..=max {
                let p = Position{x, y};
                let mut can_have_beacon = true;
                for (s, b) in self.closest.iter() {
                    if p.distance(s) <= b.distance(s) {
                        can_have_beacon = false;
                        continue 'pos;
                    }
                }
                if can_have_beacon {
                    println!("{:?} could have beacon - tuning frequency: {}", p, 4000000*x+y);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt"; let row = 10; let min = 0; let max = 20;
    //let filename = "my_input.txt"; let row = 2000000; let min = 0; let max = 4000000;

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut m = Map::new();

    for l in lines {
        let l = l?;
        let (s, b) = parse_line(&l);
        println!("Distance: {}", s.distance(&b));
        m.insert_pair(s, b);
    }

    println!("Map from ({}, {}) to ({}, {}) - Longest distance: {}", m.x_min, m.y_min, m.x_max, m.y_max, m.longest_dist);
    //m.print();

    //m.part1(row);
    m.part2(min, max);

    Ok(())
}
