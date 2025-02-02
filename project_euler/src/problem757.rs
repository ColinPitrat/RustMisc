use crate::log_verbose;
use std::collections::HashSet;

#[allow(dead_code)]
fn is_stealthy(n: i64) -> bool {
    (1..(n as f64).sqrt() as i64 + 1)
        .filter(|x| n % x == 0)
        .map(|x| x + n/x)
        .fold((false, 0), |(acc, last), x| (acc || last-x == 1, x))
        .0
}

#[allow(dead_code)]
fn solve_naive(max: i64) -> usize {
    (1..=max).filter(|&n| is_stealthy(n)).count()
}

pub fn solve(max: i64) -> usize {
    let mut stealthy = HashSet::new();
    'outer: for a in 1..=((2.*max as f64).sqrt() as i64 + 1) {
        if a % 65536 == 0 {
            log_verbose!("a = {a}");
        }
        for b in a..=((2.*max as f64).sqrt() as i64 + 1) {
            if a*b > max {
                continue 'outer;
            }
            let delta = (a-b+1)*(a-b+1) - 4*a;
            if delta == 0 && (b-a-1) % 2 == 0 {
                let k = (b-a-1)/2;
                if (a+k)*(b-k-1) == a*b {
                    stealthy.insert(a*b);
                    //assert!(is_stealthy(a*b), "for {a}*{b}");
                    //log_verbose!("0: {} = {a}*{b} = {}*{} is stealthy", a*b, a+k, b-k-1);
                }
            } else if delta > 0 {
                let sqrt_delta = (delta as f64).sqrt() as i64;
                if sqrt_delta * sqrt_delta == delta {
                    let k1 = (b-a-1) + sqrt_delta;
                    let k2 = (b-a-1) - sqrt_delta;
                    if k1 % 2 == 0 {
                        let k = k1/2;
                        if (a+k)*(b-k-1) == a*b {
                            stealthy.insert(a*b);
                            //assert!(is_stealthy(a*b), "for {a}*{b}");
                            //log_verbose!("1: {} = {a}*{b} = {}*{} is stealthy", a*b, a+k, b-k-1);
                        }
                    }
                    if k2 % 2 == 0 {
                        let k = k2/2;
                        if (a+k)*(b-k-1) == a*b {
                            stealthy.insert(a*b);
                            //assert!(is_stealthy(a*b), "for {a}*{b}");
                            //log_verbose!("2: {} = {a}*{b} = {}*{} is stealthy", a*b, a+k, b-k-1);
                        }
                    }
                }
            }
        }
    }
    stealthy.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_stealthy() {
        let stealthy = HashSet::from([4, 12, 24, 36, 40, 60, 72, 84]);
        for n in 0..100 {
            assert_eq!(stealthy.contains(&n), is_stealthy(n), "is_stealthy({n})");
        }
        assert_eq!(true, is_stealthy(420));
        assert_eq!(true, is_stealthy(5100));
    }

    #[test]
    fn test_solve() {
        for i in 1..5 {
            assert_eq!(solve_naive(10_i64.pow(i)), solve(10_i64.pow(i)));
        }

        //assert_eq!(2851, solve(10_i64.pow(6)));
    }
}
