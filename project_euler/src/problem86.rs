use crate::log_verbose;

/// Returns whether n is a square, checking on the fly.
/// This is the fastest way to do it that I could find, despite trying many options.
fn is_square(n: u64) -> bool {
    let s = (n as f64).sqrt() as u64;
    s*s == n
}

/// Computes the number of cuboids of integer side size less than n that have a shortest path
/// (staying on faces) from a corner to the opposite one of integer length.
/// This is a direct, naive translation of the problem statement.
fn count_paths(n: u64) -> u64 {
    let mut result = 0;
    for a in 1..=n {
        for b in a..=n {
            for c in b..=n {
                let l1 = (a+b)*(a+b) + c*c;
                let l2 = (a+c)*(a+c) + b*b;
                let l3 = (b+c)*(b+c) + a*a;
                let shortest = std::cmp::min(std::cmp::min(l1, l2), l3);
                if is_square(shortest) {
                    result += 1;
                    log_verbose!("Found an integer shortest path: {a}, {b}, {c} => {shortest}");
                }
            }
        }
    }
    result
}

/// Computes the number of cuboids of integer side size less than n that have a shortest path
/// (staying on faces) from a corner to the opposite one of integer length.
/// This is a faster version that uses a formula for counting all the paths for a given sum of two
/// sides sizes.
#[allow(dead_code)]
fn count_paths_fast(n: u64) -> u64 {
    let mut result = 0;
    for a in 1..=n {
        for b in 2..=2*a {
            let length = a*a + b*b;
            if is_square(length) {
                // b is the sum of two numbers, both greater than 0 and lower than a+1.
                // So if b is smaller than a+1 there are floor(b/2) possibilites (first number from 1
                // to floor(b/2), second number from b-1 to ceil(b/2)).
                // If b is bigger than a+1, there are a-ceil(b/2)+1 possibilities (first number from
                // b-a to floor(b/2), second number from a to ceil(b/2).
                let npaths = if b > a+1 { (a+1)-(b+1)/2 } else { b/2 };
                log_verbose!("Found {npaths} ({}) integer shortest path: {a}, {b} => {length}", b>n);
                result += npaths;
            }
        }
    }
    result
}

/// Find the lowest maximum side size for which the number of shortest paths is bigger than `n`.
/// This is slow as this does a linear search.
pub fn solve_linear_search(n: u64) -> u64 {
    let mut i = 1;
    loop {
        if i % 100 == 0 {
            log_verbose!("At {i}...");
        }
        if count_paths(i) >= n {
            log_verbose!("Found more than {n} integer shortest paths for cuboid of side size up to {i}");
            return i;
        }
        i += 1;
    }
}

/// Find the lowest maximum side size for which the number of shortest paths is bigger than `n`.
/// This is OKish as this does a binary search but this recomputes many times the same thing as it
/// computes the number of path for each max size.
pub fn solve_binary_search(n: u64) -> u64 {
    let mut low = 1;
    let mut high = 1;
    loop {
        if count_paths(high) > n {
            break;
        }
        high *= 2;
    }
    log_verbose!("Found a high boundary: {high}");
    while low < high {
        let mid = (low + high) / 2;
        if count_paths(mid) < n {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Find the lowest maximum side size for which the number of shortest paths is bigger than `n`.
/// This computes the number of path only once per cube size.
pub fn solve_slow(n: u64) -> u64 {
    let mut paths = 0;
    let mut a = 0;
    loop {
        for b in 1..=a {
            for c in 1..=b {
                let l1 = (a+b)*(a+b) + c*c;
                let l2 = (a+c)*(a+c) + b*b;
                let l3 = (b+c)*(b+c) + a*a;
                let shortest = std::cmp::min(std::cmp::min(l1, l2), l3);
                if is_square(shortest) {
                    paths += 1;
                    //log_verbose!("Found an integer shortest path: {a}, {b}, {c} => {shortest}");
                }
            }
        }
        if paths >= n {
            return a;
        }
        a += 1;
    }
}

/// Find the lowest maximum side size for which the number of shortest paths is bigger than `n`.
/// This computes the number of path only once per cube size and uses the same trick as
/// count_paths_fast.
pub fn solve(n: u64) -> u64 {
    let mut paths = 0;
    let mut a = 0;
    loop {
        for b in 2..=2*a {
            let length = a*a + b*b;
            if is_square(length) {
                // b is the sum of two numbers, both greater than 0 and lower than a+1.
                // So if b is smaller than a+1 there are floor(b/2) possibilites (first number from 1
                // to floor(b/2), second number from b-1 to ceil(b/2)).
                // If b is bigger than a+1, there are a-ceil(b/2)+1 possibilities (first number from
                // b-a to floor(b/2), second number from a to ceil(b/2).
                let npaths = if b > a+1 { (a+1)-(b+1)/2 } else { b/2 };
                paths += npaths;
            }
        }
        if paths >= n {
            return a;
        }
        a += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn test_count_paths() {
        assert_eq!(1975, count_paths(99));
        assert_eq!(2060, count_paths(100));
    }

    #[test]
    fn test_count_paths_agree() {
        for n in 0..100 {
            assert_eq!(count_paths(n), count_paths_fast(n), "for n={n}");
        }
    }

    #[test]
    fn test_is_square() {
        let squares = (0..10).map(|n| n*n).collect::<HashSet<_>>();
        for n in 0..100 {
            assert_eq!(squares.contains(&n), is_square(n), "for n={n}");
        }
    }

    #[test]
    fn test_all_solve_agree() {
        for n in 1..100 {
            assert_eq!(solve(n), solve_linear_search(n), "solve & solve_linear_search disagree for n={n}");
            assert_eq!(solve(n), solve_binary_search(n), "solve & solve_binary_search disagree for n={n}");
            assert_eq!(solve(n), solve_slow(n), "solve & solve_slow disagree for n={n}");
        }
    }

    #[test]
    fn test_solve() {
        assert_eq!(100, solve(2000));
    }
}
