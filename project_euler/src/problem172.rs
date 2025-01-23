use crate::log_verbose;
use std::collections::HashMap;

struct Calculator {
    memoize: HashMap<(usize, Vec<usize>), usize>,
}

fn permutations(n: usize, p: usize) -> usize {
    (n-1)*((n-p+1)..n).product::<usize>()
}

impl Calculator {
    fn new() -> Self {
        Self {
            memoize: HashMap::new(),
        }
    }

    fn n_r(&mut self, digits: usize, repeats: &mut [usize], max_repeats: usize, so_far: usize) -> usize {
        if digits == 0 {
            // Number of distinct digits.
            let p = repeats.iter()
                .filter(|&r| *r != max_repeats)
                .count();
            // Number of ways to permutate the selected digits, not allowing the first one to be a
            // 0 (so it has repeats.len()-1 possibilities).
            let n = repeats.len();
            let perms = permutations(n, p);
            log_verbose!("Signature: {repeats:?} - {so_far} - {perms}");
            return permutations(n, p);
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
            result += self.n_r(digits-1, repeats, max_repeats, so_far*10+(9-i));
            repeats[i] += 1;
            if repeats[i] == max_repeats {
                break;
            }
        }
        self.memoize.insert((digits, repeats_vec.clone()), result);
        result
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`[k]
    /// occurrences of the digit k.
    /// This excludes numbers starting with leading 0
    // The overall idea is to produce all the possible combinations of digit to a permutation.
    // So 9987, 1123, 5572, ... are all equivalent forms aabc for a choice of aabc where each
    // letter is a different digit and a != 0.
    // Any number will match a single of these combinations.
    // The number of numbers matching a given combination is the number of permutations possibles.
    // With leading 0s, this would simply be 10*9*...*(10-k) where k is the number of distinct
    // digits. As we forbid leading 0s, this is instead 9*9*...*(10-k).
    fn n_r_no0(&mut self, digits: usize, repeats: &mut [usize], max_repeats: usize, so_far: usize) -> usize {
        if digits == 0 {
            return 1;
        }
        let mut result = 0;
        repeats[0] -= 1;
        result += self.n_r(digits-1, repeats, max_repeats, so_far*10+9);
        repeats[0] += 1;
        result
    }

    /// Returns the number of numbers with `digits` digits with at most `repeats`
    /// occurrences of any digit.
    /// This includes numbers starting with leading 0
    fn n_no0(&mut self, digits: usize, repeats: usize) -> usize {
        self.n_r_no0(digits, vec!(repeats; 10).as_mut_slice(), repeats, 0)
    }
}

pub fn solve(digits: usize, repeats: usize) -> usize {
    #[cfg(debug_assertions)]
    println!("Note: build with --release for faster execution.");

    let mut calc = Calculator::new();
    calc.n_no0(digits, repeats)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(8991, solve(4, 3));
        assert_eq!(227485267000992000, solve(18, 3));
    }
}
