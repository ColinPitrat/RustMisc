use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 15 of Advent of Code 2024.
struct Day15Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day15Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day15Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day15Opts {
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
        if Day15Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Copy,Debug)]
enum Cell {
    Empty,
    Wall,
    Crate,
    CrateLeft,
    CrateRight,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Crate => 'O',
            Cell::CrateLeft => '[',
            Cell::CrateRight => ']',
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone,Copy,Debug)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Movement::Up => '^',
            Movement::Right => '>',
            Movement::Down => 'v',
            Movement::Left => '<',
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
struct SimulateError(String);

impl fmt::Display for SimulateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Simulation error: {}", self.0)
    }
}

impl Error for SimulateError {}

#[derive(Clone,Debug)]
struct Robot {
    map: Vec<Vec<Cell>>,
    pos: (usize, usize),
    movements : Vec<Movement>,
}

impl fmt::Display for Robot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "{}", self.map.iter().map(|line| line.iter().map(|cell| cell.to_string()).collect::<String>()).collect::<Vec<_>>().join("\n"))
        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if self.pos == (x, y) {
                    write!(f, "@")?;
                } else {
                    write!(f, "{cell}")?;
                }
            }
            writeln!(f, "")?;
        }
        writeln!(f, "")?;
        for m in self.movements.iter() {
            write!(f, "{}", m)?;
        }
        Ok(())
    }
}

impl Robot {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut map = vec!();
        let mut pos = (0, 0);
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
                    'O' => row.push(Cell::Crate),
                    '[' => row.push(Cell::CrateLeft),
                    ']' => row.push(Cell::CrateRight),
                    '@' => {
                        pos = (x, y);
                        row.push(Cell::Empty);
                    },
                    _ => return Err(Box::new(ParseError(format!("Unexpected char {c} in map")))),
                };
            }
            map.push(row);
        }
        // The movements come after
        let mut movements = vec!();
        for line in content.split("\n").skip(map.len()+1) {
            log_verbose!("Reading movements line: {line}");
            for m in line.chars() {
                movements.push(match m {
                    '^' => Movement::Up,
                    '>' => Movement::Right,
                    'v' => Movement::Down,
                    '<' => Movement::Left,
                    _ => return Err(Box::new(ParseError(format!("Unexpected char {m} in movements")))),
                });
            }
        }
        Ok(Self{
            map,
            pos,
            movements,
        })
    }

    fn widen(&self) -> Self {
        let mut map = vec!();
        for line in self.map.iter() {
            let mut row = vec!();
            for cell in line.iter() {
                match cell {
                    Cell::Empty => {
                        row.push(Cell::Empty);
                        row.push(Cell::Empty);
                    },
                    Cell::Wall => {
                        row.push(Cell::Wall);
                        row.push(Cell::Wall);
                    },
                    Cell::Crate => {
                        row.push(Cell::CrateLeft);
                        row.push(Cell::CrateRight);
                    },
                    Cell::CrateLeft|Cell::CrateRight => {
                        panic!("Can't widen {cell}");
                    },
                }
            }
            map.push(row);
        }
        Self{
            map,
            pos: (2*self.pos.0, self.pos.1),
            movements: self.movements.clone(),
        }
    }

    fn can_push(&mut self, x: i64, y: i64, dx: i64, dy: i64) -> Result<(bool, HashSet<(usize, usize)>), Box<dyn Error>>  {
        log_verbose!("Can push at {x},{y} towards {dx},{dy}");
        let (nx, ny) = (x+dx, y+dy);
        // If we get out of the map, something is wrong. Just complain and continue.
        if nx < 0 || ny < 0 || ny >= self.map.len() as i64 || nx >= self.map[0].len() as i64 {
            return Err(Box::new(SimulateError(format!("Got out of the map at {nx}, {ny}"))));
        }
        let (nx, ny) = (nx as usize, ny as usize);
        match self.map[ny][nx] {
            Cell::Empty => Ok((true, HashSet::new())),
            Cell::Wall => Ok((false, HashSet::new())),
            Cell::Crate => self.can_push(nx as i64, ny as i64, dx, dy),
            Cell::CrateLeft => {
                log_verbose!("  (can) CrateLeft at {nx},{ny}");
                // If pushing horizontally, only check the next cell.
                // Otherwise, check both for the left side and right side of this crate.
                if dy == 0 {
                    let (ok, mut set) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    set.insert((nx, ny));
                    Ok((ok, set))
                } else {
                    let (ok1, mut set1) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    let (ok2, set2) = self.can_push(nx as i64+1, ny as i64, dx, dy)?;
                    set1.extend(set2);
                    set1.insert((nx, ny));
                    set1.insert((nx+1, ny));
                    Ok((ok1 && ok2, set1))
                }
            },
            Cell::CrateRight => {
                log_verbose!("  (can) CrateRight at {nx},{ny}");
                // If pushing horizontally, only check the next cell.
                // Otherwise defer the logic to pushing the left side of the crate.
                if dy == 0 {
                    let (ok, mut set) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    set.insert((nx, ny));
                    Ok((ok, set))
                } else {
                    let (ok1, mut set1) = self.can_push(nx as i64-1, ny as i64, dx, dy)?;
                    let (ok2, set2) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    set1.extend(set2);
                    set1.insert((nx-1, ny));
                    set1.insert((nx, ny));
                    Ok((ok1 && ok2, set1))
                }
            },
        }
    }

    fn push_crates(&mut self, x: i64, y: i64, dx: i64, dy: i64) -> Result<bool, Box<dyn Error>> {
        log_verbose!("Push at {x},{y} towards {dx},{dy}");
        let c = self.map[y as usize][x as usize];
        let (nx, ny) = (x+dx, y+dy);
        // If we get out of the map, something is wrong. Just complain and continue.
        if nx < 0 || ny < 0 || ny >= self.map.len() as i64 || nx >= self.map[0].len() as i64 {
            return Err(Box::new(SimulateError(format!("Got out of the map at {nx}, {ny}"))));
        }
        let (x, y) = (x as usize, y as usize);
        let (nx, ny) = (nx as usize, ny as usize);
        match self.map[ny][nx] {
            Cell::Empty => {
                log_verbose!("  (push) Empty at {nx},{ny}, move from {x},{y}");
                self.map[ny][nx] = c;
                self.map[y][x] = Cell::Empty;
                Ok(true)
            },
            Cell::Wall => {
                log_verbose!("  (push) Wall at {nx},{ny}");
                Ok(false)
            },
            Cell::Crate => {
                log_verbose!("  (push) Crate at {nx},{ny}");
                if self.push_crates(nx as i64, ny as i64, dx, dy)? {
                    self.map[ny][nx] = c;
                    self.map[y][x] = Cell::Empty;
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Cell::CrateLeft => {
                log_verbose!("  (push) CrateLeft at {nx},{ny}");
                // If moving towards right, we must first make some space by pushing the right
                // part.
                // If moving towards left, we'll have been called by the right part so nothing more
                // to do than pushing this cell.
                log_verbose!("    (push) Pushing at {nx},{ny}");
                let mut ok = self.push_crates(nx as i64, ny as i64, dx, dy)?;
                self.map[ny][nx] = c;
                self.map[y][x] = Cell::Empty;
                if dy != 0 {
                    log_verbose!("    (push) Pushing at {},{ny}", nx+1);
                    ok |= self.push_crates(nx as i64+1, ny as i64, dx, dy)?;
                    self.map[ny][nx+1] = self.map[y][x+1];
                    self.map[y][x+1] = Cell::Empty;
                }
                Ok(ok)
            },
            Cell::CrateRight => {
                log_verbose!("  (push) CrateRight at {nx},{ny}");
                // If moving towards right, we must first make some space by pushing the right
                // part.
                // If moving towards left, we'll have been called by the right part so nothing more
                // to do than pushing this cell.
                log_verbose!("    (push) Pushing at {nx},{ny}");
                let mut ok = self.push_crates(nx as i64, ny as i64, dx, dy)?;
                self.map[ny][nx] = c;
                self.map[y][x] = Cell::Empty;
                if dy != 0 {
                    log_verbose!("    (push) Pushing at {},{ny}", nx-1);
                    ok |= self.push_crates(nx as i64-1, ny as i64, dx, dy)?;
                    self.map[ny][nx-1] = self.map[y][x-1];
                    self.map[y][x-1] = Cell::Empty;
                }
                Ok(ok)
            },
        }
    }

    fn do_move(&mut self, to_move: HashSet<(usize, usize)>, dx: i64, dy: i64) -> Result<(), Box<dyn Error>> {
        let mut to_move = to_move.iter().collect::<Vec<_>>();
        if dx > 0 {
            to_move.sort_by_key(|&e| -(e.0 as i64));
        } else if dx < 0 {
            to_move.sort_by_key(|&e| e.0);
        } else if dy > 0 {
            to_move.sort_by_key(|&e| -(e.1 as i64));
        } else if dy < 0 {
            to_move.sort_by_key(|&e| e.1);
        } else {
            return Err(Box::new(SimulateError(format!("Unsupported movement {dx},{dy}"))));
        }
        for (x, y) in to_move.iter() {
            let (nx, ny) = (*x as i64 + dx, *y as i64 + dy);
            self.map[ny as usize][nx as usize] = self.map[*y][*x];
            self.map[*y][*x] = Cell::Empty;
        }
        Ok(())
    }

    fn simulate(&mut self) -> Result<(), Box<dyn Error>> {
        log_verbose!("{self}\n");
        for mv in self.movements.clone().iter() {
            log_verbose!("Move {mv}:");
            let (dx, dy) = match mv {
                Movement::Up => {
                    (0, -1)
                },
                Movement::Right => {
                    (1, 0)
                },
                Movement::Down => {
                    (0, 1)
                },
                Movement::Left => {
                    (-1, 0)
                },
            };
            let (nx, ny) = (self.pos.0 as i64 + dx, self.pos.1 as i64 + dy);
            // If we get out of the map, something is wrong. Just complain and continue.
            if nx < 0 || ny < 0 || ny >= self.map.len() as i64 || nx >= self.map[0].len() as i64 {
                return Err(Box::new(SimulateError(format!("Got out of the map at {nx}, {ny}"))));
            }
            match self.map[ny as usize][nx as usize] {
                Cell::Empty => {
                    log_verbose!("  (simulate) Empty at {nx},{ny}, move from {:?}", self.pos);
                    self.pos = (nx as usize, ny as usize);
                },
                Cell::Wall => {
                    log_verbose!("  (simulate) Wall at {nx},{ny}");
                },
                Cell::Crate => {
                    log_verbose!("  (simulate) Crate at {nx},{ny}");
                    if self.push_crates(nx as i64, ny as i64, dx, dy)? {
                        self.pos = (nx as usize, ny as usize);
                    }
                },
                Cell::CrateLeft => {
                    log_verbose!("  (simulate) CrateLeft at {nx},{ny}");
                    let (ok1, mut to_move1) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    let (ok2, to_move2) = self.can_push(nx as i64 + 1, ny as i64, dx, dy)?;
                    to_move1.extend(to_move2);
                    to_move1.insert((nx as usize, ny as usize));
                    to_move1.insert((nx as usize + 1, ny as usize));
                    let ok = ok1 && ok2;
                    log_verbose!("    (simulate) Can push: {ok}, {to_move1:?}");
                    if ok {
                        self.do_move(to_move1, dx, dy)?;
                        self.pos = (nx as usize, ny as usize);
                    }
                },
                Cell::CrateRight => {
                    log_verbose!("  (simulate) CrateRight at {nx},{ny}");
                    let (ok1, mut to_move1) = self.can_push(nx as i64-1, ny as i64, dx, dy)?;
                    let (ok2, to_move2) = self.can_push(nx as i64, ny as i64, dx, dy)?;
                    to_move1.extend(to_move2);
                    to_move1.insert((nx as usize - 1, ny as usize));
                    to_move1.insert((nx as usize, ny as usize));
                    let ok = ok1 && ok2;
                    log_verbose!("    (simulate) Can push: {ok}, {to_move1:?}");
                    if ok {
                        self.do_move(to_move1, dx, dy)?;
                        self.pos = (nx as usize, ny as usize);
                    }
                },
            }
            log_verbose!("{self}\n");
        }
        Ok(())
    }

    fn gps_sum(&self) -> usize {
        let mut result = 0;
        for (y, line) in self.map.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                result += match cell {
                    Cell::Crate|Cell::CrateLeft => 100*y + x,
                    _ => 0,
                };
            }
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day15Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day15Opts::get_opts().filename.as_str())?;
    let mut robot = Robot::read(content.as_str())?;
    let mut robot2 = robot.widen();

    log_verbose!("Read:\n{robot}\n");
    robot.simulate()?;
    println!("Part 1: {}", robot.gps_sum());

    robot2.simulate()?;
    println!("Part 2: {}", robot2.gps_sum());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let content = "########\n#..O.O.#\n##@.O..#\n#...O..#\n#.#.O..#\n#...O..#\n#......#\n########\n\n<^^>>>vv<v>>v<<";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(content, robot.to_string());

        robot.simulate().unwrap();

        let end_state = "########\n#....OO#\n##.....#\n#.....O#\n#.#O@..#\n#...O..#\n#...O..#\n########\n\n<^^>>>vv<v>>v<<";
        assert_eq!(end_state, robot.to_string());
        assert_eq!(2028, robot.gps_sum());
    }

    #[test]
    fn test_example_widened() {
        let content = "#######\n#...#.#\n#.....#\n#..OO@#\n#..O..#\n#.....#\n#######\n\n<vv<<^^<<^^";
        let robot = Robot::read(content).unwrap();
        let mut robot = robot.widen();

        let widened = "##############\n##......##..##\n##..........##\n##....[][]@.##\n##....[]....##\n##..........##\n##############\n\n<vv<<^^<<^^";
        assert_eq!(widened, robot.to_string());

        robot.simulate().unwrap();

        let end_state = "##############\n##...[].##..##\n##...@.[]...##\n##....[]....##\n##..........##\n##..........##\n##############\n\n<vv<<^^<<^^";
        assert_eq!(end_state, robot.to_string());
        assert_eq!(618, robot.gps_sum());
    }

    #[test]
    fn test_can_push_up() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "####\n....\n[][]\n.[].\n.@..\n\n^";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(1, 4, 0, -1).unwrap().0);

        let content = "####\n...#\n[][]\n.[].\n.@..\n\n^";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(false, robot.can_push(1, 4, 0, -1).unwrap().0);
    }

    #[test]
    fn test_can_push_down() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "..@.\n.[].\n[][]\n....\n####\n\nv";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(2, 1, 0, 1).unwrap().0);

        let content = "..@.\n.[].\n[][]\n#...\n####\n\nv";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(2, 1, 0, 1).unwrap().0);
    }

    #[test]
    fn test_widened_push_right() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});
        println!("Verbose: {}", Day15Opts::get_opts().verbose);

        let content = "@[].#\n\n>";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(1, 0, 1, 0).unwrap().0);

        robot.simulate().unwrap();

        let end_state = ".@[]#\n\n>";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(1, 0, 1, 0).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_left() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});
        println!("Verbose: {}", Day15Opts::get_opts().verbose);

        let content = "#.[]@\n\n<";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(2, 0, -1, 0).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "#[]@.\n\n<";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(2, 0, -1, 0).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_up_on_left() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "##\n..\n[]\n@.\n\n^";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(1, 2, 0, -1).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "##\n[]\n@.\n..\n\n^";
        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_up_on_right() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "##\n..\n[]\n.@\n\n^";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(1, 2, 0, -1).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "##\n[]\n.@\n..\n\n^";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(1, 1, 0, -1).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_down_on_left() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "@.\n[]\n..\n##\n\nv";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(0, 0, 0, 1).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "..\n@.\n[]\n##\n\nv";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(0, 1, 0, 1).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_up_complex() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "####\n....\n[][]\n.[].\n.@..\n\n^";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(1, 2, 0, -1).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "####\n[][]\n.[].\n.@..\n....\n\n^";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(1, 1, 0, -1).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_widened_push_down_complex() {
        Day15Opts::set_opts(Day15Opts{verbose: true, ..Default::default()});

        let content = "..@.\n.[].\n[][]\n....\n####\n\nv";
        let mut robot = Robot::read(content).unwrap();

        assert_eq!(true, robot.can_push(2, 1, 0, 1).unwrap().0);

        robot.simulate().unwrap();

        let end_state = "....\n..@.\n.[].\n[][]\n####\n\nv";
        assert_eq!(end_state, robot.to_string());

        assert_eq!(false, robot.can_push(2, 2, 0, 1).unwrap().0);

        robot.simulate().unwrap();

        assert_eq!(end_state, robot.to_string());
    }

    #[test]
    fn test_sample_part1() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let mut robot = Robot::read(content.as_str()).unwrap();

        robot.simulate().unwrap();

        assert_eq!(10092, robot.gps_sum());
    }


    #[test]
    fn test_sample_part2() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let robot = Robot::read(content.as_str()).unwrap();
        let mut robot = robot.widen();

        robot.simulate().unwrap();

        let end_state = "####################\n##[].......[].[][]##\n##[]...........[].##\n##[]........[][][]##\n##[]......[]....[]##\n##..##......[]....##\n##..[]............##\n##..@......[].[][]##\n##......[][]..[]..##\n####################\n\n";
        assert_eq!(*end_state, robot.to_string()[..end_state.len()]);

        assert_eq!(9021, robot.gps_sum());
    }
}
