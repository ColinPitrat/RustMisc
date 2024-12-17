use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 17 of Advent of Code 2024.
struct Day17Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day17Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day17Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day17Opts {
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
        if Day17Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Instruction {
    ADV,  // Divide A by 2^combo operand, result truncated stored in A.
    BXL,  // XOR B with literal operand
    BST,  // Combo operand modulo 8 written to B register
    JNZ,  // Jump to literal operand if A is not zero.
    BXC,  // XOR B and C, result in B, ignores operand.
    OUT,  // Output combo operand modulo 8.
    BDV,  // Divide A by 2^combo operand, result truncated stored in B.
    CDV,  // Divide A by 2^combo operand, result truncated stored in C.
}

impl Instruction {
    fn from(value: i64) -> Result<Self, Box<dyn Error>> {
        match value {
            0 => Ok(Instruction::ADV),
            1 => Ok(Instruction::BXL),
            2 => Ok(Instruction::BST),
            3 => Ok(Instruction::JNZ),
            4 => Ok(Instruction::BXC),
            5 => Ok(Instruction::OUT),
            6 => Ok(Instruction::BDV),
            7 => Ok(Instruction::CDV),
            _ => Err(Box::new(ParseError(format!("Unsupported instruction {}", value)))),
        }
    }
}

#[derive(Clone,Debug)]
struct Program {
    instructions: Vec<(Instruction, i64)>,
    bytes: Vec<i64>,
    code: String,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (addr, instr) in self.instructions.iter().enumerate() {
            writeln!(f, "{addr:02}: {instr:?}")?;
        }
        Ok(())
    }
}

impl Program {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        log_verbose!("Parse program: '{}'", content);
        let mut instructions = vec!();
        let bytes = content.split(',').map(|b| b.trim().parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
        let mut bytes_iter = bytes.iter();
        loop {
            let instr = if let Some(&c) = bytes_iter.next() {
                log_verbose!("Parse instruction: {}", c);
                Instruction::from(c)?
            } else {
                break;
            };
            if let Some(&operand) = bytes_iter.next() {
                log_verbose!("Parse operand: {}", operand);
                instructions.push((instr, operand));
            } else {
                return Err(Box::new(ParseError("incomplete program, operation without operand".to_string())));
            };
        }
        Ok(Self {
            instructions,
            code: content.trim().to_string(),
            bytes,
        })
    }
}

fn parse_register(content: &str) -> Result<i64, Box<dyn Error>> {
    log_verbose!("Parse register: {}", content);
    let parts = content.split(':').collect::<Vec<_>>();
    Ok(parts[1].trim().parse::<i64>()?)
}

#[derive(Clone,Debug)]
struct Process {
    a: i64,
    b: i64,
    c: i64,
    program: Program,
    program_counter: usize,
}

impl Process {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let lines = content.split('\n').collect::<Vec<_>>();
        let a = parse_register(lines[0])?;
        let b = parse_register(lines[1])?;
        let c = parse_register(lines[2])?;
        let program = Program::read(lines[4].split(':').skip(1).next().unwrap())?;
        Ok(Process{a, b, c, program, program_counter: 0})
    }

    fn combo_operand(&self, operand: i64) -> Result<i64, Box<dyn Error>> {
        match operand {
            0..=3 => Ok(operand),
            4 => Ok(self.a),
            5 => Ok(self.b),
            6 => Ok(self.c),
            _ => Err(Box::new(ParseError(format!("Invalid combo operand '{operand}'")))),
        }
    }

    fn run(&mut self) -> Result<String, Box<dyn Error>> {
        let mut result = vec!();
        while self.program_counter < self.program.instructions.len() {
            let (instr, operand) = self.program.instructions[self.program_counter];
            match instr {
                Instruction::ADV => {
                    log_verbose!(" - ADV: a = {} / 2^{} = {}", self.a, self.combo_operand(operand)?, self.a / 2_i64.pow(self.combo_operand(operand)? as u32));
                    self.a = self.a / 2_i64.pow(self.combo_operand(operand)? as u32);
                    self.program_counter += 1;
                },
                Instruction::BXL => {
                    log_verbose!(" - BXL: b = {} XOR {} = {}", self.b, operand, self.b ^ operand);
                    self.b = self.b ^ operand;
                    self.program_counter += 1;
                },
                Instruction::BST => {
                    log_verbose!(" - BST: b = {} % 8 = {}", self.combo_operand(operand)?, self.combo_operand(operand)? % 8);
                    self.b = self.combo_operand(operand)? % 8;
                    self.program_counter += 1;
                },
                Instruction::JNZ => {
                    log_verbose!(" - JNZ: {operand}");
                    if self.a == 0 {
                        self.program_counter += 1;
                    } else {
                        self.program_counter = operand as usize;
                    }
                },
                Instruction::BXC => {
                    log_verbose!(" - BXC: b = {} XOR {} = {}", self.b, self.c, self.b ^ self.c);
                    self.b = self.b ^ self.c;
                    self.program_counter += 1;
                },
                Instruction::OUT => {
                    log_verbose!(" - OUT: {} % 8 = {}", self.combo_operand(operand)?, self.combo_operand(operand)?%8);
                    result.push((self.combo_operand(operand)? % 8).to_string());
                    self.program_counter += 1;
                },
                Instruction::BDV => {
                    log_verbose!(" - BDV: b = {} / 2^{} = {}", self.a, self.combo_operand(operand)?, self.a / 2_i64.pow(self.combo_operand(operand)? as u32));
                    self.b = self.a / 2_i64.pow(self.combo_operand(operand)? as u32);
                    self.program_counter += 1;
                },
                Instruction::CDV => {
                    log_verbose!(" - CDV: c = {} / 2^{} = {}", self.a, self.combo_operand(operand)?, self.a / 2_i64.pow(self.combo_operand(operand)? as u32));
                    self.c = self.a / 2_i64.pow(self.combo_operand(operand)? as u32);
                    self.program_counter += 1;
                },
            }
        }
        Ok(result.join(","))
    }
}

#[derive(Clone,Debug)]
struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl Error for ParseError {}

// This assumes that the output length increases with A, which seems to be the case.
// We first look for a bottom and top boundary by doubling the size repeatedly, then we bisect in
// this interval.
fn bisect_output_length(process: &Process, length: usize) -> Result<(i64, i64), Box<dyn Error>> {
    let mut i = 1;
    let mut low = 0;
    let high;
    loop {
        let mut process = process.clone();
        process.a = i;
        let len = process.run()?.len();
        log_verbose!("  Estimating at {i}: {len} (want {length})");
        if len < length {
            low = i;
        }
        if len > length {
            high = i;
            break;
        }
        i *= 2;
    }

    let (mut llow, mut lhigh) = (low, high);
    while llow < lhigh {
        let middle = (lhigh + llow)/2;
        let mut process = process.clone();
        process.a = middle;
        let len = process.run()?.len();
        log_verbose!("  Estimating at {middle}: {len} (want {length})");
        if len < length {
            llow = middle + 1;
        }
        if len >= length {
            lhigh = middle;
        }
    }

    let (mut hlow, mut hhigh) = (low, high);
    while hlow < hhigh {
        let middle = (hhigh + hlow)/2;
        let mut process = process.clone();
        process.a = middle;
        let len = process.run()?.len();
        log_verbose!("  Estimating at {middle}: {len} (want {length})");
        if len <= length {
            hlow = middle + 1;
        }
        if len > length {
            hhigh = middle;
        }
    }

    Ok((llow, hhigh))
}

fn are_equal(left: &Vec<i64>, right: &Vec<i64>) -> bool {
    for (a, b) in left.iter().zip(right.iter()) {
        if a != b {
            return false;
        }
    }
    true
}

// The beginning of the output changes as A increases and cycles through 8 values (not necessarily
// distinct). When the cycle ends, the next output byte change, and so on. When the last byte as
// cycled over all values, the sequence is extended by one byte and the cycling restarts.
// This function is aimed to be called with start and end pointing to the beginning and end of
// sequences of a given length.
// It find the lowest value with the correct last byte. Then the lowest value with the correct
// second byte, etc... until the whole sequence matches.
fn bisect_same_end(process: &Process, start: i64, end: i64) -> Result<i64, Box<dyn Error>> {
    let mut i = start;
    for length in 1..=process.program.bytes.len() {
        loop {
            if i > end {
                return Err(Box::new(ParseError(format!("Reached end without finding a match on prefix {:?}", process.program.bytes[..length].to_vec()))));
            }
            let mut process = process.clone();
            process.a = i;
            let result = process.run()?;
            let bytes = result.split(',').map(|b| b.trim().parse::<i64>()).collect::<Result<Vec<_>, _>>()?;
            let last_bytes = bytes[bytes.len()-length..].to_vec();
            if are_equal(&process.program.bytes[bytes.len()-length..].to_vec(), &last_bytes) {
                log_verbose!("Looking at {i}: {last_bytes:?} compared to {:?}", process.program.bytes[bytes.len()-length..].to_vec());
                break;
            }
            i += 8_i64.pow((bytes.len()-length) as u32);
        }
    }
    Ok(i)
}

fn part2(process: &Process) -> Result<i64, Box<dyn Error>> {
    let (start, end) = bisect_output_length(&process, process.program.code.len())?;
    log_verbose!("Looking between {start} and {end}");

    let solution = bisect_same_end(&process, start, end)?;

    // Double checking that the result works:
    {
        let mut process = process.clone();
        process.a = solution;
        let result = process.run()?;
        if result != process.program.code {
            println!("Result at {solution} doesn't work: {} (want {})", result, process.program.code);
        }
    }
    Ok(solution)
}

fn main() -> Result<(), Box<dyn Error>> {
    Day17Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day17Opts::get_opts().filename.as_str())?;
    let original_process = Process::read(content.as_str())?;

    let mut process = original_process.clone();
    println!("Part 1: {}", process.run()?);

    println!("Part 2: {}", part2(&original_process)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program() {
        let program = Program::read("0,1,5,4,3,0").unwrap();

        assert_eq!(3, program.instructions.len());
        assert_eq!(Instruction::ADV, program.instructions[0].0);
        assert_eq!(Instruction::OUT, program.instructions[1].0);
        assert_eq!(Instruction::JNZ, program.instructions[2].0);
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let mut process = Process::read(content.as_str()).unwrap();

        assert_eq!(729, process.a);
        assert_eq!(0, process.b);
        assert_eq!(0, process.c);
        assert_eq!(3, process.program.instructions.len());
        assert_eq!(Instruction::ADV, process.program.instructions[0].0);
        assert_eq!(Instruction::OUT, process.program.instructions[1].0);
        assert_eq!(Instruction::JNZ, process.program.instructions[2].0);

        assert_eq!("4,6,3,5,6,3,5,2,1,0", process.run().unwrap());
    }

    #[test]
    fn test_sample2() {
        let content = fs::read_to_string("sample2.txt").unwrap();
        let original_process = Process::read(content.as_str()).unwrap();

        let mut process = original_process.clone();

        assert_eq!(2024, process.a);
        assert_eq!(0, process.b);
        assert_eq!(0, process.c);
        assert_eq!(3, process.program.instructions.len());
        assert_eq!(Instruction::ADV, process.program.instructions[0].0);
        assert_eq!(Instruction::OUT, process.program.instructions[1].0);
        assert_eq!(Instruction::JNZ, process.program.instructions[2].0);

        assert_eq!("5,7,3,0", process.run().unwrap());

        let mut process = original_process.clone();
        process.a = 117440;
        assert_eq!(process.program.code.clone(), process.run().unwrap());

        assert_eq!(117440, part2(&original_process).unwrap());
    }
}
