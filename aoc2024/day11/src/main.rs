use argh::FromArgs;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 11 of Advent of Code 2024.
struct Day11Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day11Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day11Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day11Opts {
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
        if Day11Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Stones {
    stones: Vec<usize>,
}

impl fmt::Display for Stones {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stones.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" "))
    }
}

impl Stones {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut stones = vec!();
        for part in content.split(" ") {
            stones.push(part.trim().parse::<usize>()?);
        }
        Ok(Self{stones})
    }

    fn blink_once(&mut self) -> Result<(), Box<dyn Error>> {
        let mut new_stones = vec!();
        for &stone in self.stones.iter() {
            if stone == 0 {
                new_stones.push(1);
                continue;
            }
            let str_stone = format!("{}", stone);
            let len = str_stone.len();
            if len % 2 == 0 {
                new_stones.push(str_stone[..len/2].parse::<usize>()?);
                new_stones.push(str_stone[len/2..].parse::<usize>()?);
                continue;
            }
            new_stones.push(2024*stone);
        }
        self.stones = new_stones;
        Ok(())
    }

    fn blink(&mut self, times: usize) -> Result<(), Box<dyn Error>> {
        for _ in 0..times {
            self.blink_once()?;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        self.stones.len()
    }
}

fn part1(mut stones: Stones) -> Result<(), Box<dyn Error>> {
    stones.blink(25)?;
    println!("Part 1: {}", stones.len());
    Ok(())
}

#[derive(Clone,Debug)]
struct Stones2 {
    // A map from the stone value to the number of copies there are of it.
    stones: HashMap<usize, usize>,
}

fn add_stones(new_stones: &mut HashMap<usize, usize>, stone: usize, copies: usize) {
    if new_stones.contains_key(&stone) {
        let s = new_stones.get_mut(&stone).unwrap();
        *s += copies;
    } else {
        new_stones.insert(stone, copies);
    }
}

impl Stones2 {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut stones = HashMap::new();
        for part in content.split(" ") {
            stones.insert(part.trim().parse::<usize>()?, 1);
        }
        Ok(Self{stones})
    }

    fn blink_once(&mut self) -> Result<(), Box<dyn Error>> {
        let mut new_stones = HashMap::new();
        for (stone, &copies) in self.stones.iter() {
            if *stone == 0 {
                add_stones(&mut new_stones, 1, copies);
                continue;
            }
            let str_stone = format!("{}", stone);
            let len = str_stone.len();
            if len % 2 == 0 {
                add_stones(&mut new_stones, str_stone[..len/2].parse::<usize>()?, copies);
                add_stones(&mut new_stones, str_stone[len/2..].parse::<usize>()?, copies);
                continue;
            }
            add_stones(&mut new_stones, 2024*stone, copies);
        }
        self.stones = new_stones;
        Ok(())
    }

    fn blink(&mut self, times: usize) -> Result<(), Box<dyn Error>> {
        for _ in 0..times {
            self.blink_once()?;
        }
        Ok(())
    }

    fn len(&self) -> usize {
        self.stones.values().sum()
    }
}

fn part2(mut stones: Stones2) -> Result<(), Box<dyn Error>> {
    stones.blink(75)?;
    println!("Part 2: {}", stones.len());
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    Day11Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day11Opts::get_opts().filename.as_str())?;
    let stones = Stones::read(content.as_str())?;
    log_verbose!("Initial arrangement:\n{}", stones);

    part1(stones.clone())?;
    
    let stones2 = Stones2::read(content.as_str())?;
    part2(stones2.clone())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut stones = Stones::read("0 1 10 99 999").unwrap();
        assert_eq!("0 1 10 99 999", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("1 2024 1 0 9 9 2021976", format!("{}", stones));
    }

    #[test]
    fn test_example2() {
        let mut stones = Stones::read("125 17").unwrap();
        assert_eq!("125 17", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("253000 1 7", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("253 0 2024 14168", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("512072 1 20 24 28676032", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("512 72 2024 2 0 2 4 2867 6032", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32", format!("{}", stones));
        stones.blink_once().unwrap();
        assert_eq!("2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2", format!("{}", stones));
        assert_eq!(22, stones.len());
    }

    #[test]
    fn test_example3() {
        let mut stones = Stones::read("125 17").unwrap();
        stones.blink(25).unwrap();
        assert_eq!(55312, stones.len());
    }

    #[test]
    fn test_part2() {
        let mut stones = Stones2::read("125 17").unwrap();
        stones.blink(6).unwrap();
        assert_eq!(22, stones.len());

        let mut stones = Stones2::read("125 17").unwrap();
        stones.blink(25).unwrap();
        assert_eq!(55312, stones.len());
    }
}
