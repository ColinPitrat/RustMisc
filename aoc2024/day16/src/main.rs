use argh::FromArgs;
use std::collections::{HashSet,HashMap,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 16 of Advent of Code 2024.
struct Day16Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// slow down the exectuion to observe the exploration as it goes
    #[argh(switch, short = 's')]
    slow_down: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day16Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day16Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day16Opts {
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
        if Day16Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Copy,Debug)]
enum Movement {
    Forward,
    RotateRight,
    RotateLeft,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Movement::Forward => '^',
            Movement::RotateRight => '>',
            Movement::RotateLeft => '<',
        };
        write!(f, "{}", c)
    }
}

impl Movement {
    fn all() -> Vec<Self> {
        // TODO: Could use a crate (strum, enum-iterator) or could define a macro to avoid
        // duplicating the list of movements.
        vec!(Movement::Forward, Movement::RotateRight, Movement::RotateLeft)
    }

    fn cost(&self) -> usize {
        match self {
            Movement::Forward => 1,
            Movement::RotateLeft => 1000,
            Movement::RotateRight => 1000,
        }
    }
}

#[derive(Clone,Debug)]
struct Movements {
    movements: Vec<Movement>
}

impl Movements {
    fn new() -> Self {
        Self{movements: vec!()}
    }

    fn cost(&self) -> usize {
        self.movements.iter().map(|m| m.cost()).sum()
    }

    fn push(&mut self, mv: Movement) {
        self.movements.push(mv);
    }

    fn pop(&mut self) -> Option<Movement> {
        self.movements.pop()
    }

    fn iter(&self) -> std::slice::Iter<Movement> {
        self.movements.iter()
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        write!(f, "{}", c)
    }
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
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

#[derive(Clone,Copy,Debug)]
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
    orientation: Direction,
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.pos == (x, y) {
                    write!(f, "{}", self.orientation)?;
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
        Ok(Self{
            map,
            pos,
            objective,
            orientation: Direction::Right,
        })
    }

    fn move_towards(&mut self, direction: Direction) -> bool {
        let (dx, dy) = direction.delta();
        let (x, y) = (self.pos.0 as i64, self.pos.1 as i64);
        let (nx, ny) = (x+dx, y+dy);
        if nx < 0 || ny < 0 || ny as usize > self.map.len() || nx as usize > self.map[0].len() {
            return false;
        }
        let (nx, ny) = (nx as usize, ny as usize);
        match self.map[ny][nx] {
            Cell::Empty => {
                self.pos = (nx, ny);
                true
            },
            Cell::Wall => false,
        }
    }

    fn move_forward(&mut self) -> bool {
        self.move_towards(self.orientation)
    }

    fn move_backward(&mut self) -> bool {
        self.move_towards(self.orientation.opposite())
    }

    fn do_move(&mut self, movement: Movement) -> bool {
        match movement {
            Movement::Forward => self.move_forward(),
            Movement::RotateLeft => {
                self.orientation = self.orientation.rotate_left();
                true
            },
            Movement::RotateRight => {
                self.orientation = self.orientation.rotate_right();
                true
            },
        }
    }

    fn undo_move(&mut self, movement: Movement) -> bool {
        match movement {
            Movement::Forward => self.move_backward(),
            Movement::RotateLeft => {
                self.orientation = self.orientation.rotate_right();
                true
            },
            Movement::RotateRight => {
                self.orientation = self.orientation.rotate_left();
                true
            },
        }
    }

    fn bfs(&self) -> (usize, HashMap<usize, Vec<Movements>>) {
        let mut movements_per_cost = HashMap::new();
        let mut visited = HashMap::new();
        let mut best = usize::MAX;
        let mut state = self.clone();
        visited.insert((self.pos, self.orientation), 0);
        let mut to_visit = VecDeque::from([Step{
            pos: self.pos,
            orientation: self.orientation,
            movements: Movements::new(),
        }]);
        while !to_visit.is_empty() {
            let mut step = to_visit.pop_front().unwrap();
            state.pos = step.pos;
            state.orientation = step.orientation;

            for mv in Movement::all() {
                if state.do_move(mv) {
                    log_verbose!("{state}");
                    step.movements.push(mv);
                    if state.pos == state.objective {
                        log_verbose!("Got it in {}", step.movements.cost());
                        best = std::cmp::min(best, step.movements.cost());
                        if !movements_per_cost.contains_key(&best) {
                            movements_per_cost.insert(best, Vec::new());
                        }
                        movements_per_cost.get_mut(&best).unwrap().push(step.movements.clone());
                    } else if step.movements.cost() < best && (!visited.contains_key(&(state.pos, state.orientation)) || visited[&(state.pos, state.orientation)] >= step.movements.cost()) {
                        visited.insert((state.pos, state.orientation), step.movements.cost());
                        to_visit.push_back(Step{
                            pos: state.pos,
                            orientation: state.orientation,
                            movements: step.movements.clone(),
                        });
                    }
                    if !state.undo_move(mv) {
                        panic!("Couldn't undo {mv} in state:\n{state}");
                    }
                    step.movements.pop();
                    if Day16Opts::get_opts().slow_down {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                }
            }
        }
        (best, movements_per_cost)
    }

    fn count_tiles(&self, best: usize, movements_per_cost: HashMap<usize, Vec<Movements>>) -> usize {
        let mut good_spots = HashSet::new(); 
        for movements in movements_per_cost[&best].iter() {
            let mut state = self.clone();
            good_spots.insert(state.pos);
            for movement in movements.iter() {
                state.do_move(*movement);
                good_spots.insert(state.pos);
            }
        }
        good_spots.len()
    }

    fn solve(&self) -> (usize, usize) {
        //let mut best = usize::MAX;
        //self.solve_rec(&mut Movements::new(), &mut HashMap::new(), &mut best)
        let (best, movements_per_cost) = self.bfs();
        let tiles = self.count_tiles(best, movements_per_cost);
        (best, tiles)
    }
}

#[derive(Clone,Debug)]
struct Step {
    pos: (usize, usize),
    orientation: Direction,
    movements: Movements,
}

fn main() -> Result<(), Box<dyn Error>> {
    Day16Opts::set_opts(argh::from_env());

    #[cfg(debug_assertions)]
    println!("Note: build with --release for a fast execution.");

    let content = fs::read_to_string(Day16Opts::get_opts().filename.as_str())?;
    let maze = Maze::read(content.as_str())?;
    log_verbose!("Read:\n{maze}\n");
    println!("Part 1, Part 2: {:?}", maze.solve());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let maze = Maze::read(content.as_str()).unwrap();

        assert_eq!((7036, 45), maze.solve());
    }

    #[test]
    fn test_sample2() {
        let content = fs::read_to_string("sample2.txt").unwrap();
        let maze = Maze::read(content.as_str()).unwrap();

        assert_eq!((11048, 64), maze.solve());
    }
}
