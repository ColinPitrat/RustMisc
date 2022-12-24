use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Write};

#[derive(Clone,Debug,PartialEq)]
struct Connection {
    name: String,
    distance: i64,
}

#[derive(Clone,Debug)]
struct Valve {
    name: String,
    flow: i64,
    to: Vec<Connection>,
    opened: bool
}

impl Valve {
    fn from(l: &str) -> Valve{
        let tokens = l.split(' ').collect::<Vec<_>>();
        let name = tokens[1].to_string();
        let flow = tokens[4].split('=').collect::<Vec<_>>()[1].replace(";", "").parse::<i64>().unwrap();
        let to = tokens[9..].iter().map(|t| Connection{name: t.replace(",", ""), distance: 1}).collect::<Vec<_>>();
        Valve{name, flow, to, opened: false}
    }
}

#[derive(Clone,Debug)]
struct Agent {
    pos: String,
    prev_pos: String,
}

#[derive(Clone,Debug)]
struct Volcano {
    me: Agent,
    elephant: Agent,
    valves: HashMap<String, Valve>,
    current_flow: i64,
    max_flow: i64,
    pressure_released: i64,
    max_pressure_released: i64,
    opened: i64,
    to_open: i64,
    moves: i64,
}

impl Volcano {
    fn from(lines: Lines<BufReader<File>>) -> Volcano {
        let mut valves = HashMap::new();
        let mut to_open = 0;
        let mut max_flow = 0;
        for l in lines {
            let l = l.unwrap();
            let v = Valve::from(&l);
            if v.flow > 0 {
                to_open += 1;
                max_flow += v.flow;
            }
            valves.insert(v.name.clone(), v);
        }
        Volcano{
            me: Agent{
                pos: String::from("AA"),
                prev_pos: String::new(),
            },
            elephant: Agent{
                pos: String::from("AA"),
                prev_pos: String::new(),
            },
            valves,
            current_flow: 0,
            max_flow,
            pressure_released: 0,
            max_pressure_released: 0,
            opened: 0,
            to_open,
            moves: 0,
        }
    }

    fn simplify(&mut self) {
        loop {
            let mut to_replace = vec!();
            let mut more_to_do = false;
            for (_, v) in self.valves.iter() {
                if v.flow == 0 && v.to.len() == 2 {
                    let from = v.to[0].clone();
                    let to = v.to[1].clone();
                    //println!("To remove: {} (between {:?} and {:?})", v.name, from, to);
                    to_replace.push((from, v.name.clone(), to));
                    more_to_do = true;
                    break;
                }
            }
            for (f, m,  t) in to_replace.iter() {
                let new_distance = f.distance + t.distance;
                // Replace m by t in f
                //println!("Replacing {} by {:?} in {:?})", m, t, f);
                let v = self.valves.get_mut(&f.name).unwrap();
                let mut a = v.to.iter().filter(|x| x.name != *m).map(|x| x.clone()).collect::<Vec<_>>();
                a.push(Connection{name: t.name.clone(), distance: new_distance});
                v.to = a;
                // Replace m by f in t
                //println!("Replacing {} by {:?} in {:?})", m, f, t);
                let v = self.valves.get_mut(&t.name).unwrap();
                let mut a = v.to.iter().filter(|x| x.name != *m).map(|x| x.clone()).collect::<Vec<_>>();
                a.push(Connection{name: f.name.clone(), distance: new_distance});
                v.to = a;
            }
            for (_, m,  _) in to_replace.iter() {
                // Remove m
                /*
                println!("Removing {} from:", m);
                for (_, v) in self.valves.iter() {
                    println!("{:?}", v);
                }
                */
                self.valves.remove(m);
                /*
                println!("After removing {}:", m);
                for (_, v) in self.valves.iter() {
                    println!("{:?}", v);
                }
                */
            }
            if !more_to_do {
                break;
            }
        }
    }

    fn me_open_valve(&mut self) {
        self.me.prev_pos = String::new();
        self.current_flow += self.valves[&self.me.pos].flow;
        self.valves.get_mut(&self.me.pos).unwrap().opened = true;
        self.opened += 1;
    }

    fn me_close_valve(&mut self) {
        self.current_flow -= self.valves[&self.me.pos].flow;
        self.valves.get_mut(&self.me.pos).unwrap().opened = false;
        self.opened -= 1;
    }

    fn elephant_open_valve(&mut self) {
        self.elephant.prev_pos = String::new();
        self.current_flow += self.valves[&self.elephant.pos].flow;
        self.valves.get_mut(&self.elephant.pos).unwrap().opened = true;
        self.opened += 1;
    }

    fn elephant_close_valve(&mut self) {
        self.current_flow -= self.valves[&self.elephant.pos].flow;
        self.valves.get_mut(&self.elephant.pos).unwrap().opened = false;
        self.opened -= 1;
    }

    fn count_move(&mut self) {
        self.moves += 1;
        self.pressure_released += self.current_flow;
    }

    fn uncount_move(&mut self) {
        self.moves -= 1;
        self.pressure_released -= self.current_flow;
    }

    fn part1(&mut self) {
        //println!("At {} on move {}", self.me.pos, self.moves); 
        if self.moves == 30 {
            if self.pressure_released > self.max_pressure_released {
                println!("Reached {} moves with {} released", self.moves, self.max_pressure_released); 
                self.max_pressure_released = self.pressure_released;
            }
            return
        }
        if self.pressure_released + self.max_flow * (30 - self.moves) < self.max_pressure_released {
            // No chance of beating the current best
            return
        }

        self.count_move();
        if self.opened == self.to_open {
            //println!("All valves opened ({})", self.opened); 
            self.part1();
            self.uncount_move();
            return
        }
        if self.valves[&self.me.pos].flow != 0 && !self.valves[&self.me.pos].opened {
            //println!("Opening {}", self.me.pos); 
            self.me_open_valve();

            self.part1();

            self.me_close_valve();
        }
        for t in self.valves[&self.me.pos].to.clone() {
            if t.name == self.me.prev_pos {
                // Let's not go back and forth without action in between.
                // Note that we can still go in circle...
                continue;
            }
            for _ in 1..t.distance {
                if self.moves == 30 && self.pressure_released > self.max_pressure_released {
                    println!("Reached {} moves with {} released", self.moves, self.max_pressure_released); 
                    self.max_pressure_released = self.pressure_released;
                }
                self.count_move();
            }
            if self.moves <= 30 {
                //println!("Moving to {}", t); 
                let old_pos = self.me.pos.clone();
                let old_prev_pos = self.me.prev_pos.clone();
                self.me.prev_pos = self.me.pos.clone();
                self.me.pos = t.name;
                self.part1();
                self.me.pos = old_pos;
                self.me.prev_pos = old_prev_pos;
            }
            for _ in 1..t.distance {
                self.uncount_move();
            }
        }
        self.uncount_move();
    }

    fn elephant_move(&mut self) -> i64 {
        if self.valves[&self.elephant.pos].flow != 0 && !self.valves[&self.elephant.pos].opened {
            //println!("Elephant opening valve {}", self.elephant.pos); 
            let mut next_step = self.clone();
            next_step.elephant_open_valve();

            let r = next_step.part2();
            if r > self.max_pressure_released {
                self.max_pressure_released = r;
            }
        }

        //println!("Me at {}, elephant at {} on move {} - Elephant will consider {:?} at move {}", self.me.pos, self.elephant.pos, self.moves, self.valves[&self.elephant.pos].to, self.moves); 
        for t in self.valves[&self.elephant.pos].to.clone() {
            if t.name == self.elephant.prev_pos {
                // Let's not go back and forth without action in between.
                // Note that we can still go in circle...
                continue;
            }
            let mut next_step = self.clone();
            //println!("Me at {}, elephant at {} on move {} - Elephant moving to {:?}", self.me.pos, self.elephant.pos, self.moves, t); 
            next_step.elephant.prev_pos = next_step.elephant.pos.clone();
            next_step.elephant.pos = t.name;
            let r = next_step.part2();
            if r > self.max_pressure_released {
                self.max_pressure_released = r;
            }
        }
        self.max_pressure_released
    }

    fn me_and_elephant_move(&mut self) -> i64 {
        if self.valves[&self.me.pos].flow != 0 && !self.valves[&self.me.pos].opened {
            //println!("Me opening valve {}", self.me.pos); 
            let mut next_step = self.clone();
            next_step.me_open_valve();

            let r = next_step.elephant_move();
            if r > self.max_pressure_released {
                self.max_pressure_released = r;
            }
        }

        for t in self.valves[&self.me.pos].to.clone() {
            if t.name == self.me.prev_pos {
                // Let's not go back and forth without action in between.
                // Note that we can still go in circle...
                continue;
            }
            //println!("Me moving to {:?}", t); 
            let mut next_step = self.clone();
            next_step.me.prev_pos = next_step.me.pos.clone();
            next_step.me.pos = t.name;

            let r = next_step.elephant_move();
            if r > self.max_pressure_released {
                self.max_pressure_released = r;
            }
        }
        self.max_pressure_released
    }

    fn part2(&mut self) -> i64 {
        //println!("Me at {}, elephant at {} on move {}", self.me.pos, self.elephant.pos, self.moves); 
        if self.moves == 26 {
            if self.pressure_released > self.max_pressure_released {
                self.max_pressure_released = self.pressure_released;
                println!("Reached {} moves with {} released (new max)", self.moves, self.max_pressure_released); 
            }
            return self.max_pressure_released
        }
        if self.pressure_released + self.max_flow * (26 - self.moves) < self.max_pressure_released {
            // No chance of beating the current best
            return self.max_pressure_released
        }

        self.count_move();
        if self.opened == self.to_open {
            // All valves opened, nothing more to do than wait.
            //println!("All valves opened ({})", self.opened); 
            self.max_pressure_released = self.part2();
            return self.max_pressure_released
        }

        self.max_pressure_released = self.me_and_elephant_move();
        self.max_pressure_released
    }

    fn to_dot(&self) -> String {
        let mut result = String::new();
        result.push_str("digraph {");
        for (n, v) in self.valves.iter() {
            result.push_str(n);
            if v.flow == 0 {
                if n == "AA" {
                    result.push_str(" [shape=\"ellipse\" style=filled fillcolor=\"green\"]");
                } else {
                    result.push_str(" [shape=\"ellipse\"]");
                }
            } else {
                result.push_str(format!(" [label=\"\\N {}\" shape=\"rectangle\" style=filled fillcolor=\"red\"]", v.flow).as_str());
            }
            result.push_str(";\n");
        }
        for (n, v) in self.valves.iter() {
            for t in &v.to {
                result.push_str(n);
                result.push_str(" -> ");
                result.push_str(t.name.as_str());
                result.push_str(" [label=");
                result.push_str(t.distance.to_string().as_str());
                result.push_str("];\n");
            }
        }
        result.push_str("}");
        result
    }

}

fn main() -> Result<(), Box<dyn Error>>  {
    //let filename = "sample.txt";
    let filename = "my_input.txt";
    let part = 2;
    let simplify = false; // Simplifying not supported in part2 (yet?)

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut volcano = Volcano::from(lines);

    let mut file = File::create(filename.replace(".txt", ".dot"))?;
    write!(file, "{}", volcano.to_dot())?;

    if simplify {
        volcano.simplify();

        let mut file = File::create(filename.replace(".txt", "_simplified.dot"))?;
        write!(file, "{}", volcano.to_dot())?;
    }

    if part == 1 {
        volcano.part1();
    } else {
        volcano.part2();
    }

    println!("Max pressure released: {}", volcano.max_pressure_released);

    Ok(())
}
