use argh::FromArgs;
use std::collections::{HashSet,HashMap,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::{Deref,DerefMut};
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 24 of Advent of Code 2024.
struct Day24Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

impl Day24Opts {
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
        if Day24Opts::get_opts().verbose {
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

#[derive(Clone,Debug)]
enum Operation {
    And,
    Or,
    Xor,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Operation::And => "AND",
            Operation::Or => "OR",
            Operation::Xor => "XOR",
        })
    }
}

#[derive(Clone,Debug)]
struct Gate {
    operation: Operation,
    input1: String,
    input2: String,
    output: String
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} -> {}", self.input1, self.operation, self.input2, self.output)
    }
}

impl Gate {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let elems = content.split(' ').collect::<Vec<_>>();
        match elems[1] {
            "AND" => Ok(Self::and(elems[0], elems[2], elems[4])),
            "OR" => Ok(Self::or(elems[0], elems[2], elems[4])),
            "XOR" => Ok(Self::xor(elems[0], elems[2], elems[4])),
            _ => Err(Box::new(ParseError(format!("Unsupported gate defintion: {content}")))),
        }
    }

    fn and(input1: &str, input2: &str, output: &str) -> Self {
        Self{operation: Operation::And, input1: input1.to_string(), input2: input2.to_string(), output: output.to_string()}
    }

    fn or(input1: &str, input2: &str, output: &str) -> Self {
        Self{operation: Operation::Or, input1: input1.to_string(), input2: input2.to_string(), output: output.to_string()}
    }

    fn xor(input1: &str, input2: &str, output: &str) -> Self {
        Self{operation: Operation::Xor, input1: input1.to_string(), input2: input2.to_string(), output: output.to_string()}
    }

    fn eval(&self, variables: &HashMap<String, Option<bool>>) -> Option<bool> {
        let (i1, i2) = (variables[&self.input1], variables[&self.input2]);
        if i1.is_none() || i2.is_none() {
            return None;
        }
        match self.operation {
            Operation::And => Some(i1.unwrap() & i2.unwrap()),
            Operation::Or => Some(i1.unwrap() | i2.unwrap()),
            Operation::Xor => Some(i1.unwrap() ^ i2.unwrap()),
        }
    }
}

#[derive(Clone,Debug)]
struct Circuit {
    values: HashMap<String, Option<bool>>,
    formulas: Vec<Gate>,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (var, val) in self.values.iter() {
            writeln!(f, "{var}: {val:?}")?;
        }
        writeln!(f, "");
        for formula in self.formulas.iter() {
            writeln!(f, "{formula}")?;
        }
        Ok(())
    }
}

impl Circuit {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut formulas_start = 0;
        let mut values = HashMap::new();
        for (i, line) in content.split('\n').enumerate() {
            if line.is_empty() {
                formulas_start = i+1;
                break;
            }
            log_verbose!("Parsing value {i}: {line}");
            let elems = line.split(": ").collect::<Vec<_>>();
            let (var, value) = (elems[0], elems[1].parse::<usize>()? == 1);
            values.insert(var.to_string(), Some(value));
        }
        let mut formulas = vec!();
        for line in content.split('\n').skip(formulas_start) {
            if line.is_empty() {
                continue;
            }
            log_verbose!("Parsing gate: {line}");
            formulas.push(Gate::read(line)?);
        }
        Ok(Self{values, formulas})
    }

    fn compute(&mut self) {
        let mut finished = false;
        while !finished {
            finished = true;
            for formula in self.formulas.iter() {
                if !self.values.contains_key(formula.output.as_str()) {
                    if self.values.contains_key(formula.input1.as_str()) && self.values.contains_key(formula.input2.as_str()) {
                        self.values.insert(formula.output.clone(), formula.eval(&self.values));
                    } else {
                        finished = false;
                    }
                }
            }
        }
    }

    fn var_value(&self, prefix: &str) -> Result<usize, Box<dyn Error>> {
        let mut vars = self.values.iter()
            .filter(|&(var, _)| var.starts_with(prefix))
            .collect::<Vec<_>>();
        vars.sort_by_key(|&(var, _)| std::cmp::Reverse(var));
        let binary = vars.iter().map(|&(_, val)| if val.unwrap() { "1" } else { "0" }).collect::<String>();
        //log_verbose!("Binary output: {binary}");
        Ok(usize::from_str_radix(binary.as_str(), 2)?)
    }

    fn input1(&self) -> Result<usize, Box<dyn Error>> {
        self.var_value("x")
    }

    fn input2(&self) -> Result<usize, Box<dyn Error>> {
        self.var_value("y")
    }

    fn output(&self) -> Result<usize, Box<dyn Error>> {
        self.var_value("z")
    }

    fn find_gate(&self, variable: &str) -> Option<&Gate> {
        for formula in self.formulas.iter() {
            if formula.output == variable {
                return Some(formula);
            }
        }
        None
    }

    fn expand(&self, variable: &str) -> String {
        if variable.starts_with("x") || variable.starts_with("y") {
            return variable.to_string();
        }
        let formula = self.find_gate(variable).unwrap();
        format!("({} {} {})", self.expand(formula.input1.as_str()), formula.operation, self.expand(formula.input2.as_str()))
    }

    fn equations(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for formula in self.formulas.iter() {
            if formula.output.starts_with("z") {
                result.insert(formula.output.clone(), self.expand(formula.output.as_str()));
            }
        }
        result
    }

    fn reset(&mut self) {
        self.values.retain(|var, _| var.starts_with("x") || var.starts_with("y"))
    }

    fn set(&mut self, prefix: &str, mut value: usize) {
        let mut idx = 0;
        loop {
            let var = format!("{}{:02}", prefix, idx);
            if !self.values.contains_key(&var) {
                break
            }
            let bit = (value % 2) == 1;
            self.values.insert(var, Some(bit));
            value /= 2;
            idx += 1;
        }
    }
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day24Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day24Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

fn main() -> Result<(), Box<dyn Error>> {
    Day24Opts::set_opts(argh::from_env());

    let filename = Day24Opts::get_opts().filename;
    let content = fs::read_to_string(filename.as_str())?;
    let mut circuit = Circuit::read(content.as_str())?;
    circuit.compute();
    log_verbose!("Circuit:\n{circuit}");
    let (x, y) = (circuit.input1()?, circuit.input2()?);
    log_verbose!("X = {}", x);
    log_verbose!("Y = {}", y);
    log_verbose!("Z = {}", x+y); 
    // x:  100010011110101101010110001010110100000011001
    // y:  110111100110010010111101100101110001000111001
    // _: 1011010000101000000010011110000100101001010010
    // z: 1011010000100111111110011101100101001001010010

    println!("Part 1: {}", circuit.output()?);

    // Dumping equations for each bit.
    log_verbose!("Equations:");
    let equations = circuit.equations();
    let mut vars = equations.keys().collect::<Vec<_>>();
    vars.sort();
    for var in vars {
        let eq = equations[var].clone();
        log_verbose!("  {var} = {eq}");
    }

    // At this point I've solved it manually, this was not a huge work.
    // TODO: Solve it programmatically.
    // The overall idea should be:
    //  - Try sums of 2^n with n from 0 to 45
    //    - For each invalid sum, identify which bits are wrong (e.g. for n = 16, bits 17 and 18
    //    are wrong) 
    //    - Try permutations of gates outputs for gates that are used in formulas for all the bits
    //    involved (i.e. the one set in the inputs and the wrong ones, e.g. 16, 17 and 18).
    //    Hopefully a single one work but if multiple works, keep the list.
    //  - At the end, we should have a list of 4 permutations, just sort the names involved.
    //
    // If this doesn't work or finds too many solutions or not enough permutations, we may want to
    // try more sums (e.g. 1 and 2^n - 1 which tests all the carries up to 2^n).
    let test_value = 1;
    log_verbose!("");
    log_verbose!("Sums of x=y={test_value}<<N that are incorrect:");
    for i in 0..45 {
        circuit.reset();
        circuit.set("x", test_value << i);
        circuit.set("y", test_value << i);
        circuit.compute();
        let (x, y, z) = (circuit.input1()?, circuit.input2()?, circuit.output()?);
        // Something is wrong with:
        //  - 11 and 12
        //  - 17 and 18
        //  - 26 and 27
        //  - 39 and 40
        // This means that (manually looking at the formulas):
        //   qjj & gjc are exchanged (bits 11 and 12)
        //   wmp & z17 are exchanged (carry from z17 & output z17)
        //   gvm & z26 are exchanged 
        //   qsb & z39 are exchanged
        // => result = gjc,gvm,qjj,qsb,wmp,z17,z26,z39
        if x + y != z {
            log_verbose!("Bit {i} set in X & Y:");
            log_verbose!("  X = {x:15} {x:#045b}");
            log_verbose!("  Y = {y:15} {y:#045b}");
            log_verbose!("  Z = {z:15} {z:#045b}");
            if x == z {
                log_verbose!("Bits {} and {} are switched", i, i+1);
            } else if 2*(x + y) == z {
                log_verbose!("Bits {} and {} are switched", i+1, i+2);
            } else {
                log_verbose!("Not sure what's wrong");
            }
            log_verbose!("");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
    }
}
