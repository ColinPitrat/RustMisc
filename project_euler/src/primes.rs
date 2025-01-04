use bit_set::BitSet;

/// Returns the list of prime numbers up to `n`.
/// This is decently fast (<2s) up to 100 millions in release mode.
pub fn eratosthenes(n: u64) -> Vec<u64> {
    let mut s = BitSet::from_bytes(&std::iter::repeat(0xff).take((n as usize+7)/8).collect::<Vec<_>>());
    for i in n+1..(n+7) {
        s.remove(i as usize);
    }
    s.remove(0);
    s.remove(1);
    for p in 2..=n as usize {
        if s.contains(p) {
            let mut m = p+p;
            while m <= n as usize {
                s.remove(m as usize);
                m += p;
            }
        }
    }
    s.into_iter().map(|x| x as u64).collect::<Vec<u64>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eratosthenes() {
        let want = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97);
        assert_eq!(want, eratosthenes(100));
    }

    #[test]
    fn test_eratosthenes_counts() {
        let values_to_check = vec!(
            (1000, 168),
            (10000, 1229),
            (100000, 9592),
            (1000000, 78498),
        );
        for (r, v) in values_to_check.iter() {
            assert_eq!(*v, eratosthenes(*r).len());
        }
    }
}
