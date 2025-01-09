use crate::log_verbose;

/// Verify whether `square` is a S-number assuming `root` is its square root.
/// The return value is meaningless if `root` is not the square root of `square`.
/// (Technically it returns whether the digits of `square` can be rearranged in numbers that sum-up
/// to `root`).
fn is_s_number(root: u64, square: u64) -> bool {
    if square == 0 {
        return root == 0;
    }
    if square < root {
        return false;
    }
    let mut modulo = 10;
    loop {
        let this = square % modulo;
        if root < this {
            return false;
        }
        let remain = square / modulo;
        if is_s_number(root - this, remain) {
                return true
        }
        if modulo > square {
            break;
        }
        modulo *= 10;
    }
    false
}

/// Returns the sum of S-numbers up to `n`.
/// This solves problem 718 in <2s.
pub fn solve(n: u64) -> u64 {
    let s = (n as f64).sqrt() as u64;
    let mut sum = 0;
    for root in 2..=s {
        let square = root*root;
        if is_s_number(root, square) {
            log_verbose!("{square} is a S-number");
            sum += square;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_s_number() {
        assert_eq!(false, is_s_number(9, 80));
        assert_eq!(true, is_s_number(9, 81));
        assert_eq!(false, is_s_number(9, 82));

        assert_eq!(true, is_s_number(82, 6724));
        assert_eq!(true, is_s_number(91, 8281));
        assert_eq!(true, is_s_number(99, 9801));
    }

    #[test]
    fn test_solve() {
        assert_eq!(41333, solve(10_u64.pow(4)));
    }
}
