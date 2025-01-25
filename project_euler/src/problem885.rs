/// Compute n!.
fn factorial(n: u64) -> u64 {
    (2..=n).product()
}

/// Pre-compute factorials up to n!.
fn factorials(n: u64) -> Vec<u64> {
    (0..=n).map(|i| factorial(i)).collect::<Vec<_>>()
}

fn digits_to_number(digits: &Vec<u32>) -> u64 {
    digits.iter().fold(0, |acc, digit| 10 * acc + *digit as u64)
}

/// Return the number composed of the sorted digits of n (ignoring 0s).
fn sort_n(mut n: u64) -> u64 {
    let mut digits = vec!();
    while n > 0 {
        digits.push((n%10) as u32);
        n /= 10;
    }
    digits.sort();
    digits_to_number(&digits)
}

/// Brute force approach: apply sort_n on all numbers of less than n digits and sum the results.
pub fn solve_naive(n: u32) -> u64 {
    (0..10_u64.pow(n)).map(|i| sort_n(i)).sum()
}

/// Returns all the possible combinations of n ordered digits.
fn digits_combinations(n: u32) -> Vec<Vec<u32>> {
    if n == 0 {
        return vec!(vec!());
    }
    let mut result = vec!();
    for combi in digits_combinations(n-1) {
        let min_d = *combi.last().unwrap_or(&0);
        for d in min_d..10 {
            let mut new = Vec::with_capacity(combi.len()+1);
            new.extend(combi.clone());
            new.push(d);
            result.push(new);
        }
    }
    result
}

fn possible_numbers(digits: &Vec<u32>, factorials: &Vec<u64>) -> u64 {
    factorials[digits.len()] / digits.iter()
        .fold(&mut vec!(0; 10), |counts, x| {
            counts[*x as usize] += 1;
            counts
        }).iter()
        .map(|n| factorials[(*n) as usize])
        .product::<u64>()
}

/// Slightly smarter approach, build all the possible ordered digits combinations and count each
/// one.
/// We can deduce the number of combinations for a given list of digits as follow:
/// For digits aaabbcccc we can build 9! permutations. But for each one of them, there are 3!
/// copies permuting as, 2! permuting bs and 4! permuting cs.
/// So in the end, there are 9!/(3!2!4!) distinct numbers possible.
///
/// This turns out to be enough to solve the problem in ~1s.
pub fn solve(n: u32, modulo: u64) -> u64 {
    let factorials = factorials(n as u64);
    let mut result = 0;
    for digits in digits_combinations(n) {
        result += (digits_to_number(&digits) % modulo) * (possible_numbers(&digits, &factorials) % modulo);
        result = result % modulo;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sort_n() {
        for i in 0..20 {
            // 10 is the only number up to 20 which is not its own sorted value.
            if i == 10 {
                assert_eq!(1, sort_n(i));
            } else {
                assert_eq!(i, sort_n(i));
            }
        }
        assert_eq!(334, sort_n(3403));
        assert_eq!(12345, sort_n(53124));
    }

    #[test]
    fn test_digits_combinations() {
        assert_eq!(vec!(Vec::<u32>::new()), digits_combinations(0));
        assert_eq!(vec!(vec!(0), vec!(1), vec!(2), vec!(3), vec!(4), vec!(5), vec!(6), vec!(7), vec!(8), vec!(9)), digits_combinations(1));
        assert_eq!(vec!(
              vec!(0,0), vec!(0,1), vec!(0,2), vec!(0,3), vec!(0,4), vec!(0,5), vec!(0,6), vec!(0,7), vec!(0,8), vec!(0,9),
              vec!(1,1), vec!(1,2), vec!(1,3), vec!(1,4), vec!(1,5), vec!(1,6), vec!(1,7), vec!(1,8), vec!(1,9),
              vec!(2,2), vec!(2,3), vec!(2,4), vec!(2,5), vec!(2,6), vec!(2,7), vec!(2,8), vec!(2,9),
              vec!(3,3), vec!(3,4), vec!(3,5), vec!(3,6), vec!(3,7), vec!(3,8), vec!(3,9),
              vec!(4,4), vec!(4,5), vec!(4,6), vec!(4,7), vec!(4,8), vec!(4,9),
              vec!(5,5), vec!(5,6), vec!(5,7), vec!(5,8), vec!(5,9),
              vec!(6,6), vec!(6,7), vec!(6,8), vec!(6,9),
              vec!(7,7), vec!(7,8), vec!(7,9),
              vec!(8,8), vec!(8,9),
              vec!(9,9)
          ), digits_combinations(2));
        assert_eq!(220, digits_combinations(3).len());
    }

    #[test]
    fn test_solve_naive() {
        assert_eq!(45, solve(1, u64::MAX));
        assert_eq!(1543545675, solve(5, u64::MAX));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(1, factorial(0));
        assert_eq!(1, factorial(1));
        assert_eq!(2, factorial(2));
        assert_eq!(3628800, factorial(10));
    }

    #[test]
    fn test_solve() {
        for i in 0..4 {
            assert_eq!(solve_naive(i), solve(i, u64::MAX), "solve_naive({i}) != solve({i})");
        }
        // Computed with solve_naive but takes some time!
        assert_eq!(19457757, solve(4, u64::MAX));
        assert_eq!(1543545675, solve(5, u64::MAX));
        assert_eq!(125796691845, solve(6, u64::MAX));
        //assert_eq!(10457508399075, solve(7, u64::MAX));
        //assert_eq!(882476568727677, solve(8, u64::MAX));
    }
}
