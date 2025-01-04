use crate::log_verbose;
use crate::primes;
use std::collections::HashSet;

fn reverse(n: u64) -> u64 {
    format!("{n}").chars().rev().collect::<String>().parse::<u64>().unwrap()
}

fn is_palindrome(n: u64) -> bool {
    reverse(n) == n
}

/// Find the sum of the first `n` reversible prime squares.
/// A reversible prime square:
///  - is not a palindrome
///  - is the square of a prime
///  - its reverse (i.e. reading it backward) is the square of a prime
/// e.g. 169 and 961 are reversible prime squares
pub fn solve(n: u64) -> u64 {
    let mut prime_squares = HashSet::new();
    let mut reversible_prime_squares = vec!();
    let mut count = 0;
    let mut stop_at = u64::MAX;
    log_verbose!("Looking for the first {n} prime squares");
    let limit = if n < 5 {
        // Enough to go up to 4.
        100000
    } else {
        // Enough to go up to 50.
        32000000
    };
    for p in primes::sieve(limit) {
        let ps = p*p;
        if ps > stop_at {
            break;
        }
        if !is_palindrome(ps) {
            prime_squares.insert(ps);
        }
        let rev = reverse(ps);
        if prime_squares.contains(&rev) {
            reversible_prime_squares.push(ps);
            reversible_prime_squares.push(rev);
            count += 2;
            log_verbose!("{count}/{n} Found {ps} and {rev}");
            if count >= n {
                stop_at = 10_u64.pow((rev as f64).log10().ceil() as u32)
            }
        }
    }
    reversible_prime_squares.sort();
    reversible_prime_squares.into_iter().take(n as usize).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(12, reverse(21));
        assert_eq!(345, reverse(543));
        assert_eq!(6789, reverse(9876));
    }

    #[test]
    fn test_is_palindrome() {
        for a in 0..10 {
            assert_eq!(true, is_palindrome(a), "is_palindrome({a})");
        }
        for a in 1..10 {
            for b in 0..10 {
                assert_eq!(a == b, is_palindrome(10*a + b), "is_palindrome({a}{b})");
            }
        }
        for a in 1..10 {
            for b in 0..10 {
                for c in 0..10 {
                    assert_eq!(a == c, is_palindrome(100*a + 10*b + c), "is_palindrome({a}{b}{c})");
                }
            }
        }
        for a in 1..10 {
            for b in 0..10 {
                for c in 0..10 {
                    for d in 0..10 {
                        assert_eq!(a == d && b == c, is_palindrome(1000*a + 100*b + 10*c + d), "is_palindrome({a}{b}{c}{d})");
                    }
                }
            }
        }
        assert_eq!(false, is_palindrome(1234567890987654320));
        assert_eq!(true, is_palindrome(1234567890987654321));
        assert_eq!(false, is_palindrome(1234567890987654322));
    }

    #[test]
    fn test_solve() {
        assert_eq!(1130, solve(2));
    }
}
