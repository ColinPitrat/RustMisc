// We have:
//   N(n) = (n! - k_0*(n-1)!) / product(k_i, i=0..9)
// numbers for each pick of n digits composed of k_0 0s, k_1 1s, etc...
// because there are n! combination but we need to remove the ones starting by a 0.
// There are (n-1)! of them for each 0, hence the k_0*(n-1)!.
// Finally, each combination is counted k_i! times for each combinations of i, so we
// need to divide by the product of k_i.
//
// For all these numbers, we have:
//  sum(T(n), combis of pick) = N(n)*(N(n)-1)/2
// because all these numbers can be ordered and there are N(n)-1 bigger than the first, N(n)-2
// bigger than the second, down to 0 bigger than the last.
//
// Finally, S(n) = sum(sum(T(n), combis of pick), all picks)
// So we need to build all the combinations of N digits numbers where at least 1 is different from
// 0 and compute sum(T(n)) for them. 

/// Returns all combinations of `n` digits where all digits are greater than `min`.
fn combis(n: usize, min: usize) -> Vec<Vec<usize>> {
    let mut result = vec!();
    if n == 0 {
        return vec!(vec!());
    }
    for i in min..=9 {
        for c in combis(n-1, i) {
            let mut new_c = c.clone();
            new_c.push(i);
            result.push(new_c)
        }
    }
    result
}

/// Transforms a list of combinations (e.g. (0, 0, 1, 2, 3)) into a pick which gives the number of
/// occurrences of each digit (e.g. (2, 1, 1, 1, 0, 0, 0, 0, 0, 0) for 2 zeroes, 1 one, 1 two and 1
/// three).
fn combis_to_pick(combis: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    combis.iter()
        .map(|combi| 
                combi.iter()
                .fold(vec!(0; 10), |mut pick, &digit| {
                        pick[digit] += 1;
                        pick
                    })
            )
        .collect::<Vec<_>>()
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

fn sum_t(pick: &Vec<usize>) -> usize {
    let n = pick.iter().sum();
    let big_n = (factorial(n) - pick[0]*factorial(n-1)) / pick.iter().map(|&k| factorial(k)).product::<usize>();
    if big_n > 0 {
        big_n*(big_n-1)/2
    } else {
        0
    }
}

pub fn solve(n: usize) -> usize {
    let mut result = 0;
    for pick in combis_to_pick(&combis(n, 0)) {
        result += sum_t(&pick);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combis() {
        assert_eq!(vec!(vec!(0), vec!(1), vec!(2), vec!(3), vec!(4), vec!(5), vec!(6), vec!(7), vec!(8), vec!(9)), combis(1, 0));
        assert_eq!(55, combis(2, 0).len());
        assert_eq!(220, combis(3, 0).len());
        //assert_eq!(293930, combis(12, 0).len());
    }

    #[test]
    fn test_combis_to_pick() {
        assert_eq!(
                vec!(
                    vec!(0, 0, 1, 2, 3, 0, 0, 0, 0, 0),
                    vec!(0, 2, 0, 0, 0, 0, 0, 0, 4, 0),
                    vec!(1, 0, 0, 0, 0, 2, 3, 4, 0, 5),
                ),
                combis_to_pick(&vec!(
                    vec!(3, 2, 4, 4, 3, 4),
                    vec!(1, 8, 8, 1, 8, 8),
                    vec!(0, 9, 5, 6, 5, 7, 9, 6, 6, 9, 7, 9, 7, 9, 7),
                ))
        );
    }

    #[test]
    fn test_solve() {
        assert_eq!(1701, solve(3));
    }
}
