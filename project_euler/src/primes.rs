use bit_set::BitSet;

/// Returns the list of prime numbers up to `n`.
/// This is decently fast (<2s) up to 100 millions in release mode.
pub fn sieve(n: u64) -> Vec<u64> {
    // Using 0x55 ensures that all the even numbers are already set to false.
    let mut s = BitSet::from_bytes(&std::iter::repeat(0x55).take((n as usize+7)/8).collect::<Vec<_>>());
    for i in n+1..(n+7) {
        s.remove(i as usize);
    }
    s.remove(0);
    s.remove(1);
    s.insert(2);
    // For 3, there are a lot of values to remove and Euler's optimization don't save much
    // but pushing into the vector is costly, so handle them "à la Eratosthenes".
    // The same could be done for 5 (or other small primes) but in practice this doesn't save much.
    for r in 2..=n/3 {
        s.remove((3*r) as usize);
    }
    // Euler's optimization's idea is to only remove each composite number once by removing only
    // the products of p with the numbers greater than it that are remaining in the sieve.
    // Pure Eratosthenes takes ~15s on 1000000000.
    // Euler optimization gets that down to ~10s.
    let mut to_remove = vec!();
    for p in 5..=n as usize {
        if s.contains(p) {
            to_remove.clear();
            for p2 in p..=n as usize {
                if s.contains(p2) {
                    to_remove.push(p*p2);
                }
                if p*p2 > n as usize {
                    break;
                }
            }
            for r in to_remove.iter() {
                s.remove(*r);
            }
        }
    }
    s.into_iter().map(|x| x as u64).collect::<Vec<u64>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sieve() {
        let want = vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97);
        assert_eq!(want, sieve(100));
    }

    #[test]
    fn test_sieve_counts() {
        let values_to_check = vec!(
            (1000, 168),
            (10000, 1229),
            (100000, 9592),
            (1000000, 78498),
            //(1000000000, 50847534),
        );
        for (r, v) in values_to_check.iter() {
            assert_eq!(*v, sieve(*r).len());
        }
    }
}
