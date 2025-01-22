use std::collections::HashMap;

struct Calculator {
    memoize: HashMap<(usize, Vec<usize>), usize>,
}

impl Calculator {
    fn new() -> Self {
        Self {
            memoize: HashMap::new(),
        }
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`[k]
    /// occurrences of the digit k.
    /// This includes numbers starting with leading 0
    fn n_r(&mut self, digits: usize, repeats: &mut [usize]) -> usize {
        if digits == 0 {
            return 1;
        }
        let repeats_vec = repeats.to_vec();
        if self.memoize.contains_key(&(digits, repeats_vec.clone())) {
            return self.memoize[&(digits, repeats_vec.clone())];
        }
        let mut result = 0;
        for i in 0..=9 {
            if repeats[i] == 0 {
                continue;
            }
            repeats[i] -= 1;
            result += self.n_r(digits-1, repeats);
            repeats[i] += 1;
        }
        self.memoize.insert((digits, repeats_vec.clone()), result);
        result
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`[k]
    /// occurrences of the digit k.
    /// This excludes numbers starting with leading 0
    fn n_r_no0(&mut self, digits: usize, repeats: &mut [usize]) -> usize {
        if digits == 0 {
            return 1;
        }
        let mut result = 0;
        for i in 1..=9 {
            if repeats[i] == 0 {
                continue;
            }
            repeats[i] -= 1;
            result += self.n_r(digits-1, repeats);
            repeats[i] += 1;
        }
        result
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`
    /// occurrences of any digit.
    /// This includes numbers starting with leading 0
    #[allow(dead_code)]
    fn n_(&mut self, digits: usize, repeats: usize) -> usize {
        self.n_r(digits, vec!(repeats; 10).as_mut_slice())
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`
    /// occurrences of any digit.
    /// This includes numbers starting with leading 0
    fn n_no0(&mut self, digits: usize, repeats: usize) -> usize {
        self.n_r_no0(digits, vec!(repeats; 10).as_mut_slice())
    }
}

pub fn solve(digits: usize, repeats: usize) -> usize {
    #[cfg(debug_assertions)]
    println!("Note: build with --release for faster execution.");

    // This is just a brute force approach with memoization that builds all the possibilities.
    // We could do something much smarter.
    // One first idea of improvement is to force numbers to appear (for their first occurrence) in
    // a specific order (e.g. 1-9 then 0) and then multiply the result by 9.9! as we can do all
    // permutations we want except those that result to a leading 0.
    let mut calc = Calculator::new();
    calc.n_no0(digits, repeats)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_n_() {
        let mut calc = Calculator::new();
        for n in 0..5 {
            assert_eq!(10_usize.pow(n as u32), calc.n_(n, n));
        }
        assert_eq!(9990, calc.n_(4, 3));
    }

    #[test]
    fn test_solve() {
        assert_eq!(8991, solve(4, 3));
    }
}
