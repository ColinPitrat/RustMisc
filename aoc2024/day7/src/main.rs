use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 7 of Advent of Code 2024.
struct Day7Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day7Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day7Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day7Opts {
    fn get_opts() -> Self {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Self{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Self) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day7Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone, Debug)]
struct ParsingError {
    details: String
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "couldn't parse Operation: {}", self.details)
    }
}

impl std::error::Error for ParsingError {}
#[derive(Clone,Debug)]
struct Operation {
    result: u64,
    values: Vec<u64>,
}

impl Operation {
    fn read(content: &str) -> Result<Vec<Operation>, Box<dyn Error>> {
        let mut operations = vec!();
        for line in content.split("\n") {
            if line.is_empty() {
                break;
            }
            let parts = line.split(":").collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(Box::new(ParsingError{details: format!("Wrong number of parts in operation: '{}'", line)}));
            }
            let result = parts[0].parse::<u64>()?;
            let values = parts[1].split(" ").filter(|x| !x.is_empty()).map(|x| x.parse::<u64>()).collect::<Result<Vec<_>, _>>()?;
            operations.push(Operation{result, values});
        }
        Ok(operations)
    }

    fn _can_be_valid(&self, context: SolvingContext) -> bool {
        let idx = context.chosen.len() + 1;
        if idx == self.values.len() {
            return context.up_to_now == self.result;
        }
        let mut is_valid = false;
        for operator in context.operators.iter() {
            let mut new_context = context.clone();
            new_context.chosen.push(*operator);
            new_context.up_to_now = operator.apply(new_context.up_to_now, self.values[idx]);
            is_valid = is_valid || self._can_be_valid(new_context);
        }
        is_valid
    }

    fn can_be_valid(&self, allowed_operators: &Vec<Operator>) -> bool {
        self._can_be_valid(SolvingContext::new(self.values[0], allowed_operators.clone()))
    }
}

#[derive(Clone,Copy,Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn part1() -> Vec<Operator> {
    vec!(Operator::Add, Operator::Mul)
    }

    fn part2() -> Vec<Operator> {
        vec!(Operator::Add, Operator::Mul, Operator::Concat)
    }

    fn apply(&self, op1: u64, op2: u64) -> u64 {
        match self {
            Operator::Add => op1 + op2,
            Operator::Mul => op1 * op2,
            Operator::Concat => format!("{}{}", op1, op2).parse::<u64>().unwrap(),
        }
    }
}

#[derive(Clone,Debug)]
struct SolvingContext {
    up_to_now: u64,
    operators: Vec<Operator>,
    chosen: Vec<Operator>,
}

impl SolvingContext {
    fn new(initial: u64, operators: Vec<Operator>) -> Self {
        Self{
            up_to_now: initial,
            chosen: vec!(),
            operators,
        }
    }
}

fn total_calibration(operations: &Vec<Operation>, operators: &Vec<Operator>) -> u64 {
    let mut result = 0;
    for op in operations.iter() {
        if op.can_be_valid(operators) {
            result += op.result;
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    Day7Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day7Opts::get_opts().filename.as_str())?;
    let operations = Operation::read(content.as_str())?;

    log_verbose!("Operations: {:?}", operations);
    println!("Total calibration part 1: {:?}", total_calibration(&operations, &Operator::part1()));
    println!("Total calibration part 2: {:?}", total_calibration(&operations, &Operator::part2()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_be_valid() {
        let operation = Operation{result: 190, values: vec!(19, 10)};
        assert_eq!(true, operation.can_be_valid(&Operator::part1()));
        assert_eq!(true, operation.can_be_valid(&Operator::part2()));

        let operation = Operation{result: 54, values: vec!(5, 4)};
        assert_eq!(false, operation.can_be_valid(&Operator::part1()));
        assert_eq!(true, operation.can_be_valid(&Operator::part2()));

        let operation = Operation{result: 19, values: vec!(5, 4)};
        assert_eq!(false, operation.can_be_valid(&Operator::part1()));
        assert_eq!(false, operation.can_be_valid(&Operator::part2()));
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let operations = Operation::read(content.as_str()).unwrap();

        assert_eq!(3749, total_calibration(&operations, &Operator::part1()));
        assert_eq!(11387, total_calibration(&operations, &Operator::part2()));
    }
}
