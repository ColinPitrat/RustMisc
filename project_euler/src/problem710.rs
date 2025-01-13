use crate::log_verbose;

type Result = i64;

/////
// Building the actual sums, serves as a base for exploring & testing.
/////

#[allow(dead_code)]
fn all_sums(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec!(vec!(0));
    }
    if n == 1 {
        return vec!(vec!(1));
    }
    let mut result = vec!();
    for s in all_sums(n-1) {
        let mut new1 = s.clone();
        new1.push(1);
        result.push(new1);
        let mut new2 = s.clone();
        (*new2.last_mut().unwrap()) += 1;
        result.push(new2);
    }
    result
}

#[allow(dead_code)]
fn is_palindromic(v: &Vec<usize>) -> bool {
    let n = v.len()-1;
    for i in 0..v.len()/2 {
        if v[i] != v[n-i] {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn palindromic_sums(n: usize) -> Vec<Vec<usize>> {
    all_sums(n).into_iter()
        .filter(|v| is_palindromic(v))
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn two_sums(n: usize) -> Vec<Vec<usize>> {
    all_sums(n).into_iter()
        .filter(|v| v.iter().any(|&e| e == 2))
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
fn twopal_sums(n: usize) -> Vec<Vec<usize>> {
    palindromic_sums(n).into_iter()
        .filter(|v| v.iter().any(|&e| e == 2))
        .collect::<Vec<_>>()
}

// Number of sums - S(n):
// It's obvious that there are 2^(n-1) sums:
// Starting from (1 , 1 , 1 , ... , 1 ), for each comma, we can chose to replace it by a + or not,
// and there are n-1 commas so that's 2^(n-1) sums.
// We also define S(0) = 1, considering there's a single way to decompose 0 in a sum, which is the
// sum of no terms. To be noted that the formula doesn't work for this case.

// Number of palindromic sums - P(n)
// For palindromic sums of length 2n + 1, there will necessarily be an odd number of terms and an
// the middle term will be odd (as the sum of the two mirrored part is necessarily even).
// We have S(n) palindromic sums with a 1 in the middle, S(n-1) with a 3 in the middle, etc ...
// We also have 1 palindromic sum with a single term: 2n+1 
// For a total of P(2n+1) = sum(S(k), k=0..n) = sum(2^(n-1), k=1..n) + 1 = 2^n 
//
// For palindromic sums of length 2n, there will either be an even number of terms or an odd number
// of terms with an even number in the middle.
// For even number of terms, there are S(n) = 2^(n-1) possibilities.
// For odd number of terms, there are S(n-1) sums with a 2, S(n-2) sums with a 4, etc ... down to
// 1 sum with a single 2n term.
// For a total of P(2n) = 2^(n-1) + sum(S(k), k=0..n-1) = 2^(n-1) + 2^(n-1) = 2^n
//
// So P(2n) = P(2n+1) = 2^n.

// Number of sums containing a two - W(n):
// There are S(n-2) sums starting with a 2 and S(n-2) sums ending with a 2.
// There are (n-3) other places to put a 2, each one having S(k)*S(n-k-2) sums.
// But doing this, we're counting m times sums that contain m 2s.
//
// So in each term, we need to remove the terms that have a 2 earlier in the sum.
// So W(n) = S(n-2) + sum((S(k)-W(k))*S(n-k-2), k=1..n-3) + S(n-2)-W(n-2)
//         = 2^(n-2) + (n-3)*2^(n-4) - sum(W(k)*2^(n-k-3), k=1..n-3) - W(n-2)
//         = (n+1)*2^(n-4) - sum(W(k)*2^(n-k-3), k=1..n-3) - W(n-2)

// Number of palindromic sums containing a two (twopal sums) - T(n):
// Similar reasoning as for P(n) except S(k) is replaced by W(k), except in the case of an odd
// number of terms and the middle term is 2.
// T(2n) = sum(W(k), k=0..n, k != n-1) + S(n-1)
// T(2n+1) = sum(W(k), k=0..n)

/// A calculator for the number of twopal sums.
/// This helps computing them efficiently and without overflow by memoizing and
/// applying modulo at each operation.
struct Calculator {
    modulo: Result,
    max_memo: usize,
    memoize_pow2: Vec<Result>,
    memoize_2s: Vec<Result>,
    memoize_sw: Vec<Result>,
    memoize_2p: Vec<Result>,
}

impl Calculator {
    fn new(modulo: Result) -> Self {
        let max_memo = 1000000000;
        Self {
            modulo,
            max_memo,
            memoize_pow2: vec!(0; max_memo),
            memoize_2s: vec!(0; max_memo),
            memoize_sw: vec!(0; max_memo),
            memoize_2p: vec!(0; max_memo),
        }
    }

    fn pow2(&mut self, n: usize) -> Result {
        if n < self.max_memo && self.memoize_pow2[n] != 0 {
            return self.memoize_pow2[n];
        }
        let max_pow = 9;
        let result = if n <= max_pow {
            (2 as Result).pow(n as u32).rem_euclid(self.modulo)
        } else {
            (self.pow2(n/2) * self.pow2((n+1)/2)).rem_euclid(self.modulo)
        };
        self.memoize_pow2[n] = result;
        result
    }

    #[allow(dead_code)]
    fn two_sums_count_ori(&mut self, n: usize) -> Result {
        if n < self.max_memo && self.memoize_2s[n] != 0 {
            return self.memoize_2s[n];
        }
        let result = match n {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 4,
            _ => {
                let mut result = (n as Result+1)*self.pow2(n-4);
                result -= self.two_sums_count_ori(n-2);
                for k in 1..=(n-3) {
                    result = (result - self.two_sums_count_ori(k)*self.pow2(n-k-3)).rem_euclid(self.modulo);
                }
                result.rem_euclid(self.modulo)
            }
        };
        self.memoize_2s[n] = result;
        result
    }

    fn two_sums_count(&mut self, n: usize) -> Result {
        if n < self.max_memo && self.memoize_2s[n] != 0 {
            return self.memoize_2s[n];
        }
        // Because W(n) = (n+1)*2^(n-4) - sum(W(k)*2^(n-k-3), k=1..n-3) - W(n-2)
        // we can reuse [ sum(W(k)*2^(n-k-3), k=1..n-3) ]  used at rank n
        // to compute [ sum(W(k)*2^(n-k-2), k=1..n-2) ] at rank n+1
        // by multiplying it by 2 and adding W(n-1).
        let result = match n {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 4,
            _ => {
                let non_sum_term = (n as Result+1)*self.pow2(n-4) - self.two_sums_count(n-2);
                let mut sum_w = if n-1 < self.max_memo && self.memoize_sw[n-1] != 0 {
                    self.memoize_sw[n-1]
                } else {
                    // W(n-1) = n*2^(n-5) - sum(W(k)*2^(n-k-4), k=1..n-4) - W(n-3)
                    //   so the sum term is ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                    let mut sum = 0;
                    for k in 1..=(n-4) {
                        sum = (sum + self.two_sums_count(k)*self.pow2(n-k-4)).rem_euclid(self.modulo);
                    }
                    self.memoize_sw[n-1] = sum;
                    sum
                };
                sum_w = (sum_w*2 + self.two_sums_count(n-3)).rem_euclid(self.modulo);
                self.memoize_sw[n] = sum_w;
                (non_sum_term - sum_w).rem_euclid(self.modulo)
            }
        };
        self.memoize_2s[n] = result;
        result
    }

    // Because T(2n+3) = T(2n+1) + W(n+1)
    //     and T(2n+2) = T(2n-1) + S(n) + W(n+1)
    #[allow(dead_code)]
    fn twopal_count_ori(&mut self, n: usize) -> Result {
        if n < self.max_memo && self.memoize_2p[n] != 0 {
            return self.memoize_2p[n];
        }
        let result = match n {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 0,
            n => if n % 2 == 0 {
                (self.twopal_count(n-3) + self.pow2(n/2-2) + self.two_sums_count_ori(n/2)).rem_euclid(self.modulo)
            } else {
                (self.twopal_count(n-2) + self.two_sums_count_ori(n/2)).rem_euclid(self.modulo)
            }
        };
        self.memoize_2p[n] = result;
        result
    }

    // Because T(2n+3) = T(2n+1) + W(n+1)
    //     and T(2n+2) = T(2n-1) + S(n) + W(n+1)
    fn twopal_count(&mut self, n: usize) -> Result {
        if n < self.max_memo && self.memoize_2p[n] != 0 {
            return self.memoize_2p[n];
        }
        let result = match n {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 0,
            n => if n % 2 == 0 {
                (self.twopal_count(n-3) + self.pow2(n/2-2) + self.two_sums_count(n/2)).rem_euclid(self.modulo)
            } else {
                (self.twopal_count(n-2) + self.two_sums_count(n/2)).rem_euclid(self.modulo)
            }
        };
        self.memoize_2p[n] = result;
        result
    }
}

pub fn solve(modulo: Result) -> usize {
    // For some reason, doing % 1000000 and checking for 0 doesn't work but doing % 1000000000 and
    // checking for this result % 1000000 does.
    // There must be some stupid mistake somewhere but I couldn't find it.
    let larger_modulo = modulo * 1000;
    let mut calc = Calculator::new(larger_modulo);
    let mut i = 0;
    let mut c = 1;
    while c != 0 || i < 42 {
        i += 1;
        c = calc.twopal_count(i);
        // Sanity checks that we never return a value greater than the modulo
        if c >= larger_modulo {
            panic!("Error at {i}: {c} > {modulo}");
        }
        // or negative.
        if c < 0 {
            panic!("Error at {i}: {c} < 0");
        }
        // Display some progress and the last value.
        if i % 10000 == 0 || c % modulo == 0 {
            log_verbose!("{i:5} = {c:?}");
        }
        if c > 42 && c % modulo == 0 {
            break;
        }
    }
    i
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pow2() {
        let mut calc = Calculator::new(1000000000000000000);
        for n in 0..18 {
            assert_eq!((2 as Result).pow(n), calc.pow2(n as usize), "for pow2({n})");
        }
    }

    #[test]
    fn test_all_sums() {
        assert_eq!(vec!(vec!(1)), all_sums(1));
        assert_eq!(vec!(vec!(1, 1), vec!(2)), all_sums(2));
        assert_eq!(vec!(vec!(1, 1, 1), vec!(1, 2), vec!(2, 1), vec!(3)), all_sums(3));
        assert_eq!(vec!(vec!(1, 1, 1, 1), vec!(1, 1, 2), vec!(1, 2, 1), vec!(1, 3), vec!(2, 1, 1), vec!(2, 2), vec!(3, 1), vec!(4)), all_sums(4));

        // Checking the S(n) = 2^(n-1) formula.
        for n in 1..10 {
            assert_eq!(2_usize.pow(n-1), all_sums(n as usize).len(), "for all_sums({n})");
        }
    }

    #[test]
    fn test_is_palindromic() {
        assert_eq!(true, is_palindromic(&vec!(1)));
        assert_eq!(true, is_palindromic(&vec!(1, 1)));
        assert_eq!(true, is_palindromic(&vec!(1, 2, 1)));
        assert_eq!(true, is_palindromic(&vec!(1, 2, 3, 2, 1)));

        assert_eq!(false, is_palindromic(&vec!(1, 2)));
        assert_eq!(false, is_palindromic(&vec!(1, 2, 3)));
        assert_eq!(false, is_palindromic(&vec!(1, 3, 3, 2, 1)));
    }

    #[test]
    fn test_palindromic_sums() {
        assert_eq!(vec!(vec!(1)), palindromic_sums(1));
        assert_eq!(vec!(vec!(1, 1), vec!(2)), palindromic_sums(2));
        assert_eq!(vec!(vec!(1, 1, 1), vec!(3)), palindromic_sums(3));
        assert_eq!(vec!(vec!(1, 1, 1, 1), vec!(1, 2, 1), vec!(2, 2), vec!(4)), palindromic_sums(4));

        for n in 1..10 {
            // Checking the P(2n) = 2^n formula.
            assert_eq!(2_usize.pow(n), palindromic_sums(2*n as usize).len(), "for palindromic_sums({})", 2*n);
            // Checking the P(2n+1) = 2^n formula.
            assert_eq!(2_usize.pow(n), palindromic_sums(2*n as usize + 1).len(), "for palindromic_sums({})", 2*n+1);
        }
    }

    #[test]
    fn test_two_sums() {
        let empty = Vec::<Vec<usize>>::new();

        assert_eq!(empty, two_sums(1));
        assert_eq!(vec!(vec!(2)), two_sums(2));
        assert_eq!(vec!(vec!(1, 2), vec!(2, 1)), two_sums(3));
        assert_eq!(vec!(vec!(1, 1, 2), vec!(1, 2, 1), vec!(2, 1, 1), vec!(2, 2)), two_sums(4));
    }

    #[test]
    fn test_two_sums_count() {
        let mut calc = Calculator::new(1000000);
        // Checking the formula:
        //  W(n) = (n+1)*2^(n-4) - sum(W(k)*2^(n-k-3), k=1..n-3) - W(n-2)
        for n in 0..20 {
            assert_eq!(calc.two_sums_count(n) as usize, two_sums(n).len(), "for two_sums_count({n})");
        }
    }

    #[test]
    fn test_two_sums_count_agree() {
        let mut calc = Calculator::new(1000000);
        // Checking the two implementations on large values:
        for n in 1000..2000 {
            assert_eq!(calc.two_sums_count_ori(n), calc.two_sums_count(n), "for two_sums_count({n})");
        }
    }

    #[test]
    fn test_twopal_sums() {
        let empty = Vec::<Vec<usize>>::new();

        assert_eq!(empty, twopal_sums(1));
        assert_eq!(vec!(vec!(2)), twopal_sums(2));
        assert_eq!(empty, twopal_sums(3));
        assert_eq!(vec!(vec!(1, 2, 1), vec!(2, 2)), twopal_sums(4));
        assert_eq!(vec!(vec!(1, 1, 2, 1, 1), vec!(1, 2, 2, 1), vec!(2, 1, 1, 2), vec!(2, 2, 2)), twopal_sums(6));

        assert_eq!(824, twopal_sums(20).len());
    }

    #[test]
    fn test_twopal_count() {
        let mut calc = Calculator::new(1000000);
        // Checking the formulas:
        // T(2n) = sum(W(k), k=0..n)
        // T(2n+1) = sum(W(k), k=0..n)
        for n in 0..20 {
            assert_eq!(calc.twopal_count(n) as usize, twopal_sums(n).len(), "for twopal_count({n})");
        }
    }

    #[test]
    fn test_twopal_count_agree() {
        let mut calc1 = Calculator::new(1000000);
        let mut calc2 = Calculator::new(1000000000);
        // Checking the two implementations on large values with different modulos:
        for n in 0..2000 {
            assert_eq!(calc1.twopal_count(n), calc2.twopal_count_ori(n) % 1000000, "for twopal_count({n})");
        }
        assert_eq!(calc1.twopal_count(68), calc2.twopal_count(68) % 1000000, "different modulos check for twopal_count(68)");
        assert_eq!(calc1.two_sums_count(34), calc2.two_sums_count(34) % 1000000, "different modulos check for two_sums_count(34)")
    }

    #[test]
    fn test_solve() {
        assert_eq!(17, solve(10));
        assert_eq!(123, solve(100));
        assert_eq!(1200, solve(1000));
        assert_eq!(6003, solve(10000));
        assert_eq!(15000, solve(100000));
        assert_eq!(1275000, solve(1000000));
    }
}
