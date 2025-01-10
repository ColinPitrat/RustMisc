pub struct Fibonacci<Number> {
    current: Number,
    next: Number,
}

impl<Number> Fibonacci<Number> 
where
    Number: From<u8>,
{
    pub fn new() -> Self {
        Self {
            current: Number::from(0),
            next: Number::from(1),
        }
    }
}

impl<Number> std::iter::Iterator for Fibonacci<Number> 
where
    Number: std::ops::Add<Output = Number>,
    Number: Copy,
{
    type Item = Number;

    fn next(&mut self) -> Option<Number> {
        (self.current, self.next) = (self.next, self.current + self.next);
        Some(self.current)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(vec!(1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89), Fibonacci::<u64>::new().take(11).collect::<Vec<_>>());
    }
}
