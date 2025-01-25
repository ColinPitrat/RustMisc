type Result = i64;

/*
 * Interesting facts about the sequences a_n and S_n.
 *
 *  - a_{2^n} = 2^n (immediate from a_{2n} = 2*a_n)
 *  - a_{4n+3} = -a_{4n+1} for n > 0
 *     a_{4n+3} = a_{2n+1} - 3*a_{2n+2} = a_n - 3*a_{n+1} - 6*a_{n+1} = a_n - 9*a_{n+1}
 *     a_{4n+1} = a_{2n} - 3*a_{2n+1} = 2*a_n - 3*a_n + 9*a_{n+1} = - a_n + 9*a_{n+1}
 *     Q.E.D.
 *  - a_{8n+2} = -a_{8n+6} for n > 0
 *     a_{8n+2} = 2*a_{4n+1}
 *     a_{8n+6} = 2*a_{4n+3} = -2*a_{4n+1}
 *     Q.E.D.
 *  - a_{(2^k)*n+2^(k-2)} = - a_{(2^k)*n+2^(k-2)*3} for n > 0
 *      By recurrence.
 *  - S_{2^n} = 4 - 2^{n-1}
 */

/// Returns whether n is a power of 2.
fn is_power_of_2(n: usize) -> bool {
    n != 0 && (n - 1) & n == 0
}

fn naive_an(n: usize) -> Result {
    if n < 2 {
        n as Result
    } else if n % 2 == 0 {
        2*naive_an(n/2)
    } else {
        naive_an(n/2) - 3*naive_an((n+1)/2)
    }
}

/// A struct to compute a_n with recursively with some memoization, but limited to not blow-up the
/// memory.
struct ACalculator {
    memoize: Vec<Result>,
    min_memo: usize,
    max_memo: usize,
    computed: usize,
}

impl ACalculator {
    fn new(max_n: usize) -> Self {
        // For n = 10: 2m58 // Cache hit starts to be too low
        // For n = 15: 2m25
        // For n = 20: 2m24
        // For n = 25: 2m24
        // For n = 30: 3m10 // allocating a few GB starts taking some time
        let memo_pow = std::cmp::min(20, ((max_n as f64).log2() - 2.) as u32);
        Self {
            memoize: vec!(0;2_usize.pow(memo_pow+1)),
            min_memo: 0,
            //min_memo: 2_usize.pow(memo_pow),
            max_memo: 2_usize.pow(memo_pow+1),
            computed: 0,
        }
    }

    fn an(&mut self, n: usize) -> Result {
        if n >= self.min_memo && n < self.max_memo && self.memoize[n-self.min_memo] != 0 {
            return self.memoize[n-self.min_memo];
        }
        self.computed += 1;
        let result = if n < 2 {
            n as Result
        } else if is_power_of_2(n) {
            n as Result
        } else if n % 2 == 0 {
            2*self.an(n/2)
        } else {
            self.an(n/2) - 3*self.an((n+1)/2)
        };
        if n >= self.min_memo && n < self.max_memo {
            self.memoize[n-self.min_memo] = result;
        }
        result
    }
}

/// Compute all elements of the sequence up to n and stores them.
/// This is fast until the memory blows up, which happens for n = 10.
pub fn solve_iter(n: usize) -> Result {
    let mut ans = vec!(0, 1);
    for i in 2..=n {
        ans.push(
            if i % 2 == 0 {
                2*ans[i/2]
            } else {
                ans[i/2]-3*ans[(i+1)/2]
            }
        );
    }
    ans.into_iter().sum::<Result>()
}

/// A structure to store a given optimization.
/// An optimization is based on one instance of the formula:
///   a_{(2^k)*n+2^(k-2)} = - a_{(2^k)*n+2^(k-2)*3} for n > 0
/// The trick is that it must only be applied when both numbers are in the range to be considered.
/// As we start from the upper boundary 2^p, we're fine on this side. But the lower boundary can be
/// anything (for the problem it's 10^12, but we target a generic solution) so we need to ignore
/// numbers for which (2^k)*n+2^(k-2) is lower than N.
struct Optim {
    #[allow(dead_code)]
    power: u32,
    #[allow(dead_code)]
    modulo: usize,
    start: usize,
    /// Mask is a faster alternative than modulo as modulo is a power of 2.
    mask: usize,
}

impl Optim {
    fn new(power: u32, n: usize) -> Self {
        let modulo = 2_usize.pow(power);
        let limit = 2*modulo;
        let start = n + (limit - n%limit);
        let start = std::cmp::max(limit, start);
        let mask = modulo - 1;
        Self {
            power,
            modulo,
            start,
            mask,
        }
    }

    fn all(k: u32, n: usize) -> Vec<Self> {
        let mut result = vec!();
        for i in 1..k {
            result.push(Self::new(i, n));
        }
        result
    }
}

/// The approach that got me to the solution. This is still quite slow though (~5 minutes).
/// The idea is to start at the power of 2 that is just above n (because it's the closest for
/// 10^12, not necessarily the smartest choice for other values) for which we can compute S(N) with
/// a formula and then go down to n, skipping terms that will cancel themselves.
///
/// TODO: A large part of the time is spent looping, which could be improved with a while loop and
/// modifying optims to give by how much we can increment i.
pub fn solve(n: usize) -> Result {
    let k = ((n as f64).log2()+1.) as u32;
    let mut calc = ACalculator::new(n);
    let mut result = 4 - (2 as Result).pow(k-1);
    let end = 2_usize.pow(k);
    println!("Solve for {n} - Starting from {end} down to {n} ({} values)", end-n-1);
    let optims = Optim::all(k, n);
    let mut values = 0;
    'outer: for i in n+1..=end {
        for o in optims.iter() {
            // Replaced (i % o.modulo != 0) by (i & o.mask != 0) as modulo is a power of 2.
            // This is much faster.
            if i > o.start && i & o.mask != 0 {
                continue 'outer
            }
        }
        //println!("Not skipping {i} for {n} (lim_opt = {lim_opt}, {i} - {mod_opt} = {}, {i} % {mod_opt} = {}", i-mod_opt, i%mod_opt);
        result -= calc.an(i);
        values += 1;
    }
    println!("Actual values computed: {values}");
    //println!("Total an computed: {} - Memoized: {} / {}", calc.computed, calc.memoize.iter().filter(|&x| *x != 0).count(), calc.memoize.len());
    result
}

/// Formula based approach.
/// We have S(2*n) = (a_2 + a_4 + ... + a_2n) + (a_1 + a_3 + ... + a_{2n-1})
///                = (2*a_1 + 2*a_2 + ... + 2*a_n) + (a_1 + (a_1 - 3*a_2) + ... + (a_{n-1} - 3*a_n))
///                = 2*S(n) + (2*a_1 - 2*a_2 - ... -2*a_{n-1} - 3*a_n)
///                = 2*S(n) + 4*a_1 - 2*S(n) - a_n
///                = 4*a_1 - a_n
/// And of course, S(2*n+1) = S(2*n) + a_{2*n+1}
/// These formulas are valid only for n > 0.
pub fn solve_fast(n: usize) -> Result {
    if n == 0 {
        0
    } else if n % 2 == 0 {
        4*naive_an(1) - naive_an(n/2)
    } else {
        solve(n-1) + naive_an(n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_an() {
        _ = (1..100).map(|n| println!("{n:3}: {:5}", naive_an(n))).collect::<Vec<_>>();
        assert_eq!(vec!(1, 2, -5, 4, 17, -10, -17, 8, -47, 34, 47), (1..12).map(|n| naive_an(n)).collect::<Vec<_>>());
    }

    #[test]
    fn test_solve() {
        let want = vec!(1, 3, -2, 2, 19, 9, -8, 0, -47, -13);
        assert_eq!(want, (1..=10).map(|n| solve_iter(n)).collect::<Vec<_>>());
        //assert_eq!(want, (1..=10).map(|n| solve(n)).collect::<Vec<_>>());
        assert_eq!(1, solve(1));

        // Nice pattern for powers of 2, so we could start from the closest power of 2 and go up or
        // down from there.
        assert_eq!(0, solve_iter(8));
        assert_eq!(-4, solve_iter(16));
        assert_eq!(-12, solve_iter(32));
        assert_eq!(-28, solve_iter(64));
        assert_eq!(-60, solve_iter(128));
        assert_eq!(-124, solve_iter(256));
        assert_eq!(-252, solve_iter(512));
    }

    #[test]
    fn test_all_solve_agree() {
        for i in 1..100 {
            println!("Test {i}");
            assert_eq!(solve_iter(i), solve(i), "solve_iter({i}) != solve({i})");
        }
        for i in 0..=5 {
            assert_eq!(solve_iter(10_usize.pow(i)), solve(10_usize.pow(i)), "solve_iter(10^{i}) != solve(10^{i})");
        }
    }

    #[test]
    fn test_large_solve() {
        assert_eq!(-41038236, solve(10_usize.pow(6)));
        //assert_eq!(-1595396668, solve(10_usize.pow(7)));
        //assert_eq!(3741589066033, solve(5_usize.pow(12)));
    }
}
