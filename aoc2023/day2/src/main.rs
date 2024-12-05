use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2023.
struct Day2Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day2Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day2Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day2Opts {
    fn get_opts() -> Self {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Self{
                ..Default::default()
            }
        } }

    fn set_opts(opts: Self) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day2Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone, Debug)]
struct GameParsingError {
    details: String
}

impl fmt::Display for GameParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't parse Game: {}", self.details)
    }
}

impl std::error::Error for GameParsingError {}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    max: (usize, usize, usize),
}

fn read_games(content: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let mut result = vec!();
    for line in content.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut max = (0, 0, 0);
        let elems = line.split(':').collect::<Vec<_>>();
        if elems[0][..5] != *"Game " {
            return Err(Box::new(GameParsingError{
                details: format!("Line doesn't start with 'Game ': '{}'", line)
            }));
        }
        let id = elems[0][5..].parse::<usize>()?;
        for round in elems[1].split(';') {
            for step in round.split(',') {
                let elems = step.split(' ').collect::<Vec<_>>();
                let num = elems[1].parse::<usize>()?;
                match elems[2] {
                    "red" => {
                        if num > max.0 {
                            max.0 = num;
                        }
                    },
                    "green" => {
                        if num > max.1 {
                            max.1 = num;
                        }
                    },
                    "blue" => {
                        if num > max.2 {
                            max.2 = num;
                        }
                    },
                    _ => {
                        return Err(Box::new(GameParsingError{
                            details: format!("Unexpected color '{}' in '{}'", elems[2], line)
                        }));
                    },
                }
            }
        }
        result.push(Game{id, max});
    }
    Ok(result)
}

fn possible_games(balls: &(usize, usize, usize), games: &Vec<Game>) -> Vec<usize> {
    let mut result = vec!();
    for game in games.iter() {
        if game.max.0 <= balls.0 && game.max.1 <= balls.1 && game.max.2 <= balls.2 {
            log_verbose!("Possible game: {:?}", game);
            result.push(game.id);
        } else {
            log_verbose!("Impossible game: {:?}", game);
        }
    }
    result
}

fn part1(games: &Vec<Game>) -> usize {
    possible_games(&(12, 13, 14), &games).iter().sum::<usize>()
}

fn part2(games: &Vec<Game>) -> usize {
    let mut power = 0;
    for game in games.iter() {
        power += game.max.0 * game.max.1 * game.max.2
    }
    power
}

fn main() -> Result<(), Box<dyn Error>> {
    Day2Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day2Opts::get_opts().filename.as_str())?;
    log_verbose!("{}", content);
    let games = read_games(content.as_str())?;
    log_verbose!("Games: {:?}", games);
    println!("Sum ID possible games: {:?}", part1(&games));
    println!("Total power of all games: {:?}", part2(&games));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let games = read_games(content.as_str()).unwrap();
        assert_eq!(8, part1(&games));
        assert_eq!(2286, part2(&games));
    }
}
