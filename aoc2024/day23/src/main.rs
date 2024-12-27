use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 23 of Advent of Code 2024.
struct Day23Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

impl Day23Opts {
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
        if Day23Opts::get_opts().verbose {
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

#[derive(Clone,Debug)]
struct GraphBuilder {
    edges: HashSet<(String, String)>,
}

impl GraphBuilder {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut edges = HashSet::new();
        for line in content.split('\n') {
            if line.is_empty() {
                continue;
            }
            let elems = line.split('-').collect::<Vec<_>>();
            if elems.len() != 2 {
                return Err(Box::new(ParseError(format!("Unexpected number of elements, got {:?}, wanted 2 elements", elems))))
            }
            let (v1, v2) = (elems[0], elems[1]);
            edges.insert((v1.to_string(), v2.to_string()));
        }
        Ok(Self{edges})
    }

    /// Returns the list of components of the graph.
    /// This was a misunderstanding of the input, the graph has a single component.
    /// We're looking for fully connected components.
    #[allow(dead_code)]
    fn components(&self) -> Vec<HashSet<String>> {
        let mut sets: Vec<HashSet<String>> = vec!();
        for (v1, v2) in self.edges.iter() {
            log_verbose!("Handling {v1}-{v2}");
            let mut add_to = None;
            let mut extend_with = None;
            for (i, s) in sets.iter().enumerate() {
                if s.contains(v1) || s.contains(v2) {
                    add_to = Some(i);
                    for (j, s2) in sets.iter().enumerate() {
                        if i == j {
                            continue;
                        }
                        if s2.contains(v1) || s2.contains(v2) {
                            extend_with = Some(j);
                            break;
                        }
                    }
                    break;
                }
            }
            if let Some(add_to) = add_to {
                log_verbose!("  Found one in {add_to} ({:?}) adding {v1} and {v2}", sets[add_to]);
                sets[add_to].insert(v1.clone());
                sets[add_to].insert(v2.clone());
                if let Some(extend_with) = extend_with {
                    log_verbose!("  Also found one in {extend_with} ({:?}) merging it", sets[extend_with]);
                    let to_merge = sets.remove(extend_with);
                    sets[add_to].extend(to_merge.into_iter());
                }
            } else {
                log_verbose!("  Not found, inserting {v1} and {v2} in a new set");
                sets.push(HashSet::from([v1.clone(), v2.clone()]));
            }
            log_verbose!("{sets:?}");
        }
        sets
    }

    fn connected(&self, v1: &String, v2: &String) -> bool {
        for (v3, v4) in self.edges.iter() {
            if (v1 == v3 && v2 == v4) || (v1 == v4 && v2 == v3) {
                return true;
            }
        }
        false
    }

    fn fully_connected_3components(&self) -> HashSet<FC3Component> {
        let mut result = HashSet::new();
        for (v1, v2) in self.edges.iter() {
            for (v3, v4) in self.edges.iter() {
                if v1 == v3 {
                    if self.connected(v2, v4) {
                        result.insert(FC3Component::new(v1, v2, v4));
                    }
                }
                if v1 == v4 {
                    if self.connected(v2, v3) {
                        result.insert(FC3Component::new(v1, v2, v3));
                    }
                }
                if v2 == v3 {
                    if self.connected(v1, v4) {
                        result.insert(FC3Component::new(v1, v2, v4));
                    }
                }
                if v2 == v4 {
                    if self.connected(v1, v3) {
                        result.insert(FC3Component::new(v1, v2, v3));
                    }
                }
            }
        }
        result
    }

    fn fully_connected_components(&self) -> HashSet<String> {
        let mut sets: Vec<HashSet<String>> = vec!();
        for (v1, v2) in self.edges.iter() {
            log_verbose!("Handling {v1}-{v2}");
            let mut add_to = None;
            for (i, s) in sets.iter().enumerate() {
                let mut connected = true;
                for v in s.iter() {
                    if v == v1 || v == v2 {
                        continue;
                    }
                    if !self.edges.contains(&(v.clone(), v1.clone())) && !self.edges.contains(&(v1.clone(), v.clone())) {
                        log_verbose!("  {v} not connected to {v1} in {:?}", s);
                        connected = false;
                        break;
                    }
                    if !self.edges.contains(&(v.clone(), v2.clone())) && !self.edges.contains(&(v2.clone(), v.clone())) {
                        log_verbose!("  {v} not connected to {v2} in {:?}", s);
                        connected = false;
                        break;
                    }
                }
                if connected {
                    add_to = Some(i);
                    break;
                }
            }

            if let Some(add_to) = add_to {
                log_verbose!("  Found fully connected in {add_to} ({:?}) adding {v1} and {v2}", sets[add_to]);
                sets[add_to].insert(v1.clone());
                sets[add_to].insert(v2.clone());
            } else {
                log_verbose!("  Not found, inserting {v1} and {v2} in a new set");
                sets.push(HashSet::from([v1.clone(), v2.clone()]));
            }
            log_verbose!("{sets:?}");
        }
        sets.into_iter().map(|x| {
            let mut v = x.into_iter().collect::<Vec<_>>();
            v.sort();
            v.join(",")
        }).collect()
    }

    fn longest_fully_connected_component(&self) -> Result<String, Box<dyn Error>> {
        let components = self.fully_connected_components();
        log_verbose!("{:?}", components);
        // TODO: This shouldn't be a ParseError but a distinct error type.
        components.into_iter()
            .reduce(|longest, e| if e.len() > longest.len() { e } else { longest })
            .ok_or(Box::new(ParseError("No fully connected components found (empty graph?)".to_string())))
    }

    fn chief_candidates(&self) -> HashSet<FC3Component> {
        self.fully_connected_3components().into_iter().filter(|s| s.maybe_chief()).collect()
    }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct FC3Component(String, String, String);

impl FC3Component {
    fn sort<'a>(v1: &'a str, v2: &'a str, v3: &'a str) -> (&'a str, &'a str, &'a str) {
        let (v1, v2) = if v1 <= v2 {
            (v1, v2)
        } else {
            (v2, v1)
        };
        if v3 < v1 {
            (v3, v1, v2)
        } else if v3 < v2 {
            (v1, v3, v2)
        } else {
            (v1, v2, v3)
        }
    }

    fn new(v1: &str, v2: &str, v3: &str) -> Self {
        let (v1, v2, v3) = Self::sort(v1, v2, v3);
        Self(v1.to_string(), v2.to_string(), v3.to_string())
    }

    fn maybe_chief(&self) -> bool {
        self.0.starts_with("t") || self.1.starts_with("t") || self.2.starts_with("t")
    }
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day23Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day23Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

fn main() -> Result<(), Box<dyn Error>> {
    Day23Opts::set_opts(argh::from_env());

    let filename = Day23Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;
    let builder = GraphBuilder::read(content.as_str())?;
    log_verbose!("Builder: {builder:?}");

    let candidates = builder.chief_candidates();
    log_verbose!("{:?}", candidates);
    println!("Part 1: {}", candidates.len());

    let longest = builder.longest_fully_connected_component()?;
    println!("Part 2: {longest}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_part1() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let builder = GraphBuilder::read(content.as_str()).unwrap();
        let candidates = builder.chief_candidates();

        assert_eq!(7, candidates.len());
        assert!(candidates.contains(&FC3Component::new("co", "de", "ta")));
        assert!(candidates.contains(&FC3Component::new("co", "ka", "ta")));
        assert!(candidates.contains(&FC3Component::new("de", "ka", "ta")));
        assert!(candidates.contains(&FC3Component::new("qp", "td", "wh")));
        assert!(candidates.contains(&FC3Component::new("tb", "vc", "wq")));
        assert!(candidates.contains(&FC3Component::new("tc", "td", "wh")));
        assert!(candidates.contains(&FC3Component::new("td", "wh", "yn")));
    }

    #[test]
    fn test_sample_part2() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let builder = GraphBuilder::read(content.as_str()).unwrap();

        assert_eq!("co,de,ka,ta", builder.longest_fully_connected_component().unwrap());
    }
}
