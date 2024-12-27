use argh::FromArgs;
use std::collections::{HashSet,HashMap,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 20 of Advent of Code 2024.
struct Day20Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day20Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day20Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day20Opts {
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
        if Day20Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Cell {
    Empty,
    Wall,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
        };
        write!(f, "{}", c)
    }
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
        vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left)
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

#[derive(Clone,Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Clone,Debug)]
struct Maze {
    map: Vec<Vec<Cell>>,
    pos: (usize, usize),
    objective: (usize, usize),
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.pos == (x, y) {
                    write!(f, "X")?;
                } else if self.objective == (x, y) {
                    write!(f, "*")?;
                } else {
                    write!(f, "{cell}")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Maze {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut map = vec!();
        let mut pos = (0, 0);
        let mut objective = (0, 0);
        // The map comes first.
        for (y, line) in content.split("\n").enumerate() {
            log_verbose!("Reading map line: {line}");
            if line.is_empty() {
                break;
            }
            let mut row = vec!();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Cell::Empty),
                    '#' => row.push(Cell::Wall),
                    'S' => {
                        pos = (x, y);
                        row.push(Cell::Empty);
                    },
                    'E' => {
                        objective = (x, y);
                        row.push(Cell::Empty);
                    },
                    _ => return Err(Box::new(ParseError(format!("Unexpected char {c} in map")))),
                };
            }
            map.push(row);
        }
        log_verbose!("");
        Ok(Self{
            map,
            pos,
            objective,
        })
    }

    fn show_path(&self, path: &HashSet<(usize, usize)>) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if path.contains(&(x, y)) {
                    if self.map[y][x] == Cell::Wall {
                        print!("*");
                    } else {
                        print!("X");
                    }
                } else {
                    print!("{cell}");
                }
            }
            println!("");
        }
    }

    fn find_shortcuts(&self, memoize: &HashMap<(usize, usize), usize>, max_steps: usize) -> HashMap<usize, usize> {
        let mut result = HashMap::new();
        for (start, &dstart) in memoize.iter() {
            //log_verbose!("Looking from shortcuts starting at {start:?}");
            for dx in -(max_steps as i64)..=max_steps as i64 {
                let max_y_steps = (max_steps as i64) - dx.abs();
                //log_verbose!("  Looking from shortcuts with dx = {dx}");
                for dy in -max_y_steps..=max_y_steps {
                    let length = (dx.abs() + dy.abs()) as usize;
                    // An endpoint at the start point or an adjacent position is not a shortcut.
                    if length <= 1 {
                        continue
                    }
                    //log_verbose!("    Looking from shortcuts with dy = {dy}");
                    let (x, y) = (start.0 as i64 + dx, start.1 as i64 + dy);
                    if x < 0 || y < 0 {
                        continue;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if y >= self.map.len() || x >= self.map[y].len() {
                        continue;
                    }
                    let end = (x,y);
                    //log_verbose!("Trying shortcut from {start:?} to {end:?}");
                    if memoize.contains_key(&end) {
                        let dend = memoize[&end];
                        if dend > dstart+length {
                            let saved = dend-dstart-length;
                            if saved == 4 {
                                log_verbose!("Found shortcut saving {saved} from {start:?} to {end:?}");
                            }
                            // We have a shortcut from start to end, it saves (dend-dstart-length);
                            *result.entry(saved).or_default() += 1;
                        }
                    }
                }
            }
        }
        result
    }

    fn no_shortcut(&self, memoize: &mut HashMap<(usize, usize), usize>) -> (usize, HashSet<(usize, usize)>) {
        let mut best = usize::MAX;
        let mut best_path = HashSet::new();
        let mut to_visit = VecDeque::new();
        to_visit.push_back(Step{
            pos: self.pos,
            cost: 0,
            path: HashSet::from([self.pos]),
        });
        memoize.insert(self.pos, 0);
        while !to_visit.is_empty() {
            let step = to_visit.pop_back().unwrap();
            for direction in Direction::all() {
                let (dx, dy) = direction.delta();
                let (x, y) = (step.pos.0 as i64 + dx, step.pos.1 as i64 + dy);
                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if y >= self.map.len() || x >= self.map[y].len() {
                    continue;
                }
                if step.path.contains(&(x, y)) {
                    continue;
                }
                let mut current = self.clone();
                current.pos = (x, y);
                if self.map[y][x] == Cell::Wall {
                    continue;
                }
                let mut path = step.path.clone();
                path.insert(current.pos);
                memoize.insert(current.pos, step.cost+1);
                if current.pos == self.objective {
                    best = std::cmp::min(best, step.cost+1);
                    best_path = path.clone();
                } else {
                    to_visit.push_back(Step{
                        pos: current.pos,
                        cost: step.cost + 1,
                        path,
                    });
                }
            }
        }
        if Day20Opts::get_opts().verbose {
            log_verbose!("Best path without shortcuts:");
            self.show_path(&best_path);
        }
        (best, best_path)
    }

    fn solve(&self, max_steps: usize, min_saved: usize) -> (usize, usize, HashMap<usize, usize>) {
        let mut memoize = HashMap::new();
        let (length, _) = self.no_shortcut(&mut memoize);
        let shortcuts = self.find_shortcuts(&memoize, max_steps);
        let mut result = 0;
        for (&distance, &number) in shortcuts.iter() {
            if distance >= min_saved {
                result += number;
            }
        }
        (length, result, shortcuts)
    }
}

#[derive(Clone,Debug)]
struct Step {
    pos: (usize, usize),
    cost: usize,
    path: HashSet<(usize, usize)>,
}

fn main() -> Result<(), Box<dyn Error>> {
    Day20Opts::set_opts(argh::from_env());

    #[cfg(debug_assertions)]
    println!("Note: build with --release for a fast execution.");

    let filename = Day20Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;

    let maze = Maze::read(content.as_str())?;

    let (steps, part1, shortcuts) = maze.solve(2, 100);
    log_verbose!("Solve maze in {steps} steps");
    for (saved, number) in shortcuts {
        log_verbose!(" - {number} cheats that save(s) {saved} picoseconds.");
    }
    let (_, part2, _) = maze.solve(20, 100);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let maze = Maze::read(content.as_str()).unwrap();

        let (length, _, shortcuts) = maze.solve(2, 100);

        assert_eq!(84, length);

        println!("Shortcuts: {:?}", shortcuts);
        assert_eq!(14, shortcuts[&2]);
        assert_eq!(14, shortcuts[&4]);
        assert_eq!(2, shortcuts[&6]);
        assert_eq!(4, shortcuts[&8]);
        assert_eq!(2, shortcuts[&10]);
        assert_eq!(3, shortcuts[&12]);
        assert_eq!(1, shortcuts[&20]);
        assert_eq!(1, shortcuts[&36]);
        assert_eq!(1, shortcuts[&38]);
        assert_eq!(1, shortcuts[&40]);
        assert_eq!(1, shortcuts[&64]);
    }

    #[test]
    fn test_sample_part2() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let maze = Maze::read(content.as_str()).unwrap();

        let (length, _, shortcuts) = maze.solve(20, 100);
        assert_eq!(84, length);

        assert_eq!(3, shortcuts[&76]);
        assert_eq!(32, shortcuts[&50]);
        assert_eq!(31, shortcuts[&52]);
        assert_eq!(29, shortcuts[&54]);
        assert_eq!(39, shortcuts[&56]);
        assert_eq!(25, shortcuts[&58]);
        assert_eq!(23, shortcuts[&60]);
        assert_eq!(20, shortcuts[&62]);
        assert_eq!(19, shortcuts[&64]);
        assert_eq!(12, shortcuts[&66]);
        assert_eq!(14, shortcuts[&68]);
        assert_eq!(12, shortcuts[&70]);
        assert_eq!(22, shortcuts[&72]);
        assert_eq!(4, shortcuts[&74]);
    }
}
