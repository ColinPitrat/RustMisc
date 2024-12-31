use std::error::Error;
use std::fmt;

#[macro_export]
macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if crate::options::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl Error for ParseError {}
