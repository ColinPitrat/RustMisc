use crate::log_verbose;

use ndarray::prelude::*;

fn coefs_u() -> Vec<i64> {
    vec!(1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1)
}

fn u(n: i64) -> i64 {
    poly(&coefs_u(), n)
}

fn us(n: i64) -> Vec<i64> {
    polys(&coefs_u(), n)
}

fn poly(coefs: &Vec<i64>, n: i64) -> i64 {
    let mut pow_n = 1;
    let mut result = 0;
    for c in coefs.iter() {
        result += c * pow_n;
        pow_n *= n;
    }
    result
}

fn polys(coefs: &Vec<i64>, n: i64) -> Vec<i64> {
    (1..=n).map(|i| poly(coefs, i)).collect::<Vec<_>>()
}

fn find_index_first_non_zero(row: &ndarray::ArrayViewMut1<i64>) -> usize {
    let n = row.len();
    row.iter().enumerate().fold(n+1, |acc, (i, &e)| if acc != n+1 { acc } else { if e != 0 { i } else { n+1 } })
}

fn linear_solve(us: Vec<i64>) -> Vec<i64> {
    let n = us.len();
    // Solve A*x = b where:
    // us = 1, 8, 27
    // A = [
    //   [0^0, 0^1, 0^2],
    //   [1^0, 1^1, 1^2],
    //   [2^0, 2^1, 2^2],
    // ]
    // x = [a, b, c]
    // b = [u0, u1, u2]
    // We then have a + b*n + c*n^2 as the simplest polynomial for u.
    // Do not use ndarray_linalg because this doesn't compile for me...
    let a: ndarray::Array2<i64> = ndarray::Array::from_shape_fn((n, n), |(i, j)| (i+1).pow(j as u32) as i64);
    let b: ndarray::Array2<i64> = ndarray::Array::from_shape_fn((n, 1), |(i, _)| us[i]);
    let mut eq = ndarray::concatenate![Axis(1), a, b];

    /*
    println!("a = {a:?}");
    println!("b = {b:?}");
    println!("eq = \n{eq:?}");
    */

    // Manual Gauss method - forward pass
    let mut prev_rows: Vec<ndarray::ArrayViewMut1<i64>> = vec!();
    for mut row in eq.axis_iter_mut(Axis(0)) {
        for prev in prev_rows.iter() {
            let m_i = find_index_first_non_zero(&prev);
            let m = row[m_i] / prev[m_i];
            row.zip_mut_with(&prev, |r, p| *r -= m * *p);
            let n_i = find_index_first_non_zero(&row);
            let n = row[n_i];
            row /= n;
        }
        prev_rows.push(row);
    }
    //println!("eq = \n{eq:?}");

    // Manual Gauss method - backward pass
    let mut prev_rows: Vec<ndarray::ArrayViewMut1<i64>> = vec!();
    for mut row in eq.axis_iter_mut(Axis(0)).rev() {
        for prev in prev_rows.iter() {
            let m_i = find_index_first_non_zero(&prev);
            let m = row[m_i];
            row.zip_mut_with(&prev, |r, p| *r -= m * *p);
        }
        prev_rows.push(row);
    }
    //println!("eq = \n{eq:?}");

    eq.column(n).to_vec()
}

pub fn solve() -> i64 {
    let mut result = 0;
    let u_s = us(coefs_u().len() as i64);
    let n = u_s.len();
    log_verbose!("Values of u: {:?}", u_s);
    for i in 1..=n {
        let coefs = linear_solve(u_s[..i].to_vec());
        log_verbose!("  OP({i}, n) =  {:?}", coefs);
        if i < n {
            let j = (i+1) as i64;
            loop {
                let op_i_ip1 = poly(&coefs, j);
                let u_ip1 = u(j);
                log_verbose!("  OP({i}, {j}) = {op_i_ip1} (want {u_ip1})");
                if op_i_ip1 != u_ip1 {
                    log_verbose!("  BOP({i}, {}) = {op_i_ip1}", i+1);
                    result += op_i_ip1;
                    break;
                }
                assert!((j as usize) < n, "Couldn't find BOP for i={i}");
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_u() {
        assert_eq!(1, u(0));
        assert_eq!(1, u(1));
        assert_eq!(683, u(2));
        assert_eq!(44287, u(3));

        assert_eq!(
            vec!(1, 683, 44287, 838861, 8138021, 51828151, 247165843, 954437177, 3138105961, 9090909091),
            us(10)
        );
    }

    #[test]
    fn test_poly() {
        for i in 0..5 {
            assert_eq!(1, poly(&vec!(1), i));
            assert_eq!(i.pow(3), poly(&vec!(0, 0, 0, 1), i));
        }

        assert_eq!(1, poly(&vec!(1, 7), 0));
        assert_eq!(8, poly(&vec!(1, 7), 1));
        assert_eq!(15, poly(&vec!(1, 7), 2));

        assert_eq!(1, poly(&vec!(1, 1, 6), 0));
        assert_eq!(8, poly(&vec!(1, 1, 6), 1));
        assert_eq!(27, poly(&vec!(1, 1, 6), 2));
        assert_eq!(58, poly(&vec!(1, 1, 6), 3));
    }

    #[test]
    fn test_polys() {
        assert_eq!(vec!(1, 8, 27, 64, 125, 216), polys(&vec!(0, 0, 0, 1), 6));
    }

    #[test]
    fn test_linear_solve() {
        assert_eq!(vec!(1), linear_solve(vec!(1)));
        assert_eq!(vec!(-6, 7), linear_solve(vec!(1, 8)));
        assert_eq!(vec!(6, -11, 6), linear_solve(vec!(1, 8, 27)));
        assert_eq!(vec!(0, 0, 0, 1), linear_solve(vec!(1, 8, 27, 64)));

        assert_eq!(vec!(1, -1, 1, -1), linear_solve(polys(&vec!(1, -1, 1, -1), 4)));
    }
    
    #[test]
    fn test_solve() {
    }
}
