use crate::fibonacci;
use crate::log_verbose;

type Result = u128;

fn fibonacci(limit: usize) -> Vec<Result> {
    fibonacci::Fibonacci::<Result>::new().take(limit).collect::<Vec<_>>()
}

pub fn solve(period: usize, max_modulo: Result) -> usize {
    let f = fibonacci(period+2);
    let mut result = 0;
    'outer: for m in 1..=max_modulo {
        if (f[0] % m == f[period] % m) && (f[1] % m == f[period+1] % m) {
            for p in 1..=(period+1)/2 {
                if (f[0] % m == f[p] % m) && (f[1] % m == f[p+1] % m) {
                    continue 'outer;
                }
            }
            log_verbose!("Modulo {m} has period {period}"); 
            result += m as usize;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(vec!(1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89), fibonacci(11));
    }

    #[test]
    fn test_solve() {
        assert_eq!(57, solve(18, 50));
    }
}
