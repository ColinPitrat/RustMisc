use argh::FromArgs;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 13 of Advent of Code 2024.
struct Day13Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day13Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day13Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day13Opts {
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
        if Day13Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
}

impl Machine {
    fn solve(&self, offset: f64) -> (f64, f64) {
        // Solution of the system:
        //  ax + by = c
        //  dx + ey = f
        // is:
        //  x = (bf - ce) / (bd - ae) 
        //  y = (cd - af) / (bd - ae) 
        let d = self.b.0*self.a.1 - self.b.1*self.a.0;
        (
            (self.b.0*(self.prize.1+offset) - self.b.1*(self.prize.0+offset)) / d,
            (self.a.1*(self.prize.0+offset) - self.a.0*(self.prize.1+offset)) / d
        )
    }
}

#[derive(Clone,Debug)]
struct Machines {
    machines: Vec<Machine>,
}

fn parse_line(line: Option<&str>, prefix: &str, x_prefix: &str, y_prefix: &str) -> Result<(f64, f64), Box<dyn Error>> {
    if let Some(line) = line {
        let line = line.strip_prefix(prefix).ok_or(format!("missing prefix '{prefix}'"))?;
        let elems = line.split(",").collect::<Vec<_>>();
        // TODO: Replace unwrap by 
        let x = elems[0].strip_prefix(x_prefix).ok_or(format!("missing prefix '{x_prefix}'"))?.parse::<f64>()?;
        let y = elems[1].strip_prefix(y_prefix).ok_or(format!("missing prefix '{y_prefix}'"))?.parse::<f64>()?;
        Ok((x, y))
    } else {
        None.ok_or("No line")?
    }
}

fn parse_button(name: &str, line: Option<&str>) -> Result<(f64, f64), Box<dyn Error>> {
    let prefix = format!("Button {name}: ");
    parse_line(line, prefix.as_str(), "X+", " Y+")
}

fn parse_prize(line: Option<&str>) -> Result<(f64, f64), Box<dyn Error>> {
    parse_line(line, "Prize: ", "X=", " Y=")
}

impl Machines {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut machines = vec!();
        let mut lines = content.split("\n");
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            let a = parse_button("A", line)?;
            let b = parse_button("B", lines.next())?;
            let prize = parse_prize(lines.next())?;

            machines.push(Machine{a, b, prize});
            if let None = lines.next() {
                break;
            }
        }
        Ok(Self{machines})
    }

    fn solve(&self, offset: f64) -> f64 {
        let mut result = 0.;
        for machine in self.machines.iter() {
            let (a, b) = machine.solve(offset);
            log_verbose!("Solution: {a} A and {b} B");
            if a.is_nan() || b.is_nan() {
                log_verbose!("  Not a number, skipping");
                continue;
            }
            let epsilon = 1e-6;
            if (a - a.round()).abs() > epsilon || (b - b.round()).abs() > epsilon {
                log_verbose!("  Non-integer solution, skipping");
                continue;
            }
            result += a*3. + b;
        }
        result
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day13Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day13Opts::get_opts().filename.as_str())?;
    let machines = Machines::read(content.as_str())?;
    log_verbose!("Reading '{machines:?}'");
    println!("Part 1: {}", machines.solve(0.));
    println!("Part 2: {}", machines.solve(10000000000000.));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let machines = Machines::read(content.as_str()).unwrap();

        assert_eq!(480., machines.solve(0.));
        assert_eq!(875318608908., machines.solve(10000000000000.));
    }
}
