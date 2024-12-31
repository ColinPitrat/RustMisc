use crate::log_verbose;

/// A specialized algorithm for the issue at end.
/// We divide the number by the smallest divisors first to ensure that we don't try a composite
/// number.
pub fn solve(mut n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut result = 1;
    log_verbose!("Factors of {n}:");
    while n % 2 == 0 {
        log_verbose!(" - 2");
        n = n / 2;
        result = 2;
    }
    let mut i = 1;
    while 2*i < n {
        let p = 2*i+1;
        while n % p == 0 {
            log_verbose!(" - {p}");
            n = n / p;
            result = p;
        }
        i += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test based on the example from the problem statement.
    #[test]
    fn test_13195() {
        assert_eq!(29, solve(13195));
    }

    /// The solution to the problem.
    #[test]
    fn test_600851475143() {
        assert_eq!(6857, solve(600851475143));
    }

    /// Some additional corner cases that are worth checking.
    #[test]
    fn test_corner_cases() {
        assert_eq!(0, solve(0));
        assert_eq!(1, solve(1));
        assert_eq!(2, solve(2));
        assert_eq!(3, solve(3));
        assert_eq!(2, solve(4));
        assert_eq!(5, solve(5));
        assert_eq!(3, solve(6));
        assert_eq!(7, solve(7));
        assert_eq!(2, solve(8));
        assert_eq!(3, solve(9));
        assert_eq!(5, solve(10));
        assert_eq!(11, solve(11));
        assert_eq!(3, solve(12));
        assert_eq!(13, solve(13));
        assert_eq!(7, solve(14));
        assert_eq!(5, solve(15));
    }
}
