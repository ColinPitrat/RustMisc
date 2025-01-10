use crate::fibonacci;
use crate::log_verbose;
use std::time::Instant;

/// Returns the sum of the digits of n.
#[allow(dead_code)]
fn sum_digits(mut n: usize) -> usize {
    let mut result = 0;
    while n > 0 {
        result += n % 10;
        n = n / 10;
    }
    result
}

/// Returns the smallest number that has a digit sum of n.
#[allow(dead_code)]
fn naive_s_n(n: usize) -> usize {
    let mut i = 0;
    while sum_digits(i) != n {
        i += 1;
    }
    log_verbose!("s({n}) = {i}");
    i
}

/// Returns the smallest number that has a digit sum of n.
fn s_n(n: usize, modulo: usize) -> usize {
    // The smallest number that has a digit sum of n will be made of 9s except potentially for its
    // first digit.
    // More precisely, it's n%9 followed by n/9 9s.
    let mut result = n % 9;
    for _ in 0..(n/9) {
        result = (result*10 + 9) % modulo;
    }
    result
}

/// An object to compute increasing sums of s_n modulo some number.
/// The idea is that to compute the sum of the first N terms, you can reuse the previous sum over M
/// terms with M < N and just add the remaining N-M terms.
struct Summer {
    last_idx: usize,
    last_sum: usize,
    modulo: usize,
    pow10: Vec<usize>,
}

impl Summer {
    fn new(modulo: usize) -> Self {
        let pow10 = vec!(0;modulo);
        Self {
            last_idx: 0,
            last_sum: 0,
            modulo,
            pow10,
        }
    }

    fn sum_s_n_naive(&mut self, n: usize) -> usize {
        if n < self.last_idx {
            // We could reset, but this would mean the summer is used poorly.
            panic!("sum_s_n called with {n} < {}", self.last_idx);
        }
        while self.last_idx < n {
            self.last_idx += 1;
            self.last_sum = (self.last_sum + s_n(self.last_idx, self.modulo)) % self.modulo;
            log_verbose!("Summer computed S({:20}) = {:30}", self.last_idx, self.last_sum);
        }
        //log_verbose!("Computed S({n:20}) = {:30}", self.last_sum);
        self.last_sum
    }

    fn pow10(&mut self, n: usize) -> usize {
        if self.pow10[n] != 0 {
            return self.pow10[n];
        }
        let result = if n < 18 {
            10_usize.pow(n as u32) % self.modulo
        } else {
            (self.pow10(n/2) * self.pow10((n+1)/2)) % self.modulo
        };
        self.pow10[n] = result;
        result
    }

    /// Returns the sum of s_n numbers.
    /// That is the sum of numbers that have a digit sum of k for k from 1 to n.
    fn sum_s_n(&mut self, n: usize) -> usize {
        // The sequence of S(n) takes the value 10965 = 10999 - 5 - 29 at n=29 and from
        // then, every 9 values it takes the form 109...9 - 5 - n
        // In between, the prefix 109 goes through the values 14, 19, 25, 32, 40, 49, 59 and 79.
        if n < 29 {
            return self.sum_s_n_naive(n);
        }
        let nines = (n-2)/9;
        let m = (n-2)%9;
        //let m = (n-2)-9*nines;
        let mut result = match m {
            0 => 10,
            1 => 14,
            2 => 19,
            3 => 25,
            4 => 32,
            5 => 40,
            6 => 49,
            7 => 59,
            8 => 79,
            _ => panic!("Unexpected modulo 9 result {m}"),
        };
        // The result is:
        //   (result+1) * 10^nines - n - 6
        // We can apply the modulo on each part separately.
        // Assuming p is prime, 10^(p-1) = 1 (modulo p) so we have at most modulo values.
        // We precomputed them in self.pow10.
        let nines = nines % (self.modulo-1);
        result = ((result+1) * self.pow10(nines)) % self.modulo;
        result = ((result as i64 - n as i64 - 6) % self.modulo as i64 + self.modulo as i64) as usize;

        log_verbose!("n = {n} - m = {m} - nines = {nines} - result = {result}");
        result % self.modulo
    }
}

pub fn solve(max: usize, modulo: usize) -> usize {
    let mut summer = Summer::new(modulo);
    let mut start = Instant::now();
    fibonacci::Fibonacci::<usize>::new()
        .enumerate()
        .take(max).skip(1)
        .map(|(i, f)| {
            let s = summer.sum_s_n(f);
            log_verbose!("{i:2}: Computed S({f:20}) = {s:30} in {:.2?}", start.elapsed());
            start = Instant::now();
            s
        }
        )
        .fold(0, |acc, n| (acc + n) % modulo)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_digits() {
        assert_eq!(10, sum_digits(19));
        assert_eq!(13, sum_digits(2407));
    }

    #[test]
    fn test_s_n_agree() {
        for i in 0..20 {
            assert_eq!(naive_s_n(i), s_n(i, 1000000007), "naive_s_n({i}) != s_n({i})");
        }
    }

    #[test]
    fn test_s_n() {
        for i in 0..9 {
            assert_eq!(i, s_n(i, 1000000007));
        }
        assert_eq!(19, s_n(10, 1000000007));
    }

    #[test]
    fn test_sum_s_n() {
        let mut summer = Summer::new(1000000007);
        assert_eq!(1074, summer.sum_s_n(20));
        assert_eq!(10965, summer.sum_s_n(29));
        assert_eq!(14964, summer.sum_s_n(30));
        assert_eq!(19963, summer.sum_s_n(31));
        assert_eq!(25962, summer.sum_s_n(32));
        assert_eq!(32961, summer.sum_s_n(33));
        assert_eq!(40960, summer.sum_s_n(34));
        assert_eq!(49959, summer.sum_s_n(35));
        assert_eq!(59958, summer.sum_s_n(36));
        assert_eq!(79957, summer.sum_s_n(37));
        assert_eq!(109956,summer.sum_s_n(38));
    }

    #[test]
    fn test_solve() {
        assert_eq!(8042614, solve(10, 1000000007));
        assert_eq!(107042993, solve(20, 1000000007));
        assert_eq!(570500927, solve(40, 1000000007));
        assert_eq!(278345933, solve(50, 1000000007));
    }
}
