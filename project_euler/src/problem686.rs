use crate::log_verbose;

/// Solve the problem.
pub fn solve(prefix: u64, nth: u64) -> u64 {
    let prefix = prefix as f64;
    let mut i = 0;
    let mut n = 0;
    loop {
        let low = prefix.log2()+10_f64.log2()*(i as f64);
        let high = (prefix + 1.).log2()+10_f64.log2()*(i as f64);
        // Interestingly, this originally failed when using f64 so I had to switch to nightly channel and enable f128 to investigate.
        // It turned out that one high value was rounded perfectly to the integer (171117413) such
        // that low.ceil() was equal to high.ceil() (they were 171117412.9883182 vs. 171117413)
        // The comparison of both floor & ceil avoid that.
        if low.ceil() < high.ceil() || low.floor() < high.floor() {
            n += 1;
            log_verbose!("Found candidate {n} at {i}: {:?} - {:?}", low, high);
            if n == nth {
                log_verbose!("Found p({prefix:?}, {nth}) = {low:?}");
                return low.ceil() as u64;
            }
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(7, solve(12, 1));
        assert_eq!(80, solve(12, 2));
        assert_eq!(12710, solve(123, 45));
    }
}
