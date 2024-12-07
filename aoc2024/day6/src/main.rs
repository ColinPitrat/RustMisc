use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 6 of Advent of Code 2024.
struct Day6Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day6Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day6Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day6Opts {
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
        if Day6Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone, Debug)]
struct MapParsingError {
    details: String
}

impl fmt::Display for MapParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't parse Game: {}", self.details)
    }
}

impl std::error::Error for MapParsingError {}

#[derive(Clone,Copy,Debug)]
enum Cell {
    Empty,
    Blocked,
    Visited,
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn displacement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        })
    }
}

#[derive(Clone,Debug)]
struct Map {
    guard: (usize, usize),
    cells: Vec<Vec<Cell>>,
    direction: Direction,
    visited: HashSet<(usize, usize, Direction)>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if x == self.guard.0 && y == self.guard.1 {
                    write!(f, "{}", self.direction)?;
                } else {
                    match cell {
                        Cell::Empty => write!(f, " ")?,
                        Cell::Blocked => write!(f, "#")?,
                        Cell::Visited => write!(f, ".")?,
                    };
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn read(content: &str) -> Result<Map, Box<dyn Error>> {
        let mut cells = vec!();
        let mut guard = None;
        for (y, line) in content.split('\n').enumerate() {
            if line.is_empty() {
                break;
            }
            let mut row = vec!();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    '.' => Cell::Empty,
                    '#' => Cell::Blocked,
                    '^' => {
                        guard = Some((x, y));
                        Cell::Empty
                    },
                    _ => return Err(Box::new(MapParsingError{details: format!("Player not found in map: '{}'", content)})),
                });
            }
            cells.push(row);
        }
        if let None = guard {
            return Err(Box::new(MapParsingError{
                details: format!("Player not found in map: '{}'", content)
            }));
        }
        let guard = guard.unwrap();
        Ok(Map{
            guard, cells,
            direction: Direction::Up,
            visited: HashSet::new()
        })
    }

    fn do_move(&mut self) -> bool {
        self.cells[self.guard.1][self.guard.0] = Cell::Visited;
        self.visited.insert((self.guard.0, self.guard.1, self.direction));

        let (dx, dy) = self.direction.displacement();
        let (x, y) = (self.guard.0 as i32, self.guard.1 as i32);
        // Check if getting out of the map.
        if x+dx < 0 || x+dx >= self.cells[0].len() as i32 ||
           y+dy < 0 || y+dy >= self.cells.len() as i32 {
            return false;
        }
        let (newx, newy) = ((x+dx) as usize, (y+dy) as usize);
        match self.cells[newy][newx] {
            Cell::Blocked => self.direction = self.direction.next(),
            Cell::Empty => self.guard = (newx, newy),
            Cell::Visited => self.guard = (newx, newy),
        };
        true
    }

    fn move_until_exit(&mut self) -> usize {
        let mut nb_moves = 0;
        while self.do_move() {
            nb_moves += 1;
        }
        nb_moves
    }

    fn is_looping(&self) -> bool {
        let mut map = self.clone();
        while map.do_move() {
            if map.visited.contains(&(map.guard.0, map.guard.1, map.direction)) {
                return true;
            }
        }
        return false;
    }

    fn count_visited(&self) -> usize {
        self.cells.iter().map(|row|
            row.iter().filter(|x| if let Cell::Visited = x { true } else { false }).count()
        ).sum()
    }

    fn add_block(&mut self) -> bool {
        let (dx, dy) = self.direction.displacement();
        let (x, y) = (self.guard.0 as i32, self.guard.1 as i32);
        // Check if getting out of the map.
        if x+dx < 0 || x+dx >= self.cells[0].len() as i32 ||
           y+dy < 0 || y+dy >= self.cells.len() as i32 {
            return false;
        }
        let (newx, newy) = ((x+dx) as usize, (y+dy) as usize);
        if let Cell::Empty = self.cells[newy][newx] {
            self.cells[newy][newx] = Cell::Blocked;
            true
        } else {
            false
        }
    }

    fn looping_options(&self) -> usize {
        let mut map = self.clone();
        let mut count = 0;
        loop {
            if !map.do_move() {
                break;
            }
            let mut with_block = map.clone();
            if !with_block.add_block() {
                continue
            }
            if with_block.is_looping() {
                count += 1;
            }
        }
        count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day6Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day6Opts::get_opts().filename.as_str())?;
    let map = Map::read(content.as_str())?;

    let mut part1 = map.clone();
    log_verbose!("Map:\n{}", part1);
    part1.move_until_exit();
    log_verbose!("Map:\n{}", part1);
    println!("Cells visited: {}", part1.count_visited());

    println!("Options for adding blocks to create a loop: {}", map.looping_options());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_looping() {
        // .#..
        // .^.#
        // #...
        // ..#.
        let map = Map::read(".#..\n.^.#\n#...\n..#.").unwrap();
        assert_eq!(true, map.is_looping());

        // .#..
        // .^.#
        // ....
        // ..#.
        let mut map = Map::read(".#..\n.^.#\n....\n..#.").unwrap();
        assert_eq!(false, map.is_looping());
        assert_eq!(7, map.move_until_exit());
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = Map::read(content.as_str()).unwrap();

        let mut part1 = map.clone();
        assert_eq!(0, part1.count_visited());
        assert_eq!(54, part1.move_until_exit());
        assert_eq!(41, part1.count_visited());

        assert_eq!(6, map.looping_options());
    }
}
