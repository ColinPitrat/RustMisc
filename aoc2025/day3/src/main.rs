use argh::FromArgs;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 3 of Advent of Code 2025.
struct Day3Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Bank {
    joltages: Vec<u8>
}

impl Bank {
    fn parse(repr: &str) -> Result<Bank, Box<dyn Error>> {
        let joltages = repr.chars()
            .map(|c| -> Result<u8, Box<dyn Error>> {
                        Ok(c.to_digit(10)
                            .ok_or(format!("Not a digit: '{c}'"))?
                            .try_into()?)
                    }
                )
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Bank { joltages })
    }

    fn highest_2d_number(&self) -> u8 {
        let (first_index, first) = self.joltages.iter()
            .take(self.joltages.len()-1) // Exclude the last element.
            .enumerate()
            // We cannot use max_by because it returns the second argument if both are equal.
            .fold((0, 0), |(k_max, v_max), (kb, &b)| if v_max < b { (kb, b) } else { (k_max, v_max) });
        let second = self.joltages[first_index+1..].iter().max().unwrap();
        10*first + second
    }

    fn highest_number(&self, n: usize) -> u64 {
        let mut digits = vec!();
        let mut start_index = 0;
        for i in 0..n {
            let (next_index, next_digit) = self.joltages[..self.joltages.len()-n+i+1].iter()
                .enumerate()
                .skip(start_index)
                // We cannot use max_by because it returns the second argument if both are equal.
                .fold((0, 0), |(k_max, v_max), (kb, &b)| if v_max < b { (kb, b) } else { (k_max, v_max) });
            digits.push(next_digit);
            start_index = next_index+1;
        }
        let mut result = 0;
        for n in digits {
            result *= 10;
            result += n as u64;
        }
        result
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Array {
    banks: Vec<Bank>
}

impl Array {
    fn parse(repr: &str) -> Result<Array, Box<dyn Error>> {
        Ok(Array {
            banks: repr.split('\n')
                    .filter(|line| !line.is_empty())
                    .map(|line| Bank::parse(line))
                    .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn part1_sum(&self) -> u64 {
        self.banks.iter().map(|bank| bank.highest_2d_number() as u64).sum()
    }

    fn part2_sum(&self) -> u64 {
        self.banks.iter().map(|bank| bank.highest_number(12) as u64).sum()
    }
}

fn read_array(filename: &str) -> Result<Array, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    Array::parse(&content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day3Opts = argh::from_env();
    let array = read_array(opts.filename.as_str())?;

    println!("Part 1: {}", array.part1_sum());
    println!("Part 2: {}", array.part2_sum());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bank() {
        let bank = Bank::parse("987654321111111").unwrap();
        let want = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];

        assert_eq!(bank.joltages, want);
    }

    #[test]
    fn test_highest_2d_number() {
        let cases = vec![
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
            ("918181911112911", 99),
        ];

        for case in cases {
            let bank = Bank::parse(case.0).unwrap();

            let got = bank.highest_2d_number();

            assert_eq!(got, case.1, "Wanted {} for {}, got {}", case.1, case.0, got);
        }
    }

    #[test]
    fn test_highest_number() {
        let cases = vec![
            // Should still work for n = 2
            (2, "987654321111111", 98),
            (2, "811111111111119", 89),
            (2, "234234234234278", 78),
            (2, "818181911112111", 92),
            (2, "918181911112911", 99),

            // Examples with n = 12
            (12, "987654321111111", 987654321111),
            (12, "811111111111119", 811111111119),
            (12, "234234234234278", 434234234278),
            (12, "818181911112111", 888911112111),
            (12, "918181911112911", 988911112911),
        ];

        for case in cases {
            let bank = Bank::parse(case.1).unwrap();

            let got = bank.highest_number(case.0);

            assert_eq!(got, case.2, "With n={}, wanted {} for {}, got {}", case.0, case.2, case.1, got);
        }
    }

    #[test]
    fn test_parse_array() {
        let array = Array::parse("987654321111111\n811111111111119").unwrap();
        let want = vec![
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
            vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        ];

        assert_eq!(array.banks[0].joltages, want[0]);
        assert_eq!(array.banks[1].joltages, want[1]);
    }

    #[test]
    fn test_parse_sample() {
        let array = read_array("sample.txt").unwrap();
        let want = Array {
            banks: vec![
                Bank { joltages: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1] },
                Bank { joltages: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9] },
                Bank { joltages: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8] },
                Bank { joltages: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1] },
            ],
        };

        assert_eq!(array, want);
    }

    #[test]
    fn test_part1() {
        let array = read_array("sample.txt").unwrap();

        assert_eq!(array.part1_sum(), 357);
    }

    #[test]
    fn test_part2() {
        let array = read_array("sample.txt").unwrap();

        assert_eq!(array.part2_sum(), 3121910778619);
    }
}
