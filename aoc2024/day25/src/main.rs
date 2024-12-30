use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 25 of Advent of Code 2024.
struct Day25Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

impl Day25Opts {
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
        if Day25Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl Error for ParseError {}

#[derive(Clone,Debug,Eq,PartialEq)]
struct ProfileData(Vec<usize>);

#[derive(Clone,Debug,Eq,PartialEq)]
enum Profile {
    Key(ProfileData),
    Lock(ProfileData),
}

impl Profile {
    #[allow(dead_code)]
    fn key(data: Vec<usize>) -> Profile {
        Profile::Key(ProfileData(data))
    }

    #[allow(dead_code)]
    fn lock(data: Vec<usize>) -> Profile {
        Profile::Lock(ProfileData(data))
    }

    fn data(&self) -> &Vec<usize> {
        match self {
            Profile::Key(ProfileData(data)) => data,
            Profile::Lock(ProfileData(data)) => data,
        }
    }

    fn read(lines: &[&str]) -> Result<Profile, Box<dyn Error>> {
        if lines[0] == "....." {
            let mut data = ProfileData(vec!(0, 0, 0, 0, 0));
            for i in 0..5 {
                for j in 0..5 {
                    if lines[i+1].chars().nth(j).ok_or(format!("Not enough chars in '{}'", lines[i]))? == '#' {
                        if 5-i > data.0[j] {
                            data.0[j] = 5-i
                        }
                    }
                }
            }
            Ok(Profile::Key(data))
        } else if lines[0] == "#####" {
            let mut data = ProfileData(vec!(5, 5, 5, 5, 5));
            for i in 0..5 {
                for j in 0..5 {
                    if lines[i+1].chars().nth(j).ok_or(format!("Not enough chars in '{}'", lines[i]))? == '.' {
                        if i < data.0[j] {
                            data.0[j] = i
                        }
                    }
                }
            }
            Ok(Profile::Lock(data))
        } else {
            Err(Box::new(ParseError(format!("Bad first line '{}'", lines[0]))))
        }
    }
}

#[derive(Clone,Debug)]
struct Profiles {
    keys: Vec<Profile>,
    locks: Vec<Profile>,
}

impl Profiles {
    fn read(content: &str) -> Result<Profiles, Box<dyn Error>> {
        let lines = content.split('\n').collect::<Vec<_>>();
        let mut keys = vec!();
        let mut locks = vec!();
        for i in 0..(lines.len()+1)/8 {
            let profile = Profile::read(&lines[8*i..8*i+7])?;
            match profile {
                Profile::Key(_) => keys.push(profile),
                Profile::Lock(_) => locks.push(profile),
            }
        }
        Ok(Profiles{keys, locks})
    }

    fn fits(&self) -> usize {
        let mut fits = 0;
        for key in self.keys.iter() {
            log_verbose!("Key: {key:?}");
            for lock in self.locks.iter() {
                log_verbose!("  Lock: {lock:?}");
                let mut matches = true;
                for i in 0..5 {
                    if key.data()[i] + lock.data()[i] > 5 {
                        log_verbose!("    Doesn't match at {i}: {} + {} != 5", key.data()[i], lock.data()[i]);
                        matches = false;
                        break;
                    }
                }
                if matches {
                    fits += 1;
                }
            }
        }
        fits
    }
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day25Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day25Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

fn main() -> Result<(), Box<dyn Error>> {
    Day25Opts::set_opts(argh::from_env());

    let filename = Day25Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;
    let profiles = Profiles::read(&content).unwrap();
    log_verbose!("Profiles: {profiles:?}");
    println!("Part 1: {}", profiles.fits());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lock() {
        let content = "#####\n.####\n.####\n.####\n.#.#.\n.#...\n.....\n";
        let lines = content.split('\n').collect::<Vec<_>>();
        let profile = Profile::read(&lines[0..6]).unwrap();
        if let Profile::Lock(data) = profile {
            assert_eq!(vec!(0,5,3,4,3), data.0);
        } else {
            panic!("Profile {profile:?} is not a key");
        }
    }

    #[test]
    fn test_read_key() {
        let content = ".....\n#....\n#....\n#...#\n#.#.#\n#.###\n#####\n";
        let lines = content.split('\n').collect::<Vec<_>>();
        let profile = Profile::read(&lines[0..6]).unwrap();
        if let Profile::Key(data) = profile {
            assert_eq!(vec!(5,0,2,1,3), data.0);
        } else {
            panic!("Profile {profile:?} is not a key");
        }
    }

    #[test]
    fn test_read_profiles() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let profiles = Profiles::read(&content).unwrap();

        assert_eq!(vec!(Profile::lock(vec!(0,5,3,4,3)), Profile::lock(vec!(1,2,0,5,3))), profiles.locks);
        assert_eq!(vec!(Profile::key(vec!(5,0,2,1,3)), Profile::key(vec!(4,3,4,0,2)), Profile::key(vec!(3,0,2,0,1))), profiles.keys);
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let profiles = Profiles::read(&content).unwrap();

        assert_eq!(3, profiles.fits());
    }
}
