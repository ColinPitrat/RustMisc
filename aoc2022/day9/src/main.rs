use derive_more::Display;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Display, Clone)]
struct ParameterError(String);

impl Error for ParameterError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Knot {
        Knot{x, y}
    }

    fn do_move(&mut self, direction: &str) -> Result<(), Box<dyn Error>> {
        match direction {
            "U" => {
                self.y += 1;
                Ok(())
            },
            "D" => {
                self.y -= 1;
                Ok(())
            },
            "L" => {
                self.x -= 1;
                Ok(())
            },
            "R" => {
                self.x += 1;
                Ok(())
            },
            _ => {
                Err(Box::new(ParameterError(format!("Unsupported direction {}", direction))))
            }
        }
    }

    fn follow(&mut self, other: &Knot) {
        if (other.x-self.x).abs() <= 1 && (other.y-self.y).abs() <= 1 {
            // No need to catch-up
        } else if other.y == self.y {
            // Catch-up horizontally
            if self.x < other.x - 1 {
                self.x += 1;
            }
            if self.x > other.x + 1 {
                self.x -= 1;
            }
        } else if other.x == self.x {
            // Catch-up vertically
            if self.y < other.y - 1 {
                self.y += 1;
            }
            if self.y > other.y + 1 {
                self.y -= 1;
            }
        } else {
            // Catch-up diagonally
            if self.x < other.x && self.y < other.y {
                self.x +=1;
                self.y += 1;
            } else if self.x < other.x && self.y > other.y {
                self.x +=1;
                self.y -= 1;
            } else if self.x > other.x && self.y < other.y {
                self.x -=1;
                self.y += 1;
            } else if self.x > other.x && self.y > other.y {
                self.x -=1;
                self.y -= 1;
            }
        }
    }

}

fn main() -> Result<(), Box<dyn Error>>  {
    //let filename = "sample.txt";
    let filename = "sample2.txt"; // Sample from Part 2
    //let filename = "my_input.txt";

    let mut h = Knot::new(0, 0);
    //let mut t = Knot::new(0, 0);
    let mut rope = vec!();
    //let rope_length = 2; // Part 1
    let rope_length = 10; // Part 2
    for i in 0..rope_length-1 {
        rope.push(Knot::new(0, 0));
    }

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut visited = HashSet::new();
    for l in lines {
        let l = l?;
        println!("Movement: {}", l);
        let parts = l.split(' ').collect::<Vec<_>>();
        let direction = parts[0];
        let steps = parts[1].parse::<i32>()?;

        for _ in 0..steps {
            // Move H
            h.do_move(direction)?;
            // The rest of the rope follows
            let mut prev: &Knot = &h;
            for k in rope.iter_mut() {
                k.follow(&prev);
                prev = k;
            }

            let t = rope.last().unwrap();
            visited.insert(t.clone());
            println!("H at {:?}, T at {:?}", h, t);
        }
    }

    println!("T visited {} different places", visited.len());
    Ok(())
}
