use crate::log_verbose;

/// Factorize `n` in 2 numbers, both between `min` and `max`. 
/// Returns `None` if no such numbers exist.
fn factors(n: u64, min: u64, max: u64) -> Option<(u64, u64)> {
    if n == 0 {
        return Some((0, 0));
    }
    let mut m = max;
    while m >= min && m > 0 {
        if (n / m) * m == n && n/m <= max && n/m >= min {
            return Some((n/m, m));
        }
        m -= 1;
    }
    return None;
}

/// Returns whether `n` is a palindrome.
fn is_palindrome(n: u64) -> bool {
    let f = format!("{n}");
    f.chars().rev().collect::<String>() == f
}

/// Returns the minimal and maximal numbers of `n` digits.
/// e.g. `min_and_max(3) == (100, 999)`
fn min_and_max(n: usize) -> (u64, u64) {
    if n == 0 {
        (0, 0)
    } else if n == 1 {
        (0, 9)
    } else {
        ("9".repeat(n-1).parse::<u64>().unwrap() + 1, "9".repeat(n).parse::<u64>().unwrap())
    }
}

/// This approach starts from the result (6 digit numbers) going from the highest candidate
/// (999*999) and going down until it finds a product which is also a palindrome.
/// This is clearly sub-optimal as this is testing many numbers that are not products. On top of
/// that, it's doing useless computation to find back the factors.
/// Takes about 30 seconds for n = 4.
pub fn slow_solve(n: usize) -> Option<u64> {
    let (min, max) = min_and_max(n);
    let mut n = max * max;
    while n >= min*min {
        if let Some((a, b)) = factors(n, min, max) {
            log_verbose!("Found {a}*{b}={n}");
            if is_palindrome(n) {
                return Some(a*b);
            }
        }
        n -= 1;
    }
    None
}

/// This approach looks at all possible products of `n` digits numbers, starting from the highest
/// and going down.
/// We stop early as:
///  - as soon as we find a palindrome for a given value of `a`, no lower value of `b` can give a
///  bigger solution.
///  - as soon as `a` reaches the best value we have for `b`, we won't do any better.
/// Takes about 15 seconds for n = 5.
pub fn better_solve(n: usize) -> Option<u64> {
    let (min, max) = min_and_max(n);
    let mut result = 0;
    let mut best_b = 0;
    for a in (min..=max).rev() {
        log_verbose!("Trying a={a}");
        // If we get to the point where a is smaller than the best b so far, we have no chance of
        // finding any new better result.
        if a <= best_b {
            break;
        }
        for b in (min..=max).rev() {
            log_verbose!("  Trying b={b}");
            let candidate = a*b;
            if is_palindrome(candidate) {
                log_verbose!("    Found palindrome {a}*{b}={candidate}");
                result = std::cmp::max(candidate, result);
                best_b = b;
                // There's no point in testing more options for b for this a as the result will
                // necessarily be lower.
                break;
            }
        }
    }
    Some(result)
}

/// This approach looks at all possible products of `n` digits numbers, starting from the highest
/// and going down using a diagonal scanning (999*999, 998*999, 997*999, 998*998, etc...).
/// We stop when there's no chance of finding a bigger product than the best palindrome we found.
/// Takes about 7 seconds for n = 7.
pub fn solve(n: usize) -> Option<u64> {
    let (min, max) = min_and_max(n);
    let mut d = 0;
    let mut result = 0;
    let mut last_d = max;
    while d < max - min {
        //log_verbose!("Looking for d={d}");
        for i in 0..d {
            let a = max - i;
            let b = max + i - d;
            let candidate = a*b;
            if is_palindrome(candidate) {
                result = std::cmp::max(candidate, result);
                // Could it be that a bigger palindrome exist for a larger d but closer to the diagonal?
                // If N is the number "on the diagonal", then for a distance `r` the product is:
                //  (N-r)(N+r) = N^2 - r^2
                // The best we could do on the kth line from it is:
                //  (N-k)^2 = N^2 - 2*k*N + k^2
                // Which will be bigger if 2*k*N - k^2 < r^2
                //  i.e. k^2 - 2*N*k + r^2 > 0
                //  i.e. k > N + sqrt(N^2 - r^2)
                let r = ((d + 1) as i64/2 - i as i64).abs() as u64;
                let n = max - (d+1)/2;
                // I think the +1 shouldn't be needed but it doesn't cost much to do one more line
                // and I'm not 100% sure ...
                last_d = std::cmp::min(last_d, d + n - (((n*n - r*r) as f64).sqrt() as u64) + 1);
                log_verbose!("  Found palindrome {a}*{b}={candidate} - n = {n} - r = {r} - will stop at d = {last_d}");
            }
        }
        d += 1;
        if d > last_d {
            break
        }
    }
    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_and_max() {
        assert_eq!((0, 0), min_and_max(0));
        assert_eq!((0, 9), min_and_max(1));
        assert_eq!((10, 99), min_and_max(2));
        assert_eq!((100, 999), min_and_max(3));
        assert_eq!((1000, 9999), min_and_max(4));
    }

    #[test]
    fn test_factors() {
        assert_eq!(Some((10, 10)), factors(10*10, 10, 20));
        assert_eq!(Some((17, 21)), factors(17*21, 10, 30));
        // 1721 is prime.
        assert_eq!(None, factors(1721, 10, 99));
        // 277 is prime.
        assert_eq!(None, factors(277*15, 10, 99));
        assert_eq!(None, factors(277*15, 100, 300));
        assert_eq!(Some((15, 277)), factors(277*15, 10, 300));
    }

    #[test]
    fn test_is_palindrome() {
        // Any single digit number is a palindrome.
        for n in 0..10 {
            assert_eq!(true, is_palindrome(n));
        }

        // Among 2 digits numbers, only multiples of 11 are palindromes
        for n in 10..100 {
            assert_eq!(n%11 == 0, is_palindrome(n));
        }

        // A 3 digits number is a palindrome if the first digit is equal to the last digit.
        for a in 1..10 {
            for b in 0..10 {
                for c in 0..10 {
                    assert_eq!(a == c, is_palindrome(a*100 + b*10 + c));
                }
            }
        }

        // A 4 digits number is a palindrome if the first digit is equal to the last digit and the
        // second is equal to the third.
        for a in 1..10 {
            for b in 0..10 {
                for c in 0..10 {
                    for d in 0..10 {
                        assert_eq!(a == d && b == c, is_palindrome(a*1000 + b*100 + c*10 + d));
                    }
                }
            }
        }

        // A few bigger examples.
        assert_eq!(false, is_palindrome(132211));
        assert_eq!(true, is_palindrome(132231));
        assert_eq!(false, is_palindrome(1234565432));
        assert_eq!(true, is_palindrome(12345654321));
    }

    /// Test based on the example from the problem statement.
    #[test]
    fn test_2() {
        assert_eq!(Some(9009), solve(2));
    }

    /// Test based on the example from the problem statement.
    #[test]
    fn test_3() {
        assert_eq!(Some(906609), solve(3));
    }

    /// Test based on the example from the problem statement.
    #[test]
    fn test_all_agree() {
        for i in 0..3 {
            assert_eq!(slow_solve(i), better_solve(i));
            assert_eq!(slow_solve(i), solve(i));
        }
    }
}
