use crate::log_verbose;

fn f(x: f64) -> f64 {
    2f64.powf(30.403243784-x.powi(2)).floor() * 10f64.powi(-9)
}

pub fn solve() -> f64 {
    let mut un = -1.;
    let mut prev_un = -1.;
    for i in 1..=10_usize.pow(12) {
        if i % (32*1024*1024) == 0 {
            log_verbose!("u_{i} = {un}");
        }
        let prev_sum = prev_un + un;
        prev_un = un;
        un = f(un);
        if un + prev_un == prev_sum {
            log_verbose!("Converged after {i} iterations: u_{} = {prev_un} - u_{i} = {un}", i-1);
            break;
        }
    }
    un + f(un)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(1.710637717, solve());
    }
}
