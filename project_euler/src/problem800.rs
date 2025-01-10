use crate::primes;

pub fn solve(n: u64) -> u64 {
    let nf = n as f64;
    let ln = nf.log2();

    // The biggest prime we need to consider is if p = 2, 2^max_p <= n^n
    // i.e. max_p <= n*log(n) / log(2) = n*log2(n)
    let max_p = (nf * ln) as u64 + 1;
    let primes = primes::sieve(max_p);

    let mut result = 0;
    let mut last_q = primes.len()-1;
    for (i, p) in primes.iter().enumerate() {
        let pf = *p as f64;
        let lp = pf.log2();

        let mut found = false;
        // We want q bigger than p. We only need to look up to last_q as if a q was too big for a
        // given p, it will be too big for any larger p.
        for (j, q) in primes[i+1..=last_q].iter().enumerate().rev() {
            let qf = *q as f64;
            let lq = qf.log2();
            // We want p^q*q^p <= n^n
            // That is q*log(p)+p*log(q) <= n*log(n).
            if pf*lq + qf*lp <= nf*ln {
                last_q = j+i+1;
                // If this is the case, it will also be true for any smaller q.
                result += (j+1) as u64;
                found = true;
                break;
            }
        }
        // If we didn't find any q for this p, we won't find any for any larger p.
        if !found {
            break;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        // 5^5 = 3125 - Only 2 hybrids below that:
        //  - 2^3*3^2 = 72
        //  - 2^5*5^2 = 800
        assert_eq!(2, solve(5));
        // The next ones being 
        //  - 2^7*7^2 = 6272
        //  - 3^5*5^3 = 30375
        // which are reached before 6^6 = 46656.
        assert_eq!(4, solve(6));
        // Then come:
        //  - 2^11*11^2 = 247808
        //  - 3^7*7^3 = 750141
        // which are below 7^7 = 823543
        assert_eq!(6, solve(7));

        // Given example.
        assert_eq!(10790, solve(800));
    }
}
