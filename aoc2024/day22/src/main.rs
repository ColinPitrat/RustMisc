use argh::FromArgs;
use std::collections::{HashMap,HashSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::{Deref,DerefMut};
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 22 of Advent of Code 2024.
struct Day22Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day22Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day22Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day22Opts {
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
        if Day22Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Prng {
    secret: usize,
    multiplicator1: usize,
    divisor: usize,
    multiplicator2: usize,
}

impl fmt::Display for Prng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PRNG(secret={}, mul1={}, div={}, mul2={}", self.secret, self.multiplicator1, self.divisor, self.multiplicator2)
    }
}

impl Prng {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let secret = content.parse::<usize>()?;
        Ok(Self{
            secret,
            multiplicator1: 64,
            divisor: 32,
            multiplicator2: 2048,
        })
    }

    fn mix_and_prune(&mut self, number: usize) {
        self.secret = (number ^ self.secret) % 16777216;
    }

    fn next(&mut self) -> usize {
        self.mix_and_prune(self.secret * self.multiplicator1);
        self.mix_and_prune(self.secret / self.divisor);
        self.mix_and_prune(self.secret * self.multiplicator2);
        self.secret
    }

    fn simulate(&mut self, iterations: usize) -> usize {
        for _ in 1..iterations {
            self.next();
        }
        self.next()
    }

    fn prices(&mut self, iterations: usize) -> Vec<(usize, i64)> {
        let mut result = vec!();
        let mut prev = self.secret % 10;
        for _ in 1..iterations {
            let new = self.next() % 10;
            result.push((new, new as i64-prev as i64));
            prev = new;
        }
        result
    }
}

#[derive(Clone,Debug)]
struct Prngs(Vec<Prng>);

impl fmt::Display for Prngs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for prng in self.0.iter() {
            writeln!(f, "{prng}")?;
        }
        Ok(())
    }
}

impl Deref for Prngs {
    type Target = Vec<Prng>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prngs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Prngs {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut prngs = vec!();
        for line in content.split('\n') {
            if line.is_empty() {
                continue;
            }
            prngs.push(Prng::read(line)?);
        }
        Ok(Self(prngs))
    }

    fn part1(&self) -> usize {
        let mut result = 0;
        for prng in self.clone().iter_mut() {
            result += prng.simulate(2000);
        }
        result
    }

    fn prices(&self, iterations: usize) -> Vec<Vec<(usize, i64)>> {
        let mut result = vec!();
        for prng in self.clone().iter_mut() {
            result.push(prng.prices(iterations));
        }
        result
    }

    fn compute_bananas(&self, seq: Vec<i64>, prices: &Vec<Vec<(usize, i64)>>) -> usize {
        let mut result = 0;
        for this_prices in prices.iter() {
            for idx in 0..this_prices.len()-seq.len() {
                let mut matches = true;
                for i in 0..seq.len() {
                    if this_prices[idx+i].1 != seq[i] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    result += this_prices[idx+seq.len()-1].0;
                    break;
                }
            }
        }
        result
    }

    // Brute force approach: try all possible combinations of 4 price changes. These go from -9 to
    // 9 so there are 19^4 = 130321 possibilities which is not huge.
    // This takes ~15 minutes without short-cutting for impossible sequences.
    // With all the short-cuts, it takes ~5 minutes.
    #[allow(dead_code)]
    fn part2_slow(&self) -> (usize, Vec<i64>) {
        let mut max_bananas = 0;
        let mut best_seq = Vec::new();
        let prices = self.prices(2000);
        let mut combis = 0;
        for i in -9i64..=9 {
            log_verbose!("i={i}");
            for j in -9i64..=9 {
                if (i+j).abs() >= 10 {
                    continue
                }
                //log_verbose!("i={i}, j={j}");
                for k in -9i64..=9 {
                    if (i+j+k).abs() >= 10 || (j+k).abs() >= 10 {
                        continue
                    }
                    for l in -9i64..=9 {
                        if (i+j+k+l).abs() >= 10 || (j+k+l).abs() >= 10 || (k+l).abs() >= 10 {
                            continue
                        }
                        combis += 1;
                        let total_price = self.compute_bananas(vec!(i, j, k, l), &prices);
                        if total_price > max_bananas {
                            log_verbose!("New best at ({i}, {j}, {k}, {l}): {total_price}");
                            best_seq = vec!(i, j, k, l);
                            max_bananas = total_price;
                        }
                    }
                }
            }
        }
        log_verbose!("Number of combinations: {combis}");
        (max_bananas, best_seq)
    }

    // Smarter approach: count the price for all the combinations that do exist in a single pass.
    // Then go over all of them to find the maximum one.
    fn part2(&self) -> (usize, Vec<i64>) {
        let prices = self.prices(2000);
        let mut counts = HashMap::new();
        for this_prices in prices {
            let mut seen = HashSet::new();
            let (mut a, mut b, mut c, mut d) = (0, this_prices[0].1, this_prices[1].1, this_prices[2].1);
            for i in 3..this_prices.len() {
                (a, b, c, d) = (b, c, d, this_prices[i].1);
                if !seen.contains(&(a,b,c,d)) {
                    *counts.entry((a,b,c,d)).or_insert(0_usize) += this_prices[i].0;
                    seen.insert((a,b,c,d));
                }
            }
        }
        log_verbose!("Number of combinations: {}", counts.len());
        let mut max_bananas = 0;
        let mut best_seq = (0, 0, 0, 0);
        for (seq, count) in counts.iter() {
            if *count > max_bananas {
                max_bananas = *count;
                best_seq = *seq;
            }
        }
        (max_bananas, vec!(best_seq.0, best_seq.1, best_seq.2, best_seq.3))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day22Opts::set_opts(argh::from_env());

    #[cfg(debug_assertions)]
    println!("Note: build with --release for a fast execution.");

    let filename = Day22Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;

    let prngs = Prngs::read(content.as_str())?;

    println!("Part 1: {}", prngs.part1());
    println!("Part 2: {}", prngs.part2().0);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prng() {
        let mut prng = Prng::read("123").unwrap();

        assert_eq!(15887950, prng.next());
        assert_eq!(16495136, prng.next());
        assert_eq!(527345, prng.next());
        assert_eq!(704524, prng.next());
        assert_eq!(1553684, prng.next());
        assert_eq!(12683156, prng.next());
        assert_eq!(11100544, prng.next());
        assert_eq!(12249484, prng.next());
        assert_eq!(7753432, prng.next());
        assert_eq!(5908254, prng.next());
    }

    #[test]
    fn test_examples() {
        let mut prng = Prng::read("1").unwrap();
        assert_eq!(8685429, prng.simulate(2000));

        let mut prng = Prng::read("10").unwrap();
        assert_eq!(4700978, prng.simulate(2000));

        let mut prng = Prng::read("100").unwrap();
        assert_eq!(15273692, prng.simulate(2000));

        let mut prng = Prng::read("2024").unwrap();
        assert_eq!(8667524, prng.simulate(2000));
    }
    
    #[test]
    fn test_prices() {
        let prngs = Prngs::read("123").unwrap();
        assert_eq!(vec!(vec!((0, -3), (6, 6), (5, -1), (4, -1), (4, 0), (6, 2), (4, -2), (4, 0), (2, -2))), prngs.prices(10));
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let prngs = Prngs::read(content.as_str()).unwrap();

        assert_eq!(37327623, prngs.part1());
        assert_eq!(24, prngs.part2().0);
    }

    // Not having this test in debug mode is not great, but it takes about 20 seconds to run!
    #[test]
    fn test_part2() {
        let prngs = Prngs::read("1\n2\n3\n2024").unwrap();
        let (best_price, best_seq) = prngs.part2();

        assert_eq!(23, best_price);
        assert_eq!(vec!(-2, 1, -1, 3), best_seq);
    }
}
