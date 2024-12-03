use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2024.
struct Day3Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day3Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day3Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day3Opts {
    fn get_opts() -> Day3Opts {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Day3Opts{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Day3Opts) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day3Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Debug)]
enum Cell {
    Number{value: u32},
    Part{desc: char},
    Empty,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Number{value} => write!(f, "{}", value)?,
            Cell::Part{desc} => write!(f, "{}", desc)?,
            Cell::Empty => write!(f, ".")?,
        };
        Ok(())
    }
}
#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<Cell>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.iter() {
            for cell in line.iter() {
                write!(f, "{:>5}", format!("{}", cell))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Grid {
    fn read_string(content: &str) -> Result<Grid, Box<dyn Error>> {
        let mut cells = vec!();
        for line in content.split("\n") {
            let mut row = vec!();
            let mut reading_number = false;
            let mut number = 0;
            let mut number_length = 0;
            for cell in line.chars() {
                if cell.is_digit(10) {
                    if !reading_number {
                        reading_number = true;
                        number = 0;
                    }
                    number = 10*number + cell.to_digit(10).ok_or("not a digit")?;
                    number_length += 1;
                } else {
                    if reading_number {
                        for _ in 0..number_length {
                            row.push(Cell::Number{value: number});
                        }
                        reading_number = false;
                        number = 0;
                        number_length = 0;
                    }
                    if cell == '.' {
                        row.push(Cell::Empty)
                    } else {
                        row.push(Cell::Part{desc: cell})
                    }
                }
            }
            // TODO: Find a cleaner way to handle numbers so that I don't have to duplicate this.
            if reading_number {
                for _ in 0..number_length {
                    row.push(Cell::Number{value: number});
                }
            }
            cells.push(row);
        }
        let grid = Grid{cells};
        log_verbose!("Read grid:\n{}", grid);
        Ok(grid)
    }

    fn read_file(filename: &str) -> Result<Grid, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        log_verbose!("Reading: {}:\n{}\n", filename, content);
        Self::read_string(content.as_str())
    }

    fn find_number_around(&self, x: i32, y: i32) -> Vec<u32> {
        let mut numbers = HashSet::new();
        for dy in vec!(-1, 0, 1) {
            if y + dy < 0 || y + dy >= self.cells.len() as i32 {
                continue;
            }
            for dx in vec!(-1, 0, 1) {
                if x + dx < 0 || x + dx >= self.cells[(y+dy) as usize].len() as i32 {
                    continue;
                }
                if let Cell::Number{value} = self.cells[(y+dy) as usize][(x+dx) as usize] {
                    numbers.insert(value);
                }
            }
        }
        numbers.into_iter().collect()
    }

    fn find_parts_numbers(&self) -> Vec<u32> {
        let mut numbers = vec!();
        for (y, line) in self.cells.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if let Cell::Part{desc} = cell {
                    for &number in self.find_number_around(x as i32, y as i32).iter() {
                        numbers.push(number);
                        if number == 989 {
                            log_verbose!("989 found near {desc}");
                        }
                    }
                }
            }
        }
        numbers.sort();
        numbers
    }

    fn parts_numbers_sum(&self) -> u32 {
        self.find_parts_numbers().into_iter().sum()
    }

    fn find_gear_ratios(&self) -> Vec<u32> {
        let mut gear_ratios = vec!();
        for (y, line) in self.cells.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                if let Cell::Part{desc: '*'} = cell {
                    let numbers = self.find_number_around(x as i32, y as i32);
                    if numbers.len() == 2 {
                        gear_ratios.push(numbers[0]*numbers[1]);
                    }
                }
            }
        }
        gear_ratios.sort();
        gear_ratios
    }

    fn gear_ratios_sum(&self) -> u32 {
        self.find_gear_ratios().into_iter().sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day3Opts::set_opts(argh::from_env());

    let grid = Grid::read_file(Day3Opts::get_opts().filename.as_str())?;
    log_verbose!("Parts numbers: {:?}", grid.find_parts_numbers());
    println!("Sum of parts numbers: {}", grid.parts_numbers_sum());
    println!("Sum of gear ratios: {}", grid.gear_ratios_sum());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_at_start_of_line() {
        let content = "123...\n..$...\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(123), grid.find_parts_numbers());

        let content = "$.....\n123...\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(123), grid.find_parts_numbers());
    }

    #[test]
    fn test_number_at_end_of_line() {
        let content = "...123\n...$..\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(123), grid.find_parts_numbers());

        let content = ".....$\n...123\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(123), grid.find_parts_numbers());
    }

    #[test]
    fn test_no_adjacent_part_number() {
        let content = "$....-\n..12..\n%....#\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(Vec::<u32>::new(), grid.find_parts_numbers());

        let content = "1....2\n..#%..\n3....4\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(Vec::<u32>::new(), grid.find_parts_numbers());
    }

    #[test]
    fn test_part_numbers() {
        let content = ".$....\n..12..\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "..$...\n..12..\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "...$..\n..12..\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "....$.\n..12..\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n.$12..\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n..12$.\n......\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n..12..\n.$....\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n..12..\n..$...\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n..12..\n...$..\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());

        let content = "......\n..12..\n....$.\n";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(12), grid.find_parts_numbers());
    }

    #[test]
    fn test_gear_ratios() {
        // Two numbers next to a star: this is a gear
        let content = ".12*3...\n........";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(36), grid.find_gear_ratios());

        // Single number next to a star: not a gear
        let content = ".12*....\n........";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(Vec::<u32>::new(), grid.find_gear_ratios());

        // Three numbers next to a star: not a gear
        let content = ".12*3...\n...4....";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(Vec::<u32>::new(), grid.find_gear_ratios());

        // Two numbers next to a non-star symbol: not a gear
        let content = ".12#3...\n........";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(Vec::<u32>::new(), grid.find_gear_ratios());

        // Multiple gears
        let content = ".12*3...\n.......5\n......7*";
        let grid = Grid::read_string(content).unwrap();
        assert_eq!(vec!(35, 36), grid.find_gear_ratios());
    }

    #[test]
    fn test_sample() {
        let grid = Grid::read_file("sample.txt").unwrap();
        assert_eq!(4361, grid.parts_numbers_sum());
        assert_eq!(467835, grid.gear_ratios_sum());
    }
}
