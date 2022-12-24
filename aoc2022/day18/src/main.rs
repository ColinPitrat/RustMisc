use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn neighbours(&self) -> Vec<Point> {
        let mut result = vec!();
        for d in [-1,1] {
            result.push(Point{
                x: self.x + d,
                y: self.y,
                z: self.z,
            });
            result.push(Point{
                x: self.x,
                y: self.y + d,
                z: self.z,
            });
            result.push(Point{
                x: self.x,
                y: self.y,
                z: self.z + d,
            });
        }
        return result;
    }

    fn _path_to_exterior(&self, visited: &mut HashSet<Point>, points: &HashSet<Point>, bound: &Bounding) -> bool {
        //println!("path_to_exterior: visited: {:?}, point: {:?}, bound: {:?}", visited, self, bound);
        visited.insert(self.clone());
        if self.x < bound.min_x || self.x > bound.max_x || self.y < bound.min_y || self.y > bound.max_y || self.z < bound.min_z || self.z > bound.max_z {
            return true;
        }
        for n in self.neighbours() {
            if visited.contains(&n) {
                continue;
            }
            if !points.contains(&n) {
                if n._path_to_exterior(visited, points, bound) {
                    return true;
                }
            }
        }
        false
    }

    // TODO: Add memoization to make it much faster
    fn path_to_exterior(&self, points: &HashSet<Point>, bound: &Bounding) -> bool {
        let mut visited = HashSet::new();
        self._path_to_exterior(&mut visited, points, bound)
    }
}

#[derive(Debug)]
struct Bounding {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Bounding {
    fn new() -> Bounding {
        Bounding {
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }

    fn update(&mut self, x: i64, y: i64, z: i64) {
        if x <= self.min_x {
            self.min_x = x - 1;
        }
        if x >= self.max_x {
            self.max_x = x + 1
        }
        if y <= self.min_y {
            self.min_y = y - 1;
        }
        if y >= self.max_y {
            self.max_y = y + 1
        }
        if z <= self.min_z {
            self.min_z = z - 1;
        }
        if z >= self.max_z {
            self.max_z = z + 1
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    let mut points = HashSet::new();
    let mut bounding = Bounding::new();

    for l in lines {
        let l = l?;
        let (x, y, z) = l.split(',').map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap();
        points.insert(Point{x, y, z});
        bounding.update(x, y, z);
    }

    println!("{:?}", points);
    println!("{:?}", bounding);

    let mut faces = 0;
    for p in &points {
        for n in p.neighbours() {
            if !points.contains(&n) {
                faces += 1;
            }
        }
    }

    println!("Part 1: Free faces: {}", faces);

    let mut faces = 0;
    for p in &points {
        for n in p.neighbours() {
            if !points.contains(&n) {
                if n.path_to_exterior(&points, &bounding) {
                    //println!("Path to exterior for {:?}", n);
                    faces += 1;
                }
            }
        }
    }
    println!("Part 2: Free faces: {}", faces);

    Ok(())
}
