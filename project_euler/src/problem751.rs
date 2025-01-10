use crate::log_verbose;

/// Computes tau starting from theta with `places` decimal places.
fn concatenation(theta: f64, places: usize) -> String {
    let mut result = String::new();
    let mut b1 = theta;
    let mut b2 ;
    let mut first = true;
    // + 2 because we want the leading digit and the decimal point.
    // This assumes that there's a single leading digit, which we know from the
    // problem statement (a1 = 2).
    while result.len() < places + 2 {
        let a = b1.floor() as usize;
        result.extend(format!("{a}").chars());
        if first {
            result.push('.');
            first = false;
        }
        b2 = b1.floor()*(b1 - b1.floor() + 1.);
        b1 = b2;
    }
    result
}

/// Find theta such that tau = theta to 24 decimal places.
pub fn solve() -> String {
    let places = 24;
    let mut low = 2.;
    let mut hi = 3.;
    let mut result = String::new();
    // We observe that theta - tau is increasing between 2. and 3., starting
    // negative and ending positive. So we bisect to find a result close enough
    // to 0.
    while hi - low > 10_f64.powi(-(places as i32)) {
        let mid = (hi + low) / 2.;
        result = concatenation(mid, places);
        let c = result.parse::<f64>().unwrap();
        let delta = mid - c;
        if delta < 0. {
            low = mid;
        } else if delta > 0. {
            hi = mid;
        } else {
            break;
        }
        log_verbose!("For {mid:20}: {result:30} - {delta}");
    }
    result.chars().take(places+2).collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_concatenation() {
        assert_eq!("2.3581321345589", concatenation(2.956938891377988, 13));
    }

    #[test]
    fn test_solve() {
        assert_eq!("2.223561019313554106173177", solve());
    }
}
