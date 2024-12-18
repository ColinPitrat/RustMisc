use argh::FromArgs;
use itertools::Itertools;
use std::collections::{HashMap,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 18 of Advent of Code 2024.
struct Day18Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// very_verbose output
    #[argh(switch, short = 'V')]
    very_verbose: bool,

    /// slow down the exectuion to observe the exploration as it goes
    #[argh(switch, short = 's')]
    slow_down: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day18Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day18Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day18Opts {
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
        if Day18Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn all() -> Vec<Self> {
        vec!(Direction::Up, Direction::Left, Direction::Down, Direction::Right)
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum Cell {
    Empty,
    Blocked
}

#[derive(Clone,Debug)]
struct Map {
    map: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    pos: (usize, usize),
    objective: (usize, usize),
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, line) in self.map.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if (x, y) == self.pos {
                    write!(f, "O")?;
                    continue;
                }
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Blocked => '#',
                };
                write!(f, "{c}")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn read(content: &str, width: usize, height: usize, max_bytes: usize) -> Result<Self, Box<dyn Error>> {
        let mut map = vec!();
        for _ in 0..height {
            map.push(vec![Cell::Empty; height]);
        }
        for (i, line) in content.split('\n').enumerate() {
            if line.is_empty() || i >= max_bytes {
                continue;
            }
            let (x, y) = line.split(',').map(|c| c.parse::<usize>()).collect_tuple().ok_or(format!("Not a tuple for {line}"))?;
            map[y?][x?] = Cell::Blocked;
        }
        let pos = (0, 0);
        let objective = (width-1, height-1);
        Ok(Map{map, width, height, pos, objective})
    }

    fn show_path(&self, path: &Vec<(usize, usize)>) -> String {
        let mut result = String::new();
        for (y, line) in self.map.iter().enumerate() {
            'outer: for (x, cell) in line.iter().enumerate() {
                for p in path {
                    if (x, y) == *p {
                        result.push('O');
                        continue 'outer
                    }
                }
                let c = match cell {
                    Cell::Empty => '.',
                    Cell::Blocked => '#',
                };
                result.push(c);
            }
            result.push('\n');
        }
        result
    }

    fn dfs(&self) -> usize {
        let mut state = self.clone();
        let mut visited = HashMap::new();
        let mut to_visit = VecDeque::from([Step{
            pos: self.pos,
            cost: 0,
            path: vec!(self.pos),
        }]);
        let mut best = usize::MAX;
        let mut best_path = vec!();
        while !to_visit.is_empty() {
            // To do a DFS, we use pop_back. If we wanted a BFS, we would need pop_front instead.
            // In this case, DFS is faster because it gets us a lower boundary quickly.
            let current = to_visit.pop_back().unwrap();
            state.pos = current.pos;
            visited.insert(current.pos, current.cost);
            for dir in Direction::all() {
                let (x, y) = (state.pos.0 as i64 + dir.delta().0, state.pos.1 as i64 + dir.delta().1);
                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if x >= self.width || y >= self.height {
                    continue;
                }
                if self.map[y][x] == Cell::Blocked {
                    continue;
                }
                state.pos = (x as usize, y as usize);
                let cost = current.cost + 1;
                if cost > best {
                    continue;
                }
                let mut path = current.path.clone();
                path.push(state.pos);
                if !visited.contains_key(&state.pos) || visited[&state.pos] > cost {
                    if state.pos == self.objective && cost < best {
                        best = cost;
                        best_path = path.clone();
                    }
                    to_visit.push_back(Step{
                        pos: state.pos,
                        cost,
                        path,
                    });
                }
                state.pos = current.pos;
            }
            if Day18Opts::get_opts().very_verbose {
                log_verbose!("{state}");
            }
            if Day18Opts::get_opts().slow_down {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
        log_verbose!("{}", self.show_path(&best_path));
        best
    }
}

fn bisect(content: &str, width: usize, height: usize, low: usize) -> Result<String, Box<dyn Error>> {
    let (mut low, mut high) = (low, content.split('\n').collect::<Vec<_>>().len());

    while low < high {
        let current = (high + low)/2;
        log_verbose!("Trying with {current} bytes");
        let map = Map::read(content, width, height, current)?;
        if map.dfs() == usize::MAX {
            high = current;
        } else {
            low = current+1;
        }
    }

    let byte = content.split('\n').collect::<Vec<_>>()[high-1];
    Ok(byte.to_string())
}

#[derive(Clone,Debug)]
struct Step {
    pos: (usize, usize),
    cost: usize,
    path: Vec<(usize, usize)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    Day18Opts::set_opts(argh::from_env());

    #[cfg(debug_assertions)]
    println!("Note: build with --release for faster execution.");

    let filename = Day18Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;
    let (width, height, max_bytes) = if filename == "sample.txt" {
        (7, 7, 12)
    } else {
        (71, 71, 1024)
    };

    let map = Map::read(content.as_str(), width, height, max_bytes)?;
    log_verbose!("{map}");
    println!("Part 1: {}", map.dfs());

    println!("Part 2: {}", bisect(content.as_str(), width, height, max_bytes)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = Map::read(content.as_str(), 7, 7, 12).unwrap();

        assert_eq!(22, map.dfs());
        assert_eq!("6,1".to_string(), bisect(content.as_str(), 7, 7, 12).unwrap());
    }
}
