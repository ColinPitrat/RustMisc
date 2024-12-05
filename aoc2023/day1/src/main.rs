use argh::FromArgs;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 1 of Advent of Code 2023.
struct Day1Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day1Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day1Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day1Opts {
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
        if Day1Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

fn first_digit(line: &str) -> u32 {
    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
    }
    0
}

fn last_digit(line: &str) -> u32 {
    first_digit(line.chars().rev().collect::<String>().as_str())
}

static NUMBERS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn first_digit2(line: &str) -> u32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for number in NUMBERS.iter() {
            if i+number.0.len() > line.len() {
                continue;
            }
            if &line[i..i+number.0.len()] == number.0 {
                return number.1
            }
        }
    }
    0
}

fn last_digit2(line: &str) -> u32 {
    for (i, c) in line.chars().enumerate().collect::<Vec<_>>().iter().rev() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap()
        }
        for number in NUMBERS.iter() {
            if i+number.0.len() > line.len() {
                continue;
            }
            if &line[*i..i+number.0.len()] == number.0 {
                return number.1
            }
        }
    }
    0
}

fn part1(content: &str) -> u32 {
    let mut result = 0;
    for line in content.split('\n') {
        let number = 10*first_digit(line) + last_digit(line);
        log_verbose!("{} (from {})", number, line);
        result += number;
    }
    result
}

fn part2(content: &str) -> u32 {
    let mut result = 0;
    for line in content.split('\n') {
        let number = 10*first_digit2(line) + last_digit2(line);
        log_verbose!("{} (from {})", number, line);
        result += number;
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    Day1Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day1Opts::get_opts().filename.as_str())?;
    println!("Part 1: {}", part1(content.as_str()));
    println!("Part 2: {}", part2(content.as_str()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        assert_eq!(1, first_digit("1234"));
        assert_eq!(1, first_digit("azerty1234uiop"));
        assert_eq!(1, first_digit("azerty1"));
        assert_eq!(1, first_digit("1"));
    }

    #[test]
    fn test_last_digit() {
        assert_eq!(4, last_digit("1234"));
        assert_eq!(4, last_digit("azerty1234uiop"));
        assert_eq!(1, last_digit("azerty1"));
        assert_eq!(1, last_digit("1"));
    }

    #[test]
    fn test_first_digit2() {
        assert_eq!(1, first_digit2("1234"));
        assert_eq!(1, first_digit2("azerty1234uiop"));
        assert_eq!(1, first_digit2("azerty1"));
        assert_eq!(1, first_digit2("1"));

        assert_eq!(1, first_digit2("one"));
        assert_eq!(2, first_digit2("zztwo"));
        assert_eq!(3, first_digit2("threezz"));
        assert_eq!(4, first_digit2("zzfourzz"));
        assert_eq!(5, first_digit2("five"));
        assert_eq!(6, first_digit2("zzsixyysevenxx"));
        assert_eq!(7, first_digit2("zzsevenyyeight"));
        assert_eq!(8, first_digit2("zzeightynine"));
        assert_eq!(9, first_digit2("zznineyyeight"));

        assert_eq!(1, first_digit2("zzone2three4five"));
    }

    #[test]
    fn test_last_digit2() {
        assert_eq!(4, last_digit2("1234"));
        assert_eq!(4, last_digit2("azerty1234uiop"));
        assert_eq!(1, last_digit2("azerty1"));
        assert_eq!(1, last_digit2("1"));

        assert_eq!(1, last_digit2("one"));
        assert_eq!(1, last_digit2("zzone"));
        assert_eq!(1, last_digit2("onezz"));
        assert_eq!(1, last_digit2("zzonezz"));
        assert_eq!(5, last_digit2("zzone2three4five"));
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample1.txt").unwrap();
        assert_eq!(142, part1(content.as_str()));

        let content = fs::read_to_string("sample2.txt").unwrap();
        assert_eq!(281, part2(content.as_str()));
    }
}
