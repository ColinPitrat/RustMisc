/// "Naive" solve: we go over all integers up to the max and we count the ones that are multiples
/// of 3 or 5.
pub fn naive_solve(below: u64) -> u64 {
    (0..below).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

/// Slightly better solve: we can directly list the multiples of 3 and 5. We must also list the
/// multiples of 15 to substract them as they are counted twice.
pub fn better_solve(below: u64) -> u64 {
    if below < 1 {
        return 0;
    }
    let max = below-1;
    // Count all multiples of 3.
    (0..=max/3).map(|n| 3*n).sum::<u64>() +
    // And all multiples of 5.
    (0..=max/5).map(|n| 5*n).sum::<u64>() -
    // Remove multiples of 15 that were counted twice.
    (0..=max/15).map(|n| 15*n).sum::<u64>()
}

/// The best way to solve is to use a formula though.
/// We know that the sum of the N first integer is N*(N+1)/2 so the sum of the N first multiples of
/// k is k*N*(N+1)/2.
/// We sum this formula for 3 and for 5 and substract for 15 as these are counted twice.
pub fn solve(below: u64) -> u64 {
    if below < 1 {
        return 0;
    }
    let max = below - 1;
    let (n3, n5, n15) = (max/3, max/5, max/15);
    3*n3*(n3+1)/2 + 5*n5*(n5+1)/2 - 15*n15*(n15+1)/2
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The example provided in the problem.
    /// This is not a great test though as there are no multiple of 15.
    #[test]
    fn test_10() {
        assert_eq!(23, naive_solve(10));
        assert_eq!(23, better_solve(10));
        assert_eq!(23, solve(10));
    }

    /// The solution to the problem.
    /// It's a pitty that such a small number was used as all the solutions are extremely fast
    /// anyway.
    #[test]
    fn test_1000() {
        assert_eq!(233168, naive_solve(1000));
        assert_eq!(233168, better_solve(1000));
        assert_eq!(233168, solve(1000));
    }

    /// A test that all solutions agree.
    /// This is great to test extensively the more complex solutions based on the trivial one.
    #[test]
    fn test_all_agree() {
        for n in 0..1000 {
            assert_eq!(naive_solve(n), better_solve(n));
            assert_eq!(naive_solve(n), solve(n));
        }
    }
}
