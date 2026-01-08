use argh::FromArgs;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 10 of Advent of Code 2025.
struct Day11Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Network {
    connections: HashMap<String, Vec<String>>,
}

impl Network {
    fn parse(repr: &str) -> Result<Self, Box<dyn Error>> {
        let connections = repr.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts = line.split(':').collect::<Vec<_>>();
                if parts.len() != 2 {
                    return Err(format!("Wanted 2 parts separated by ':', got '{line}'"));
                }
                let key = parts[0].to_string();
                let values = parts[1].split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                Ok((key, values))
            })
            .collect::<Result<HashMap<_,_>, _>>()?;

        Ok(Network {
            connections,
        })
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn part1_rec(&self, current: &String, memoized: &mut HashMap<String, usize>) -> usize {
        if memoized.contains_key(current) {
            return *memoized.get(current).unwrap();
        }
        let mut result = 0;
        for next in self.connections.get(current).unwrap_or(&vec![]).iter() {
            if memoized.contains_key(current) {
                result += *memoized.get(current).unwrap();
            } else {
                result += self.part1_rec(next, memoized);
            }
        }
        memoized.insert(current.clone(), result);
        result
    }

    fn part1(&self) -> usize {
        let mut memoized = HashMap::from([("out".into(), 1)]);
        let start = "you".into();
        self.part1_rec(&start, &mut memoized)
    }

    fn paths_from_to(&self, from: &String, to: &String) -> usize {
        let mut memoized = HashMap::from([(to.clone(), 1)]);
        self.part1_rec(from, &mut memoized)
    }

    // Not very elegant, but it works ¯\_(ツ)_/¯
    fn part2(&self) -> usize {
        let svr = "svr".into();
        let fft = "fft".into();
        let dac = "dac".into();
        let out = "out".into();

        let svr_fft = self.paths_from_to(&svr, &fft);
        let svr_dac = self.paths_from_to(&svr, &dac);
        let dac_fft = self.paths_from_to(&dac, &fft);
        let fft_dac = self.paths_from_to(&fft, &dac);
        let fft_out = self.paths_from_to(&fft, &out);
        let dac_out = self.paths_from_to(&dac, &out);

        svr_fft*fft_dac*dac_out + svr_dac*dac_fft*fft_out
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day11Opts = argh::from_env();

    let network = Network::load(opts.filename.as_str())?;

    println!("Part 1: {}", network.part1());
    println!("Part 2: {}", network.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_network() {
        let want = Network{
            connections: HashMap::from([
                ("aaa".into(), vec!["you".into(), "hhh".into()]),
                ("you".into(), vec!["bbb".into(), "ccc".into()]),
                ("bbb".into(), vec!["out".into()]),
            ]),
        };

        assert_eq!(want, Network::parse("aaa: you hhh\nyou: bbb ccc\nbbb: out").unwrap());
    }

    #[test]
    fn test_part1() {
        let network = Network::load("sample.txt").unwrap();
        assert_eq!(5, network.part1());
    }

    #[test]
    fn test_part2() {
        let network = Network::load("sample2.txt").unwrap();
        assert_eq!(2, network.part2());
    }
}
