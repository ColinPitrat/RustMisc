use crate::log_verbose;
use std::collections::HashMap;

/// Returns a^n % p.
fn pow_mod(a: u64, n: u64, p: u64) -> u64 {
    match n {
        0 => 1,
        1 => a % p,
        _ => {
            let m = pow_mod(a, n/2, p) as u128;
            if n % 2 == 0 {
                ((m * m)  % p as u128) as u64
            } else {
                (a as u128 * ((m * m) % p as u128) % p as u128) as u64
            }
        },
    }
}

// Let's see where the last euler coin is...
// We have (for p prime):
//  a ^ (p-1) = 1 mod p
// So the inverse of a is a ^ (p-2).
/// Returns the inverse of a modulo p.
fn inverse_mod(a: u64, p: u64) -> u64 {
    pow_mod(a, p-2, p)
}

/// Returns x such that a^x % p = b.
/// Uses baby-step giant-step algorithm:
/// https://en.wikipedia.org/wiki/Baby-step_giant-step
fn log_mod(a: u64, b: u64, p: u64) -> u64 {
    let m = (p as f64).sqrt().ceil() as u64;
    let mut alphas = vec!();
    let mut alphas_rev = HashMap::new();
    let mut alpha = 1;
    for j in 0..=m {
        alphas.push(alpha);
        alphas_rev.insert(alpha, j);
        alpha = ((alpha as u128 * a as u128) % p as u128) as u64;
    }
    let inv_alpha_m = inverse_mod(alphas[m as usize], p);
    let mut gamma = b;
    for i in 0..=m {
        if alphas_rev.contains_key(&gamma) {
            return i*m + alphas_rev[&gamma];
        }
        gamma = ((gamma as u128 * inv_alpha_m as u128) % p as u128) as u64;
    }
    0
}

pub fn solve_naive(limit: Option<u64>) -> u64 {
    let euler = 1504170715041707;
    let m = 4503599627370517;

    let mut coins = 0;
    let mut n = 0_u64;
    let mut sum = 0;
    let mut min = euler+1;
    let mut value = 0;

/*
    // Reinitializing from the best value found so far
    // New euler coin ( 60    715439060875):             4765 - Sum so far:     1517926517696910
    let mut coins = 60;
    let mut n = 715439060875;
    let mut sum = 1517926517696910;
    let mut value = 4765;
    let mut min = value;
*/
    while limit.is_none() || coins < limit.unwrap() {
        value = (value + euler) % m;
        if value < min {
            sum += value;
            coins += 1;
            min = value;
            log_verbose!("New euler coin ({coins:3} {n:16}): {value:16} - Sum so far: {sum:20}");
            if value == 1 {
                break;
            }
        }
        n += 1;
    }
    sum
}

pub fn solve(limit: Option<u64>) -> u64 {
    let euler = 1504170715041707;
    let m = 4503599627370517;

    let inv_euler = inverse_mod(euler, m);
    log_verbose!("Last euler coin is at {}", inv_euler);
    let verify = (inverse_mod(euler, m) as u128 * euler as u128) % m as u128;
    log_verbose!("Verification: {verify}");

    let mut coins = 0;
    let mut n = 0_u64;
    let mut sum = 0;
    let mut min = euler+1;
    let mut candidate_max = m+1;
    let mut value = 0;
    let mut candidates = vec!();

    while limit.is_none() || coins < limit.unwrap() {
        value = (value + euler) % m;
        if value < min {
            sum += value;
            coins += 1;
            min = value;
            log_verbose!("New euler coin ({coins:3} {n:16}): {value:16} - Sum so far: {sum:20}");
            if value == 1 {
                break;
            }
        }
        n += 1;
        // Coin of value n is at rank c
        let c = ((inv_euler as u128*n as u128) % m as u128) as u64;
        if c < candidate_max {
            candidates.push((c, n));
            candidate_max = c;
        }
        if candidate_max <= n {
            break;
        }
    }
    log_verbose!("Sorting candidates");
    candidates.sort_by_key(|x| x.0);
    for (n, value) in candidates.iter() {
        if limit.is_some() && coins >= limit.unwrap() {
            break;
        }
        if *value < min {
            sum += *value;
            coins += 1;
            min = *value;
            log_verbose!("New euler coin ({coins:3} {n:16}): {value:16} - Sum so far: {sum:20}");
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_mod() {
        assert_eq!(1, pow_mod(3, 0, 5));
        assert_eq!(3, pow_mod(3, 1, 5));
        assert_eq!(4, pow_mod(3, 2, 5));
        assert_eq!(2, pow_mod(3, 3, 5));
        assert_eq!(1, pow_mod(3, 4, 5));
        assert_eq!(1, pow_mod(3, 16, 5));

        assert_eq!(1, pow_mod(5, 0, 7));
        assert_eq!(5, pow_mod(5, 1, 7));
        assert_eq!(4, pow_mod(5, 2, 7));
        assert_eq!(6, pow_mod(5, 3, 7));
        assert_eq!(2, pow_mod(5, 4, 7));
        assert_eq!(3, pow_mod(5, 5, 7));
        assert_eq!(1, pow_mod(5, 6, 7));
    }

    #[test]
    fn test_inverse_mod() {
        assert_eq!(2, inverse_mod(3, 5));
        assert_eq!(4, inverse_mod(4, 5));

        assert_eq!(3, inverse_mod(5, 7));
        assert_eq!(2, inverse_mod(4, 7));
        assert_eq!(6, inverse_mod(6, 7));
    }

    #[test]
    fn test_log_mod() {
        assert_eq!(4, log_mod(3, 13, 17));
        assert_eq!(7, log_mod(3, 11, 17));
        assert_eq!(12, log_mod(3, 4, 17));
    }

    #[test]
    fn test_solve() {
        assert_eq!(1513083232796311, solve(Some(2)));
    }
}
