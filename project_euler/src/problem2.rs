/// Naive solve: we list all Fibonacci numbers and sum the ones that are even.
/// We stop whenever the next number is bigger than the limit provided.
pub fn solve(below: u64) -> u64 {
    let (mut a, mut b) = (1, 2);
    let mut result = 0;
    while b < below {
        if b%2 == 0 {
            result += b;
        }
        (a, b) = (b, a+b);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test based on the Fibonacci number given in the problem statement.
    #[test]
    fn test_100() {
        assert_eq!(44, solve(100));
    }

    /// The solution to the problem.
    #[test]
    fn test_4000000() {
        assert_eq!(4613732, solve(4000000));
    }
}
