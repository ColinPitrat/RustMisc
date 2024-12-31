use crate::log_verbose;

fn factors(n: u64, min: u64, max: u64) -> Option<(u64, u64)> {
    let mut m = max;
    while m >= min {
        if (n / m) * m == n && n/m <= max && n/m >= min {
            return Some((m, n/m));
        }
        m -= 1;
    }
    return None;
}

fn is_palindrome(n: u64) -> bool {
    let f = format!("{n}");
    f.chars().rev().collect::<String>() == f
}

pub fn solve(n: usize) -> Option<u64> {
    let max = "9".repeat(n).parse::<u64>().unwrap();
    let min = "9".repeat(n-1).parse::<u64>().unwrap() + 1;
    let mut n = max * max;
    while n > min*min {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
