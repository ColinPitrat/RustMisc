use std::collections::HashMap;

/// Returns the number of partitions for integer `n` modulo `modulo`.
fn npartitions_mod(n: i64, modulo: i64, memoize: &mut HashMap<(i64, i64), i64>) -> i64 {
    if memoize.contains_key(&(n, modulo)) {
        return *memoize.get(&(n, modulo)).unwrap();
    }
    if n == 0 {
        return 1
    }
    // The recurrence formula is:
    //  p(n) = sum( (-1)^(k+1) * p(n - k*(3*k -1) / 2), k != 0 )
    // with p(0) = 1 and p(n) = 0 for n < 0.
    // We can only look at k between -(sqrt(24*n+1)-1)/6 and (sqrt(24*n+1)+1)/6 as values outside
    // this range leads to partitions for negative integers.
    let sn = (24.*(n as f64) + 1.).sqrt();
    let min = (-(sn - 1.)/6.) as i64;
    let max = ((sn + 1.)/6.) as i64;
    let mut result = 0;
    for k in min..=max {
        if k == 0 {
            continue
        }
        let term = if k%2 == 0 {
            -1
        } else {
            1
        } * npartitions_mod(n-k*(3*k-1)/2, modulo, memoize) as i64;
        result += term;
    }
    let result = result.rem_euclid(modulo);
    memoize.insert((n, modulo), result);
    result
}

/// Finds the first integer for which the number of partitions is divisible by n.
pub fn solve(n: i64) -> i64 {
    let mut i = 0;
    let mut memoize = HashMap::new();
    loop {
        if npartitions_mod(i, n, &mut memoize) == 0 {
            return i;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npartitions_mod() {
        let p_n = vec!(1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231, 297, 385, 490, 627, 792, 1002, 1255, 1575, 1958, 2436, 3010, 3718, 4565, 5604);
        let mut memoize = HashMap::new();
        for modulo in vec!(10, 27, 52, 100) {
            for (i, &p) in (0..100).zip(p_n.iter()) {
                let got = npartitions_mod(i, modulo, &mut memoize);
                let want = p%modulo;
                assert_eq!(want, got, "for npartitions_mod({i}, {modulo}), wanted {p}%{modulo}={want}, got {got}");
            }
        }
    }

    #[test]
    fn test_solve() {
        // p(5) = 7
        assert_eq!(5, solve(7));
        // p(9) = 30 is the first divisible by 6.
        assert_eq!(9, solve(6));
        // p(11) = 56 is the first divisible by 8.
        assert_eq!(11, solve(8));
    }
}
