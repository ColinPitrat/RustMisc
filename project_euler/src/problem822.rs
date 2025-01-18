use crate::log_verbose;
use crate::primes;

// Starting with the simpler case where we're just looping over the numbers and squaring them.
// If m < n, we'll have the sum of numbers from m+1 to n + the sum of squares from 2 to m:
//  S(n, m) = (n+m+1)*(n-m)/2 + m*(m+1)*(2*m+1)/6 - 1
// And if m > n, we'll have the sum of the ceil(m/n)th powers of numbers from 2 to m
//                              and the floor(m/n)th powers of numbers from m+1 to n
// 
// But we have to take into account that we sum the smallest number first, not just loop over
// numbers. Adjusting for that means that we'll square one more time the numbers from 2 to sqrt(n),
// and yet another more time the numbers from 2 to sqrt(sqrt(n)), etc... until n^(1/(2^k)) is
// smaller than 2. Let's call Q(n) the sum of these sqrts.
// These squaring operations have to be removed from the tail of the sum, so numbers from
// m-Q(n)+1 to m are squared one time less than they should be.
//
// One difficulty is that the sum of sqrt has to be reduced if m < n and there's a perfect square
// between m-Q(n)+1 and m, as the number its the square of doesn't need to be squared.
// This is a difficulty that doesn't need to be addressed to solve the problem as n > m.

/// Returns base^exponent % modulo.
/// `base * modulo^2` must fit in `usize`.
fn pow_mod(base: usize, exponent: usize, modulo: usize) -> usize {
    match exponent {
        0 => 1,
        1 => base % modulo,
        _ => {
            let m = pow_mod(base, exponent/2, modulo);
            if exponent % 2 == 0 {
                (m * m)  % modulo
            } else {
                base * ((m * m) % modulo) % modulo
            }
        },
    }
}

pub fn solve(n: usize, m: usize, modulo: usize) -> usize {
    if m == 0 {
        return (2..=n).sum::<usize>() % modulo
    }
    let mut last = n;
    let mut squaring_bounds = vec!();
    while last > 2 {
        last = (last as f64).sqrt() as usize;
        squaring_bounds.push(last);
    }

    // TODO: if m is too small compared to n, we must do only the beginning of the first pass and
    // the order in which we do the squaring becomes important.
    // For now, these cases just won't work.
    assert!(m >= squaring_bounds.iter().sum::<usize>() - squaring_bounds.len());

    // Handle the first pass which squares all the numbers < sqrt(n) enough times to bring
    // them in the (floor(sqrt(n)), n] interval.
    // The assumption here is that n < modulo as we're not doing the modulo.
    // We can't do the modulo as we need to sort the result after that.

    assert!(n < modulo);
    let mut numbers = vec!();
    for i in 2..=n {
        let mut pow = 0;
        for &s in squaring_bounds.iter() {
            if i <= s {
                pow += 1;
            } else {
                break;
            }
        }
        numbers.push(pow_mod(i, 2_usize.pow(pow as u32), modulo));
    }
    log_verbose!("  after first pass {numbers:?}");
    numbers.sort();

    // For each value in squaring_bounds, we squared number up to it one more time.
    // This means we consume (k-1) square per value k in squaring_bounds.
    // Summing squaring_bounds gives the sum of ks.
    // The len gives the sum of 1s.
    let m = m + squaring_bounds.len() - squaring_bounds.iter().sum::<usize>();
    // The power to which each number after m % (n-1) must be raised.
    let common_pow = m / (n-1);
    let one_more_pow = m % (n-1);
    log_verbose!("squaring_bounds: {squaring_bounds:?} - m: {m} - pow: {common_pow} - one more: {one_more_pow}");
    // squaring_bounds: [100, 10, 3, 1] - 9999999999999890 - 1000100010000 - 9890

    // The code below uses:
    // 2^(p-1) = 1 mod p if p is prime
    // This won't work if p is not prime.
    // We still allow it if common_pow is low enough that the modulo won't kick in.
    // This is helpful for tests.
    assert!(common_pow < ((modulo-1) as f64).log2() as usize || primes::is_prime(modulo));
    // Handle all the next passes that square all the numbers in the list.
    for i in 0..numbers.len() {
        numbers[i] = pow_mod(numbers[i], pow_mod(2, common_pow, modulo-1), modulo);
    }
    log_verbose!("  before last pass {numbers:?}");

    // Handle the remaining "one_more_pow"
    for i in 0..numbers.len() {
        if i < one_more_pow {
            numbers[i] = (numbers[i] * numbers[i]) % modulo;
        } else {
            break;
        }
    }

    log_verbose!("  summing {numbers:?}");
    numbers.into_iter().fold(0, |acc, n| (acc + n) % modulo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(14, solve(5, 0, 1234567891));
        assert_eq!(16, solve(5, 1, 1234567891));
        assert_eq!(22, solve(5, 2, 1234567891));
        assert_eq!(34, solve(5, 3, 1234567891));
        assert_eq!(46, solve(5, 4, 1234567891));
        assert_eq!(66, solve(5, 5, 1234567891));
        assert_eq!(138, solve(5, 6, 1234567891));
        assert_eq!(378, solve(5, 7, 1234567891));
        assert_eq!(618, solve(5, 8, 1234567891));
        assert_eq!(1218, solve(5, 9, 1234567891));
        assert_eq!(7698, solve(5, 10, 1234567891));
        assert_eq!(98, solve(5, 10, 100));
        // Not yet supported as m is too small compared to n
        //assert_eq!(5051, solve(100, 1, 1234567891));
        //assert_eq!(5057, solve(100, 2, 1234567891));
        assert_eq!(845339386, solve(10, 100, 1234567891));
    }
}
