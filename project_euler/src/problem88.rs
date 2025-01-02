use crate::log_verbose;
use std::collections::HashMap;

/// Returns the integer partitions for n with a part that has k elements.
fn partitions_k(n: u64, k: u64) -> Vec<Vec<u64>> {
    if n == 0 && k == 0 {
        // There's a single way to partition 0 with a max size of 0.
        return vec!(vec!(0));
    }
    if k == 0 || n == 0{
        // No partition possible if k is 0 and n > 0.
        // No partition possible if n is 0 and k > 0.
        return vec!();
    }
    if k > n {
        // No partition possible with a largest partition bigger than n.
        return vec!();
    }
    // Case where only one partition has k elements.
    let mut result = partitions_k(n-1, k-1).into_iter().map(|mut p| {
        p[0] += 1;
        p
    }).collect::<Vec<_>>();
    // Case where the second partition also has k elements.
    result.extend(partitions_k(n-k, k).into_iter().map(|mut p| {
        p.insert(0, k);
        p
    }));
    result
}

/// Returns the integer partitions for n.
fn partitions(n: u64) -> Vec<Vec<u64>> {
    let mut result = vec!();
    for k in 0..=n {
        result.extend(partitions_k(n, k));
    }
    result
}

/// Returns all the factors of n (including the trivials 1 and n) in increasing order.
#[allow(dead_code)]
fn factors(n: u64) -> Vec<u64> {
    let mut result = vec!(1);
    for i in 2..=(n+1)/2 {
        if n%i == 0 {
            result.push(i);
        }
    }
    if n > 1 {
        result.push(n);
    }
    result
}

/// Returns all the prime factors of n (keys) with their multiplicities (values).
fn prime_factors(mut n: u64) -> HashMap<u64, u64> {
    let mut result = HashMap::new();
    let mut f = 2;
    while f < n {
        let mut multiplicity = 0;
        while n%f == 0 {
            multiplicity += 1;
            n /= f;
        }
        if multiplicity > 0 {
            result.insert(f, multiplicity);
        }
        f += 1;
    }
    if n > 1 {
        result.insert(n, 1);
    }
    result
}

/// Distributes `factor` `multiplicity` times in all the possible ways on the factors in `right`.
fn distribute(right: &Vec<HashMap<u64, u64>>, factor: u64, multiplicity: u64) -> Vec<HashMap<u64, u64>> {
    if multiplicity == 0 {
        return right.clone();
    }
    let mut distr_one = vec!();
    let mut copy = right.clone();
    for part in copy.iter_mut() {
        for (f, _) in part.iter() {
            let mut new = part.clone();
            *new.entry(f*factor).or_insert(0) += 1;
            *(new.get_mut(f).unwrap()) -= 1;
            if *new.get(&f).unwrap() == 0 {
                new.remove(&f);
            }
            distr_one.push(new);
        }
    }
    distribute(&distr_one, factor, multiplicity-1)
}

/// Distributes the factors in `left` and `right` in all the possible ways.
fn concatenate(left: &Vec<HashMap<u64, u64>>, right: &Vec<HashMap<u64, u64>>) -> Vec<HashMap<u64, u64>> {
    let mut result = vec!();
    if left.is_empty() {
        return right.clone();
    }
    if right.is_empty() {
        return left.clone();
    }
    for l in left.iter() {
        for r in right.iter() {
            let mut new = l.clone();
            new.extend(r);
            result.push(new);
        }
    }
    result
}

/// Compute the mulitplicative partitions of a number whose factors are `factors`.
fn mul_partitions_rec(factors: &HashMap<u64, u64>) -> Vec<HashMap<u64, u64>> {
    if factors.len() == 0 {
        return vec!();
    }
    let first = factors.keys().nth(0).unwrap();
    let first_mul = factors[&first];
    let partitions = partitions(first_mul);
    let mut result = vec!();
    if factors.len() == 1 {
        for p in partitions {
            let mut mp = vec!();
            for e in p {
                mp.push(first.pow(e as u32));
            }
            result.push(
                mp.chunk_by(|a, b| a == b)
                  .map(|v| (v[0], v.len() as u64))
                  .collect::<HashMap<_, _>>()
            );
        }
    } else {
        let mut factors = factors.clone();
        factors.remove(&first);
        let right = mul_partitions_rec(&factors);
        // We distribute n-k of the first factors on the partition of the other
        // factors and concatenate the result with the multiplicative partition
        // of the k-th power of the first factor.
        for k in 0..=first_mul {
            let left = mul_partitions(first.pow(k as u32));
            let right = distribute(&right, *first, first_mul - k);
            let new = concatenate(&left, &right);
            result.extend(new);
        }
    }
    result
}

/// Compute the mulitplicative partitions of `n`.
fn mul_partitions(n: u64) -> Vec<HashMap<u64, u64>> {
    let factors = prime_factors(n);
    mul_partitions_rec(&factors)
}

pub fn solve(min: u64, max: u64) -> u64 {
    let mut found = 0;
    let mut ps_num = vec![0; (max+1) as usize];
    let mut i = 0;
    loop {
        for part in mul_partitions(i) {
            let nb_factors = part.iter().map(|(_, m)| m).sum::<u64>();
            let nb_ones = i - part.iter().map(|(f, m)| f*m).sum::<u64>();
            let length = (nb_ones + nb_factors) as usize;
            if i <= 12 {
                log_verbose!("Products for {i}: {part:?} - factors: {nb_factors} - ones: {nb_ones} - length: {length}");
            }
            if (length as u64) < min || (length as u64) > max {
                continue;
            }
            if ps_num[length] == 0 {
                // Check if the partition can lead to a sum (necessarily true if the sum of the
                // components is lower than i).
                if part.iter().map(|(f, m)| f*m).sum::<u64>() <= i {
                    log_verbose!("Products for {i}: {part:?} - smallest PS number of length {length}");
                    ps_num[length] = i;
                    found += 1;
                }
            }
        }
        if found > max-min {
            break;
        }
        i += 1;
    }
    ps_num.sort();
    ps_num.dedup();
    ps_num.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;
    use super::*;

    #[test]
    fn test_partitions_k() {
        let none: Vec<Vec<u64>> = vec!();
        assert_eq!(vec!(vec!(0)), partitions_k(0, 0));
        for i in 1..10 {
            assert_eq!(none, partitions_k(0, i));
            assert_eq!(none, partitions_k(i, 0));
            assert_eq!(vec!(vec!(i)), partitions_k(i, i));
            assert_eq!(vec!(repeat(1).take(i as usize).collect::<Vec<_>>()), partitions_k(i, 1));
        }
        assert_eq!(vec!(vec!(3, 1, 1, 1, 1), vec!(3, 2, 1, 1), vec!(3, 2, 2), vec!(3, 3, 1)), partitions_k(7, 3));
    }

    #[test]
    fn test_partitions() {
        assert_eq!(vec!(vec!(0)), partitions(0));
        assert_eq!(vec!(vec!(1)), partitions(1));
        assert_eq!(vec!(vec!(1, 1), vec!(2)), partitions(2));
        assert_eq!(vec!(vec!(1, 1, 1), vec!(2, 1), vec!(3)), partitions(3));
        assert_eq!(vec!(vec!(1, 1, 1, 1), vec!(2, 1, 1), vec!(2, 2), vec!(3, 1), vec!(4)), partitions(4));
        assert_eq!(vec!(vec!(1, 1, 1, 1, 1), vec!(2, 1, 1, 1), vec!(2, 2, 1), vec!(3, 1, 1), vec!(3, 2), vec!(4, 1), vec!(5)), partitions(5));
    }

    #[test]
    fn test_factors() {
        assert_eq!(vec!(1), factors(1));
        assert_eq!(vec!(1, 2), factors(2));
        assert_eq!(vec!(1, 3), factors(3));
        assert_eq!(vec!(1, 5), factors(5));
        assert_eq!(vec!(1, 2, 3, 6), factors(6));
        assert_eq!(vec!(1, 2, 5, 10), factors(10));
        assert_eq!(vec!(1, 11), factors(11));
        assert_eq!(vec!(1, 2, 3, 4, 6, 12), factors(12));
    }

    #[test]
    fn test_prime_factors() {
        let no_factors = HashMap::new();
        assert_eq!(no_factors, prime_factors(0));
        assert_eq!(no_factors, prime_factors(1));
        assert_eq!(HashMap::from([(2, 1)]), prime_factors(2));
        assert_eq!(HashMap::from([(3, 1)]), prime_factors(3));
        assert_eq!(HashMap::from([(2, 2)]), prime_factors(4));
        assert_eq!(HashMap::from([(5, 1)]), prime_factors(5));
        assert_eq!(HashMap::from([(2, 1), (3, 1)]), prime_factors(6));
        assert_eq!(HashMap::from([(2, 1), (5, 1)]), prime_factors(10));
        assert_eq!(HashMap::from([(11, 1)]), prime_factors(11));
        assert_eq!(HashMap::from([(2, 2), (3, 1)]), prime_factors(12));
        assert_eq!(HashMap::from([(17, 1)]), prime_factors(17));
        assert_eq!(HashMap::from([(2, 2), (3, 1), (5, 1)]), prime_factors(60));
        assert_eq!(HashMap::from([(2, 5), (3, 1), (5, 2)]), prime_factors(2400));
    }

    /// Validates whether to vectors are the same, ignoring the order of elements.
    /// This only requires comparison on elements to be available.
    /// This doesn't support passing a custom message.
    macro_rules! assert_vec_eq {
        ($a:expr, $b:expr) => {{
            assert_eq!($a.len(), $b.len(), "Length differ for {:?} and {:?}", $a, $b);
            for e1 in $a.iter() {
                let mut found = false;
                for e2 in $b.iter() {
                    if e1 == e2 {
                        found = true;
                    }
                }
                assert!(found, "Vectors differ: {:?} and {:?}", $a, $b);
            }
        }};
    }

    #[test]
    fn test_distribute() {
        assert_vec_eq!(
            vec!(
              HashMap::from([(2, 1), (6, 1)]),
              HashMap::from([(12, 1)])
            ),
            distribute(
              &vec!(
                HashMap::from([(2, 2)]),
                HashMap::from([(4, 1)])
              ), 3, 1)
        );
        assert_vec_eq!(
            vec!(
              HashMap::from([(2, 1), (18, 1)]),
              HashMap::from([(6, 2)]),
              HashMap::from([(36, 1)])
            ),
            distribute(
              &vec!(
                HashMap::from([(2, 2)]),
                HashMap::from([(4, 1)])
              ), 3, 2)
        );
    }

    #[test]
    fn test_concatenate() {
        assert_eq!(
            vec!(HashMap::from([(2, 2), (3, 1)])),
            concatenate(
              &vec!(HashMap::from([(2, 2)])),
              &vec!(HashMap::from([(3, 1)]))
            )
        );
        assert_eq!(
            vec!(
                HashMap::from([(2, 2), (3, 2)]),
                HashMap::from([(2, 2), (9, 1)]),
                HashMap::from([(4, 1), (3, 2)]),
                HashMap::from([(4, 1), (9, 1)])
            ),
            concatenate(
              &vec!(
                HashMap::from([(2, 2)]),
                HashMap::from([(4, 1)])
              ),
              &vec!(
                HashMap::from([(3, 2)]),
                HashMap::from([(9, 1)])
              )
            )
        );
    }

    #[test]
    fn test_mul_partitions() {
        assert_vec_eq!(
            vec!(HashMap::from([(2, 1)])),
            mul_partitions(2)
        );
        assert_vec_eq!(
            vec!(
                HashMap::from([(2, 3)]), 
                HashMap::from([(4, 1), (2, 1)]), 
                HashMap::from([(8, 1)])
            ),
            mul_partitions(8)
        );
        assert_vec_eq!(
            vec!(
                HashMap::from([(2, 1), (3, 1)]), 
                HashMap::from([(6, 1)])
            ),
            mul_partitions(6)
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(30, solve(2, 6));
        assert_eq!(61, solve(2, 12));
    }
}
