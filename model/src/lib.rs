pub fn factorial(n: i32) -> i32 {
    if n <= 0 {
        return 1;
    }
    n * factorial(n-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_negative_number() {
        assert_eq!(factorial(-42), 1);
        assert_eq!(factorial(-37), 1);
        assert_eq!(factorial(-10), 1);
        assert_eq!(factorial(-1), 1);
    }

    #[test]
    fn factorial_zero_is_one() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_positive_numbers() {
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}
