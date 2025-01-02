use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 10 of Advent of Code 2024.
struct Day10Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day10Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day10Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day10Opts {
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
        if Day10Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Map {
    cells: Vec<Vec<usize>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in &self.cells {
            for c in line {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut cells = vec!();
        for line in content.split("\n") {
            if line.is_empty() {
                continue;
            }
            let mut row = vec!();
            for c in line.chars() {
                row.push(c.to_string().parse::<usize>()?);
            }
            cells.push(row);
        }
        Ok(Map{cells})
    }

    #[allow(dead_code)]
    fn starting_places(&self) -> Vec<(usize, usize)> {
        let mut result = vec!();
        for (y, line) in self.cells.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if *cell == 0 {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn at(&self, x: usize, y: usize) -> usize {
        self.cells[y][x]
    }

    fn heads_from(&self, (from_x, from_y): (usize, usize)) -> HashSet<(usize, usize)> {
        let value = self.at(from_x, from_y);
        if value == 9 {
            return HashSet::from([(from_x, from_y)]);
        }
        let mut result = HashSet::new();
        if from_x > 0 && self.at(from_x-1, from_y) == value+1{
            result.extend(self.heads_from((from_x-1, from_y)));
        }
        if from_y > 0 && self.at(from_x, from_y-1) == value+1 {
            result.extend(self.heads_from((from_x, from_y-1)));
        }
        if from_x < self.cells[0].len()-1 && self.at(from_x+1, from_y) == value+1 {
            result.extend(self.heads_from((from_x+1, from_y)));
        }
        if from_y < self.cells.len()-1 && self.at(from_x, from_y+1) == value+1 {
            result.extend(self.heads_from((from_x, from_y+1)));
        }
        result
    }

    fn part1(&self) -> usize {
        let mut result = 0;
        for (y, line) in self.cells.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                log_verbose!("Contribution from {}, {}: {}", x, y, self.heads_from((x, y)).len());
                if self.at(x, y) == 0 {
                    result += self.heads_from((x, y)).len();
                }
            }
        }
        result
    }

    fn paths_from(&self, (from_x, from_y): (usize, usize)) -> usize {
        let value = self.at(from_x, from_y);
        if value == 9 {
            return 1;
        }
        let mut result = 0;
        if from_x > 0 && self.at(from_x-1, from_y) == value+1{
            result += self.paths_from((from_x-1, from_y));
        }
        if from_y > 0 && self.at(from_x, from_y-1) == value+1 {
            result += self.paths_from((from_x, from_y-1));
        }
        if from_x < self.cells[0].len()-1 && self.at(from_x+1, from_y) == value+1 {
            result += self.paths_from((from_x+1, from_y));
        }
        if from_y < self.cells.len()-1 && self.at(from_x, from_y+1) == value+1 {
            result += self.paths_from((from_x, from_y+1));
        }
        result
    }

    fn part2(&self) -> usize {
        let mut result = 0;
        for (y, line) in self.cells.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                log_verbose!("Contribution from {}, {}: {}", x, y, self.paths_from((x, y)));
                if self.at(x, y) == 0 {
                    result += self.paths_from((x, y));
                }
            }
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day10Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day10Opts::get_opts().filename.as_str())?;
    let map = Map::read(content.as_str())?;
    log_verbose!("{}", map);

    println!("Part 1: {}", map.part1());
    println!("Part 2: {}", map.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_vec_eq {
        ($a:expr, $b:expr) => {{
            assert_eq!($a.iter().collect::<HashSet<_>>(), $b.iter().collect::<HashSet<_>>());
        }};
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = Map::read(content.as_str()).unwrap();

        assert_vec_eq!(vec!((2,0), (4,0), (4,2), (6,4), (2,5), (5,5), (0,6), (6,6), (1,7)), map.starting_places());

        // Reaching the 9 at (1, 0) from:
        //  - itself
        assert_eq!(HashSet::from([(1, 0)]), map.heads_from((1, 0)));
        assert_eq!(1, map.paths_from((1, 0)));
        //  - the 8 at (0, 0)
        assert_eq!(HashSet::from([(1, 0)]), map.heads_from((0, 0)));
        assert_eq!(1, map.paths_from((0, 0)));

        // Two different 9s are reachable from the 7 at (1, 2) and the one at (0, 1)
        assert_eq!(HashSet::from([(1, 0), (0, 3)]), map.heads_from((1, 2)));
        assert_eq!(2, map.paths_from((1, 2)));
        assert_eq!(HashSet::from([(1, 0), (0, 3)]), map.heads_from((0, 1)));
        assert_eq!(3, map.paths_from((0, 1)));

        assert_eq!(HashSet::from([(4, 3), (5, 4), (0, 3), (4, 5), (1, 0)]), map.heads_from((2, 0)));
        assert_eq!(20, map.paths_from((2, 0)));

        assert_eq!(36, map.part1());
        assert_eq!(81, map.part2());
    }
}
