use argh::FromArgs;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 8 of Advent of Code 2024.
struct Day8Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day8Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day8Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day8Opts {
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
        if Day8Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug,PartialEq,Eq,Hash)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Clone,Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: HashMap<char, Vec<Location>>
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = vec!();
        for _y in 0..self.height {
            let mut line = vec!();
            for _x in 0..self.width {
                line.push('.');
            }
            result.push(line);
        }
        for kind in self.antennas.keys() {
            for loc in self.antennas.get(kind).unwrap() {
                result[loc.y][loc.x] = *kind;
            }
        }
        for node in self.find_harmonic_nodes() {
            result[node.y][node.x] = '*';
        }
        for node in self.find_nodes() {
            result[node.y][node.x] = '#';
        }
        for line in result.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn read(content: &str) -> Self {
        let mut antennas = HashMap::new();
        let lines = content.split("\n").filter(|l| !l.is_empty()).collect::<Vec<_>>();
        for (y, line) in lines.iter().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if cell != '.' {
                    antennas.entry(cell).or_insert(vec!()).push(Location{x, y});
                }
            }
        }
        let width = lines[0].len();
        let height = lines.len();
        Self{width, height, antennas}
    }

    fn symmetric(&self, l1: &Location, l2: &Location) -> Option<Location> {
        let newx = l2.x as i64 + l2.x as i64 - l1.x as i64;
        let newy = l2.y as i64 + l2.y as i64 - l1.y as i64;
        log_verbose!("     Candidate is {},{}", newx, newy);
        if newx >= 0 && newy >= 0 && newx < self.width as i64 && newy < self.height as i64 {
            Some(Location{x: newx as usize, y: newy as usize})
        } else {
            None
        }
    }

    fn aligned(&self, l1: &Location, l2: &Location) -> Vec<Location> {
        let mut result = vec!();
        let dx = l2.x as i64 - l1.x as i64;
        let dy = l2.y as i64 - l1.y as i64;

        // It is not clear from the problem statement whether we should divide by the gcd of dx &
        // dy or not.
        // The "harmonic" naming suggests not to but this part suggests otherwise:
        // "an antinode occurs at any grid position exactly in line with at least two antennas of
        // the same frequency, regardless of distance"
        // The situation where the gcd is not 1 doesn't occur in the sample nor in my input, so
        // I'm just not doing it. But the alternative would be:
        // let dx = dx/gcd(dx, dy);
        // let dy = dy/gcd(dx, dy);
        let mut n = -max((l1.x as i64 / dx).abs(), (l2.y as i64 / dy).abs());
        // We can be out of the grid when we start but we must go in it at some point. We can stop
        // whenever we're out again.
        let mut was_in = false;
        loop {
            let newx = l1.x as i64 + n*dx;
            let newy = l1.y as i64 + n*dy;
            n += 1;
            log_verbose!("    Candidate: {}, {}", newx, newy);
            let is_out = newx < 0 || newy < 0 || newx >= self.width as i64 || newy >= self.height as i64;
            if was_in && is_out {
                break;
            }
            if !is_out {
                was_in = true;
                result.push(Location{x: newx as usize, y: newy as usize});
            }
        }
        result
    }

    fn find_nodes(&self) -> Vec<Location> {
        let mut all_locations = HashSet::new();
        for kind in self.antennas.keys() {
            for a in self.antennas.get(&kind).unwrap().iter() {
                for b in self.antennas.get(&kind).unwrap().iter() {
                    log_verbose!(" Looking for node for {:?} and {:?}", a, b);
                    if a == b {
                        continue;
                    }
                    if let Some(p) = self.symmetric(a, b) {
                        log_verbose!("   Found {:?}", p);
                        all_locations.insert(p);
                    }
                    if let Some(p) = self.symmetric(b, a) {
                        log_verbose!("   Found {:?}", p);
                        all_locations.insert(p);
                    }
                }
            }
        }
        all_locations.into_iter().collect::<Vec<_>>()
    }

    fn find_harmonic_nodes(&self) -> Vec<Location> {
        let mut all_locations = HashSet::new();
        for kind in self.antennas.keys() {
            log_verbose!("Looking at antennas of type {}", kind);
            for a in self.antennas.get(&kind).unwrap().iter() {
                for b in self.antennas.get(&kind).unwrap().iter() {
                    log_verbose!(" Looking for harmonic node for {:?} and {:?}", a, b);
                    if a == b {
                        continue;
                    }
                    for p in self.aligned(a, b) {
                        log_verbose!("   Found {:?}", p);
                        all_locations.insert(p);
                    }
                }
            }
        }
        all_locations.into_iter().collect::<Vec<_>>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day8Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day8Opts::get_opts().filename.as_str())?;
    let map = Map::read(content.as_str());
    log_verbose!("Map: {:?}", map);
    let locations1 = map.find_nodes();
    log_verbose!("Locations 1: {:?}", locations1);
    let locations2 = map.find_harmonic_nodes();
    log_verbose!("Locations 2: {:?}", locations2);
    println!("{}", map);

    println!("Number of locations (part 1): {:?}", locations1.len());
    println!("Number of locations (part 2): {:?}", locations2.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric() {
        let map = Map::read("......\n......\n......\n......\n......\n......\n");

        assert_eq!(Location{x: 4, y: 4}, map.symmetric(&Location{x: 2, y: 2}, &Location{x: 3, y: 3}).unwrap());
        assert_eq!(Location{x: 1, y: 1}, map.symmetric(&Location{x: 3, y: 3}, &Location{x: 2, y: 2}).unwrap());

        assert_eq!(Location{x: 4, y: 2}, map.symmetric(&Location{x: 0, y: 0}, &Location{x: 2, y: 1}).unwrap());
        assert_eq!(Location{x: 2, y: 4}, map.symmetric(&Location{x: 0, y: 0}, &Location{x: 1, y: 2}).unwrap());
        assert_eq!(Location{x: 5, y: 2}, map.symmetric(&Location{x: 1, y: 2}, &Location{x: 3, y: 2}).unwrap());
        assert_eq!(Location{x: 5, y: 4}, map.symmetric(&Location{x: 1, y: 2}, &Location{x: 3, y: 3}).unwrap());

        assert_eq!(true, map.symmetric(&Location{x: 2, y: 1}, &Location{x: 0, y: 0}).is_none());
        assert_eq!(true, map.symmetric(&Location{x: 4, y: 5}, &Location{x: 2, y: 2}).is_none());
        assert_eq!(true, map.symmetric(&Location{x: 5, y: 4}, &Location{x: 2, y: 2}).is_none());
    }

    macro_rules! assert_vec_eq {
        ($a:expr, $b:expr) => {{
            assert_eq!($a.iter().collect::<HashSet<_>>(), $b.iter().collect::<HashSet<_>>());
        }};
    }

    #[test]
    fn test_aligned() {
        let map = Map::read("......\n......\n......\n......\n......\n......\n");

        let expected = vec!(
                Location{x: 0, y: 0}, Location{x: 1, y: 1}, Location{x: 2, y: 2},
                Location{x: 3, y: 3}, Location{x: 4, y: 4}, Location{x: 5, y: 5}
        );
        assert_vec_eq!(expected, map.aligned(&Location{x: 2, y: 2}, &Location{x: 3, y: 3}));
        assert_vec_eq!(expected, map.aligned(&Location{x: 3, y: 3}, &Location{x: 2, y: 2}));

        let expected = vec!(Location{x: 0, y: 0}, Location{x: 2, y: 1}, Location{x: 4, y: 2});
        assert_vec_eq!(expected, map.aligned(&Location{x: 0, y: 0}, &Location{x: 2, y: 1}));
        assert_vec_eq!(expected, map.aligned(&Location{x: 2, y: 1}, &Location{x: 0, y: 0}));

        // This is the test that would behave differently if we were to divide dx & dy by the GCD.
        // It would have 0,0; 1,1; 2,2; 3,3; 4,4 and 5,5.
        let expected = vec!(Location{x: 1, y: 1}, Location{x: 3, y: 3}, Location{x: 5, y: 5});
        assert_vec_eq!(expected, map.aligned(&Location{x: 1, y: 1}, &Location{x: 3, y: 3}));
        assert_vec_eq!(expected, map.aligned(&Location{x: 3, y: 3}, &Location{x: 1, y: 1}));

    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = Map::read(content.as_str());

        let locations1 = map.find_nodes();
        assert_eq!(14, locations1.len());

        let locations2 = map.find_harmonic_nodes();
        assert_eq!(34, locations2.len());
    }
}
