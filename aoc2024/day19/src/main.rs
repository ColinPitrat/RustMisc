use argh::FromArgs;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 19 of Advent of Code 2024.
struct Day19Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day19Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day19Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day19Opts {
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
        if Day19Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Puzzle {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.patterns.join(", "))?;
        writeln!(f, "")?;
        for p in self.designs.iter() {
            writeln!(f, "{p}")?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut lines = content.split('\n');

        let patterns = lines.next().ok_or("No first line in input")?.split(", ").map(|p| p.to_string()).collect::<Vec<_>>();
        lines.next().ok_or("No second line in input")?;

        let mut designs = vec!();
        for line in lines {
            if line.is_empty() {
                break;
            }
            designs.push(line.to_string());
        }

        Ok(Self{patterns, designs})
    }

    fn can_do(&self, design: &str, doable: &mut HashMap<String, bool>) -> bool {
        if doable.contains_key(design) {
            return doable[design];
        }

        for pattern in self.patterns.iter() {
            if design.starts_with(pattern.as_str()) {
                let design = design.strip_prefix(pattern.as_str()).unwrap();
                let ok = self.can_do(design, doable);
                doable.insert(design.to_string(), ok);
                if ok {
                    return true;
                }
            }
        }

        false
    }

    fn doable(&self) -> usize {
        let mut doable = HashMap::from([("".to_string(), true)]);
        if Day19Opts::get_opts().verbose {
            for design in self.designs.iter().filter(|d| self.can_do(d, &mut doable)) {
                log_verbose!("  '{design}' is doable");
            }
        }
        self.designs.iter().filter(|d| self.can_do(d, &mut doable)).count()
    }

    fn ways_to_do(&self, design: &str, doable: &mut HashMap<String, usize>) -> usize {
        if doable.contains_key(design) {
            return doable[design];
        }

        let mut ways = 0;
        for pattern in self.patterns.iter() {
            if design.starts_with(pattern.as_str()) {
                let design = design.strip_prefix(pattern.as_str()).unwrap();
                ways += self.ways_to_do(design, doable);
            }
        }
        doable.insert(design.to_string(), ways);

        ways
    }

    fn possibilities(&self) -> usize {
        let mut doable = HashMap::from([("".to_string(), 1)]);
        self.designs.iter().map(|d| self.ways_to_do(d, &mut doable)).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day19Opts::set_opts(argh::from_env());

    let filename = Day19Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;

    let puzzle = Puzzle::read(content.as_str())?;
    log_verbose!("Input:\n{puzzle}");

    println!("Part 1: {}", puzzle.doable());
    println!("Part 2: {}", puzzle.possibilities());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let puzzle = Puzzle::read(content.as_str()).unwrap();

        assert_eq!(6, puzzle.doable());
        assert_eq!(16, puzzle.possibilities());
    }
}
