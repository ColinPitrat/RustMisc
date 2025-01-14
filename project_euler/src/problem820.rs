use crate::log_verbose;
use std::collections::HashMap;

// Original "naive" implementation. Way too slow.
/// Returns the nth decimal of 1 / divisor.
#[allow(dead_code)]
fn dn_inv_naive(divisor: usize, n: usize) -> usize {
    let mut dividend = 1;
    let mut decimals = vec!();
    let mut dividends = vec!();
    let mut encountered = HashMap::new();

    let mut digits = 0;
    while !encountered.contains_key(&dividend) {
        let decimal = dividend / divisor;
        encountered.insert(dividend, digits);
        dividends.push(dividend);
        dividend = 10* (dividend - decimal * divisor);
        decimals.push(decimal);
        digits += 1;
    }
    let prefix_length = encountered[&dividend];
    log_verbose!("decimals 1/{divisor} = {decimals:?} - prefix len: {prefix_length}");
    log_verbose!("dn_inv({divisor}, {n} = {}", decimals[n%decimals.len()]);

    if n > prefix_length { 
        decimals[(n-prefix_length)%(decimals.len()-prefix_length)+prefix_length]
    } else {
        decimals[n]
    }
}

fn pow10(mut n: usize, modulo: usize) -> usize {
    let mut result = 1;
    let mut current = 10;
    while n > 0 {
        if n & 1 != 0 {
            result = (result * current) % modulo;
        }
        current = (current * current) % modulo;
        n /= 2;
    }
    result
}

/// Returns the nth decimal of 1 / divisor.
fn dn_inv(divisor: usize, n: usize) -> usize {
    // The overall idea is that:
    // d_n(1/k) = d_1(10^(n-1)/k) = d_1((10^(n-1) % k)/k) = floor(10*(10^(n-1) % k)/k)
    10 * pow10(n-1, divisor) / divisor
}

pub fn solve(n: usize) -> usize {
    let mut result = 0;
    for i in 1..=n {
        result += dn_inv(i, n);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow10() {
        assert_eq!(1, pow10(0, 1000000));
        assert_eq!(10, pow10(1, 1000000));
        assert_eq!(100, pow10(2, 1000000));
        assert_eq!(1000, pow10(3, 1000000));
        assert_eq!(10000, pow10(4, 1000000));
        assert_eq!(100000, pow10(5, 1000000));
        assert_eq!(0, pow10(6, 1000000));
    }

    #[test]
    fn test_solve() {
        assert_eq!(10, solve(7));
        assert_eq!(418, solve(100));
    }
}
