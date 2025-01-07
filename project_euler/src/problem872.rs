pub fn solve(n: u64, k: u64) -> u64 {
    let mut d = n - k;
    let mut result = n;
    let mut current = n;
    let mut i = 1;
    while d > 0 {
        if d % 2 == 1 {
            result += current - i;
            current -= i;
        }
        i *= 2;
        d = d/2;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(12, solve(6, 1));
        assert_eq!(29, solve(10, 3));
    }
}
