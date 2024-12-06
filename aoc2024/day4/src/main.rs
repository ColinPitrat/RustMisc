use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2024.
struct Day4Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// verbose output
    #[argh(switch)]
    use_nom: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day4Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day4Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day4Opts {
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
        if Day4Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

enum VDirection {
    NONE,
    UP,
    DOWN,
}

enum HDirection {
    NONE,
    LEFT,
    RIGHT,
}

#[derive(Clone, Debug)]
struct DirectionError;

impl fmt::Display for DirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid combination of directions provided")
    }
}

// Defaults are OK.
impl Error for DirectionError {}

impl Grid {
    fn read_string(content: &str) -> Self {
        let mut cells = vec!();
        for line in content.split("\n") {
            if line.len() == 0 {
                break;
            }
            let mut row = vec!();
            for cell in line.chars() {
                row.push(cell);
            }
            cells.push(row);
        }
        Self{cells}
    }

    fn find_token(&self, token: &str, vdirection: VDirection, hdirection: HDirection) -> Result<usize, Box<dyn Error>> {
        if let HDirection::NONE = hdirection {
            if let VDirection::NONE = vdirection {
                return Err(Box::new(DirectionError));
            }
        }
        let mut nb = 0;
        for y in 0..self.cells.len() as i32{
            for x in 0..self.cells[0].len() as i32 {
                log_verbose!("Checking pos {}, {}", x, y);
                let mut found = true;
                for j in 0..token.len() as i32 {
                    let dx = match hdirection {
                        HDirection::NONE => 0,
                        HDirection::LEFT => -1,
                        HDirection::RIGHT => 1,
                    };
                    let dy = match vdirection {
                        VDirection::NONE => 0,
                        VDirection::UP => -1,
                        VDirection::DOWN => 1,
                    };
                    if x+j*dx < 0 || x+j*dx >= self.cells[0].len() as i32 {
                        found = false;
                        break;
                    }
                    if y+j*dy < 0 || y+j*dy >= self.cells.len() as i32 {
                        found = false;
                        break;
                    }
                    if self.cells[(y+j*dy) as usize][(x+j*dx) as usize] != token.as_bytes()[j as usize] as char {
                        log_verbose!("  {} != {} at ({}, {})", self.cells[(y+j*dy) as usize][(x+j*dx) as usize], token.as_bytes()[j as usize] as char, x+dx, y+dy);
                        found = false;
                        break;
                    }
                }
                if found {
                    nb += 1;
                }
            }
        }
        Ok(nb)
    }

    fn find_all_token(&self, token: &str) -> Result<usize, Box<dyn Error>> {
        let mut result = 0;
        result += self.find_token("XMAS", VDirection::NONE, HDirection::RIGHT)?;
        result += self.find_token("XMAS", VDirection::NONE, HDirection::LEFT)?;
        result += self.find_token("XMAS", VDirection::DOWN, HDirection::NONE)?;
        result += self.find_token("XMAS", VDirection::UP, HDirection::NONE)?;
        result += self.find_token("XMAS", VDirection::DOWN, HDirection::RIGHT)?;
        result += self.find_token("XMAS", VDirection::UP, HDirection::RIGHT)?;
        result += self.find_token("XMAS", VDirection::DOWN, HDirection::LEFT)?;
        result += self.find_token("XMAS", VDirection::UP, HDirection::LEFT)?;
        Ok(result)
    }

    fn check_ms(&self, a: char, b: char) -> bool {
        (a == 'M' && b == 'S') || (a == 'S' && b == 'M')
    }

    // TODO: This is a lazy solution. Would be nice to have a generic one, working for any (odd
    // number of letters) token. Or any token that crosses at any point? Or at least any 3 letters
    // token!
    fn find_crosses(&self) -> usize {
        let mut result = 0;
        for y in 1..(self.cells.len() as i32 - 1){
            for x in 1..(self.cells[0].len() as i32-1) {
                if self.cells[y as usize][x as usize] != 'A' {
                    continue
                }
                let (a, b) = (self.cells[(y-1) as usize][(x-1) as usize], self.cells[(y+1) as usize][(x+1) as usize]);
                if !self.check_ms(a, b) {
                    continue
                }
                let (a, b) = (self.cells[(y-1) as usize][(x+1) as usize], self.cells[(y+1) as usize][(x-1) as usize]);
                if !self.check_ms(a, b) {
                    continue
                }
                result += 1;
            }
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day4Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day4Opts::get_opts().filename.as_str())?;
    let grid = Grid::read_string(content.as_str());
    println!("Number of XMAS: {}", grid.find_all_token("XMAS").unwrap());
    println!("Number of X-MAS: {}", grid.find_crosses());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let grid = Grid::read_string(fs::read_to_string("sample.txt").unwrap().as_str());
        // Find horizontally forward.
        assert_eq!(3, grid.find_token("XMAS", VDirection::NONE, HDirection::RIGHT).unwrap());
        // Find horizontally backward.
        assert_eq!(2, grid.find_token("XMAS", VDirection::NONE, HDirection::LEFT).unwrap());
        // Find vertically downward.
        assert_eq!(1, grid.find_token("XMAS", VDirection::DOWN, HDirection::NONE).unwrap());
        // Find vertically upward .
        assert_eq!(2, grid.find_token("XMAS", VDirection::UP, HDirection::NONE).unwrap());
        // Find diagonally.
        assert_eq!(1, grid.find_token("XMAS", VDirection::DOWN, HDirection::RIGHT).unwrap());
        assert_eq!(4, grid.find_token("XMAS", VDirection::UP, HDirection::RIGHT).unwrap());
        assert_eq!(1, grid.find_token("XMAS", VDirection::DOWN, HDirection::LEFT).unwrap());
        assert_eq!(4, grid.find_token("XMAS", VDirection::UP, HDirection::LEFT).unwrap());
        
        assert_eq!(18, grid.find_all_token("XMAS").unwrap());

        assert_eq!(9, grid.find_crosses());
    }
}
