use argh::FromArgs;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 2 of Advent of Code 2025.
struct Day2Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

// The approach taken to solve this problem is to iterate over the ranges and verify whether each
// number is valid or not.
// Thinking more about it, there's likely a smarter approach.
// Once an invalid number is found, we know that another invalid number cannot be found for the
// same pattern length until we add at least 1 to each pattern. So if there are 3 patterns of
// lenght 2, if N is invalid, the next candidate to be invalid is N+10101.
// Even better, for each pattern length, we can generate the candidates and verify if they fall in
// the range. For example, for the range 212120-333335
//  - pattern length = 1, we can only have 222222 and 333333
//  - pattern length = 2, we can only have 212121, 222222, 232323, etc...
//  - pattern length = 3, we can only have 212212, 213213, etc...

#[derive(Debug, Eq, PartialEq)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn parse(repr: &str) -> Result<Range, Box<dyn Error>> {
        let parts = repr.trim().split("-").collect::<Vec<_>>();
        if parts.len() != 2 {
            Err(format!("Invalid range: '{}'", repr).into())
        } else {
            Ok(Range{
                min: parts[0].parse()?,
                max: parts[1].parse()?,
            })
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RangesList {
    ranges: Vec<Range>,
}

impl RangesList {
    fn parse(repr: &str) -> Result<RangesList, Box<dyn Error>> {
        Ok(RangesList{
            ranges: repr.split(",")
                        .map(|r| Range::parse(r))
                        .collect::<Result<Vec<_>, _>>()?,
                })
    }
}

fn read_ranges(filename: &str) -> Result<RangesList, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    RangesList::parse(&content)
}

fn is_invalid_part1(n: usize) -> bool {
    let s = n.to_string();
    s.len() % 2 == 0 && s[..s.len()/2] == s[s.len()/2..]
}

// Brute force approach.
#[allow(dead_code)]
fn is_invalid_part2_stupid(n: usize) -> bool {
    let s = n.to_string();
    'outer: for patterns in 2..=s.len() {
        if s.len() % patterns != 0 {
            continue;
        }
        let pattern_length = s.len()/patterns;
        for i in 1..patterns {
            if s[..pattern_length] != s[i*pattern_length..(i+1)*pattern_length] {
                continue 'outer
            }
        }
        return true;
    }
    false
}

// We can be smarter: if s.len() % 4 == 0 and s.len() > 0 there's no need to test
// pattern_length = 2 because pattern_length = 4 will cover it anyway.
// (e.g. in 21212121 we have 2121 == 2121 on top of 21 == 21).
// In general, if s.len() % n == 0 and s.len() > n we can avoid testing all the
// factors of n by testing n.
// This means that in general, we can only test the 2 largest factors of s.len(). 
// For example for s.len()=40 we can test just 20 (2 patterns) and 8 (5 patterns).
//
// The result is not faster than the stupid approach.
#[allow(dead_code)]
fn is_invalid_part2_slow(n: usize) -> bool {
    let s = n.to_string();
    let mut factors = vec!();
    for f in (2..s.len()).rev() {
        if s.len() % f == 0 {
            factors.push(f);
            if factors.len() == 2 {
                break;
            }
        }
    }
    if is_prime(s.len()) {
        factors.push(s.len());
    }
    'outer: for patterns in factors {
        if s.len() % patterns != 0 {
            continue;
        }
        let pattern_length = s.len()/patterns;
        for i in 1..patterns {
            if s[..pattern_length] != s[i*pattern_length..(i+1)*pattern_length] {
                continue 'outer
            }
        }
        return true;
    }
    false
}

fn is_prime(n: usize) -> bool {
    if n > 20 {
        panic!("Fast is_prime doesn't support numbers bigger than 100, asked for {n}");
    }
    match n {
        2|3|5|7|11|13|17|19 => true,
        _ => false
    }
}

#[allow(dead_code)]
fn is_prime_slow(n: usize) -> bool {
    if n <= 3 {
        return n > 1;
    }
    if n % 6 != 1 && n % 6 != 5 {
        return false;
    }
    for i in (5..(n as f64).sqrt() as usize+1).step_by(6) {
        if n%i == 0 || n%(i+2) == 0 {
            return false;
        }
    }

    return true;
}

// The implementation above is slow. We could precompute the factors.
// Because numbers always have less than 10 digits, it's easy to hardcode the list.
const FACTORS : [[usize; 2]; 13] = [
    [0, 0],
    [1, 0],
    [2, 0],
    [3, 0],
    [2, 0],
    [5, 0],
    [2, 3],
    [7, 0],
    [2, 0],
    [3, 0],
    [5, 2],
    [11, 0],
    [2, 3],
];

// This implementation is indeed faster than the stupid approach but not by a
// huge margin.
fn is_invalid_part2_fast(n: usize) -> bool {
    let s = n.to_string();
    let factors = FACTORS[s.len()];
    'outer: for patterns in factors {
        if patterns < 1 {
            continue
        }
        if s.len() % patterns != 0 {
            continue;
        }
        let pattern_length = s.len()/patterns;
        for i in 1..patterns {
            if s[..pattern_length] != s[i*pattern_length..(i+1)*pattern_length] {
                continue 'outer
            }
        }
        return true;
    }
    false
}

#[inline(always)]
fn is_invalid_part2(n: usize) -> bool {
    //is_invalid_part2_stupid(n)
    //is_invalid_part2_slow(n)
    is_invalid_part2_fast(n)
}

fn solve(ranges: &RangesList, is_invalid: fn(usize) -> bool) -> usize {
    ranges.ranges.iter().map(|range|
        (range.min..=range.max)
                .filter(|n| is_invalid(*n))
                .sum::<usize>()
    ).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day2Opts = argh::from_env();

    let ranges = read_ranges(opts.filename.as_str())?;

    println!("Part 1: {}", solve(&ranges, is_invalid_part1));
    println!("Part 2: {}", solve(&ranges, is_invalid_part2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_range_parse() {
        assert_eq!(Range{min: 11, max: 22}, Range::parse("11-22").unwrap());
        assert_eq!(Range{min: 95, max: 115}, Range::parse("95-115").unwrap());
    }

    #[test]
    fn test_range_list_parse() {
        let want = RangesList{
            ranges: vec![
                        Range{min: 11, max: 22},
                        Range{min: 1188511880, max: 1188511890},
                        Range{min: 2121212118, max: 2121212124},
            ],
        };

        assert_eq!(want, RangesList::parse("11-22,1188511880-1188511890,2121212118-2121212124").unwrap());
    }

    #[test]
    fn test_is_prime() {
        /*
        let primes = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
        for i in 0..100 {
            assert_eq!(primes.contains(&i), is_prime(i));
        }
        */
        let primes = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19]);
        for i in 0..20 {
            assert_eq!(primes.contains(&i), is_prime(i));
        }
    }

    #[test]
    fn test_is_invalid_part1() {
        assert_eq!(true, is_invalid_part1(55));
        assert_eq!(true, is_invalid_part1(6464));
        assert_eq!(true, is_invalid_part1(123123));

        assert_eq!(true, is_invalid_part1(11));
        assert_eq!(true, is_invalid_part1(22));
        assert_eq!(true, is_invalid_part1(99));
        assert_eq!(true, is_invalid_part1(1010));
        assert_eq!(true, is_invalid_part1(1188511885));
        assert_eq!(true, is_invalid_part1(222222));
        assert_eq!(true, is_invalid_part1(446446));
        assert_eq!(true, is_invalid_part1(38593859));

        assert_eq!(false, is_invalid_part1(95));
        assert_eq!(false, is_invalid_part1(115));
        assert_eq!(false, is_invalid_part1(12345));
        assert_eq!(false, is_invalid_part1(123123123));
    }

    #[test]
    fn test_is_invalid_part2() {
        // All invalid for part 1 are also invalid for part 2
        assert_eq!(true, is_invalid_part2(55));
        assert_eq!(true, is_invalid_part2(6464));
        assert_eq!(true, is_invalid_part2(123123));

        assert_eq!(true, is_invalid_part2(11));
        assert_eq!(true, is_invalid_part2(22));
        assert_eq!(true, is_invalid_part2(99));
        assert_eq!(true, is_invalid_part2(1010));
        assert_eq!(true, is_invalid_part2(1188511885));
        assert_eq!(true, is_invalid_part2(222222));
        assert_eq!(true, is_invalid_part2(446446));
        assert_eq!(true, is_invalid_part2(38593859));

        // And more:
        assert_eq!(true, is_invalid_part2(123123123));
        assert_eq!(true, is_invalid_part2(565656));
        assert_eq!(true, is_invalid_part2(824824824));
        assert_eq!(true, is_invalid_part2(828282));
    }

    #[test]
    fn test_part1() {
        let ranges = read_ranges("sample.txt").unwrap();

        assert_eq!(1227775554, solve(&ranges, is_invalid_part1));
    }

    #[test]
    fn test_part2() {
        let ranges = read_ranges("sample.txt").unwrap();

        assert_eq!(4174379265, solve(&ranges, is_invalid_part2));
    }
}
