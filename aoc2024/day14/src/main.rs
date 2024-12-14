use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::iter;
use std::sync::{LazyLock,RwLock};
use std::thread;
use std::time;

#[derive(Clone, Default, FromArgs)]
/// Solve day 14 of Advent of Code 2024.
struct Day14Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// exploration part for part 2, to be used in conjunction with verbose output
    #[argh(switch, short = 'e')]
    explore: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day14Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day14Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day14Opts {
    fn get_opts() -> Self {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Self{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Self) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day14Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Robot {
    p: (i64, i64),
    v: (i64, i64),
}

impl Robot {
    fn simulate(&mut self, seconds: i64, width: i64, height: i64) {
        // The result of the modulo can be negative, so we add the divisor to it and take the
        // modulo a second time to ensure it's positive.
        self.p.0 = (self.p.0 + self.v.0*seconds).rem_euclid(width);
        self.p.1 = (self.p.1 + self.v.1*seconds).rem_euclid(height);
    }
}

#[derive(Clone,Debug)]
struct Robots {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

fn parse_vec(vec: &str) -> Result<(i64, i64), Box<dyn Error>> {
    let coords = vec.split("=").collect::<Vec<_>>()[1].split(",").map(|x| x.parse::<i64>()).collect::<Result<Vec<_>,_>>()?;
    Ok((coords[0], coords[1]))
}

impl fmt::Display for Robots {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = iter::repeat('.').take(self.width as usize).collect::<Vec<_>>();
        let mut map = iter::repeat(line).take(self.height as usize).collect::<Vec<_>>();
        for r in self.robots.iter() {
            map[r.p.1 as usize][r.p.0 as usize] = 'X';
        }
        write!(f, "{}", map.into_iter().map(|x| x.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"))
    }
}

impl Robots {
    fn read(content: &str, width: i64, height: i64) -> Result<Self, Box<dyn Error>> {
        let mut robots = vec!();
        for line in content.split("\n") {
            if line.is_empty() {
                break;
            }
            let elems = line.split(" ").collect::<Vec<_>>();
            let p = parse_vec(elems[0])?;
            let v = parse_vec(elems[1])?;

            robots.push(Robot{p, v});
        }
        Ok(Self{
            robots,
            width,
            height,
        })
    }

    fn simulate(&mut self, seconds: i64) {
        for robot in self.robots.iter_mut() {
            robot.simulate(seconds, self.width, self.height);
        }
    }

    fn per_quadrant(&self) -> (usize, usize, usize, usize) {
        let (mut r1, mut r2, mut r3, mut r4) = (0, 0, 0 , 0);
        for (i, robot) in self.robots.iter().enumerate() {
            log_verbose!("Robot {i} is at {},{}", robot.p.0, robot.p.1);
            if robot.p.0 < self.width/2 {
                if robot.p.1 < self.height/2 {
                    log_verbose!("  that is in quadrant 1");
                    r1 += 1;
                }
                if robot.p.1 > self.height/2 {
                    log_verbose!("  that is in quadrant 2");
                    r2 += 1;
                }
            }
            if robot.p.0 > self.width/2 {
                if robot.p.1 < self.height/2 {
                    log_verbose!("  that is in quadrant 3");
                    r3 += 1;
                }
                if robot.p.1 > self.height/2 {
                    log_verbose!("  that is in quadrant 4");
                    r4 += 1;
                }
            }
        }
        (r1, r2, r3, r4)
    }

    // This method helped finding what the christmas tree looked like.
    fn candidate_tree(&self) -> bool {
        // Let's assume that if it's a tree, the top-right and top-left 20x10
        // boxes will be empty. 
        for x in 0..10 {
            for y in 0..10 {
                for r in self.robots.iter() {
                    if r.p.0 == x && r.p.1 == y {
                        return false
                    }
                    if r.p.0 == self.width-x && r.p.1 == y {
                        return false
                    }
                }
            }
        }
        true
    }
}

fn part1(robots: &mut Robots) -> usize {
    robots.simulate(100);
    let (r1, r2, r3, r4) = robots.per_quadrant();
    r1*r2*r3*r4
}

fn part2(robots: &mut Robots) -> usize {
    for i in 0..10000 {
        // Exploration part, let try to spot a tree.
        if Day14Opts::get_opts().explore {
            if robots.candidate_tree() {
                log_verbose!("After {i} seconds:");
                log_verbose!("{robots}");
                log_verbose!("");
                // Just to be sure to have time to spot it if there are 2 in a row.
                thread::sleep(time::Duration::from_millis(100));
            }
        }
        // Exploration showed that a tree is in a box and can be found easily thanks to this.
        let repr = format!("{robots}");
        if repr.contains("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX") {
            log_verbose!("After {i} seconds:");
            log_verbose!("{robots}");
            log_verbose!("");
            return i;
        }
        robots.simulate(1);
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    Day14Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day14Opts::get_opts().filename.as_str())?;
    let (width, height) = if Day14Opts::get_opts().filename == "sample.txt" {
        (11, 7)
    } else {
        (101, 103)
    };
    let robots = Robots::read(content.as_str(), width, height)?;
    log_verbose!("Reading '{robots:?}'");

    println!("Part 1: {}", part1(&mut robots.clone()));
    println!("Part 2: {}", part2(&mut robots.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let robots = Robots::read(content.as_str(), 11, 7).unwrap();

        assert_eq!(12, part1(&mut robots.clone()));
    }
}
