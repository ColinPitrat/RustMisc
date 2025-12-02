use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(FromArgs)]
/// Solve day 1 of Advent of Code 2025.
struct Day1Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "L"),
            Direction::Right => write!(f, "R"),
        }
    }
}

impl Direction {
    fn parse(repr: char) -> Result<Direction, Box<dyn Error>> {
        match repr {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(format!("Unknown direction '{}'", repr).into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    dir: Direction,
    steps: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{}{}", self.dir, self.steps))
    }
}

#[derive(Debug)]
struct Dial {
    pos: isize,
}

impl Dial {
    fn new() -> Self {
        Dial { pos: 50 }
    }

    // Simple but inefficient implementation, good as a ground truth to compare against.
    #[allow(dead_code)]
    fn execute_slow(&mut self, instr: &Instruction) -> usize {
        print!("Execute {instr:5} from {:3}: ", self.pos);
        let d = match instr.dir {
            Direction::Left => -1,
            Direction::Right => 1,
        };
        let mut crosses = 0;
        for _ in 0..instr.steps {
            self.pos += d;
            self.pos = self.pos.rem_euclid(100);
            if self.pos == 0 {
                crosses += 1;
            }
        }
        println!("{crosses:2} crosses, landing on {:3}", self.pos);
        crosses
    }

    fn execute(&mut self, instr: &Instruction) -> usize {
        print!("Execute {instr:5} from {:3}: ", self.pos);
        let steps = instr.steps;
        let mut crosses = steps / 100;
        let steps = steps % 100;
        match instr.dir {
            Direction::Left => {
                // If pos == 0, we already counted it.
                if steps >= self.pos as usize && self.pos != 0 {
                    crosses += 1;
                }
                self.pos -= instr.steps as isize;
            },
            Direction::Right => {
                if steps >= 100 - self.pos as usize{
                    crosses += 1;
                }
                self.pos += instr.steps as isize;
            },
        }
        self.pos = self.pos.rem_euclid(100);
        println!("{crosses:2} crosses, landing on {:3}", self.pos);
        crosses
    }

    fn count_zeroes(&mut self, instructions: &Vec<Instruction>) -> usize {
        let mut count = 0;
        for instr in instructions.iter() {
            self.execute(instr);
            if self.pos == 0 {
                count += 1
            }
        }
        count
    }

    fn count_crossing_zeroes(&mut self, instructions: &Vec<Instruction>) -> usize {
        let mut count = 0;
        for instr in instructions.iter() {
            count += self.execute(instr);
        }
        count
    }
}

impl Instruction {
    // Useful for testing.
    #[allow(dead_code)]
    fn new(dir: Direction, steps: usize) -> Instruction {
        Instruction { dir, steps }
    }

    fn parse(repr: &str) -> Result<Instruction, Box<dyn Error>> {
        Ok(Instruction{
            dir: Direction::parse(repr.chars().next().ok_or("Empty instruction")?)?,
            steps: repr[1..].parse()?,
        })
    }
}

fn read_instructions(filename: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;

    let mut instructions = vec!();
    for line in content.split("\n") {
        if line.len() == 0 {
            continue;
        }

        instructions.push(Instruction::parse(line)?);
    }

    Ok(instructions)
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day1Opts = argh::from_env();

    let mut dial = Dial::new();
    let instructions = read_instructions(opts.filename.as_str())?;

    println!("Part 1: {}", dial.count_zeroes(&instructions));
    println!("Part 2: {}", dial.count_crossing_zeroes(&instructions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direction() {
        assert_eq!(Direction::Left, Direction::parse('L').unwrap());
        assert_eq!(Direction::Right, Direction::parse('R').unwrap());
        assert!(Direction::parse('?').is_err());
    }

    #[test]
    fn test_display_direction() {
        assert_eq!("L", format!("{}", Direction::Left));
        assert_eq!("R", format!("{}", Direction::Right));
    }

    #[test]
    fn test_parse_instruction() {
        let l30 = Instruction{
            dir: Direction::Left, 
            steps: 30,
        };
        let r10 = Instruction{
            dir: Direction::Right, 
            steps: 10,
        };

        assert_eq!(l30, Instruction::parse("L30").unwrap());
        assert_eq!(r10, Instruction::parse("R10").unwrap());
        assert!(Instruction::parse("10").is_err());
        assert!(Instruction::parse("RL").is_err());
    }

    #[test]
    fn test_display_instruction() {
        assert_eq!("L42", format!("{}", Instruction::parse("L42").unwrap()));
        assert_eq!("R512", format!("{}", Instruction::parse("R512").unwrap()));

        // Ensure this supports padding
        assert_eq!("L42  ", format!("{:5}", Instruction::parse("L42").unwrap()));
    }

    #[test]
    fn test_read_instructions() {
        let got = read_instructions("sample.txt").unwrap();
        let want = vec![
            Instruction::new(Direction::Left, 68),
            Instruction::new(Direction::Left, 30),
            Instruction::new(Direction::Right, 48),
            Instruction::new(Direction::Left, 5),
            Instruction::new(Direction::Right, 60),
            Instruction::new(Direction::Left, 55),
            Instruction::new(Direction::Left, 1),
            Instruction::new(Direction::Left, 99),
            Instruction::new(Direction::Right, 14),
            Instruction::new(Direction::Left, 82),
        ];

        assert_eq!(want, got);
    }

    #[test]
    fn test_execute() {
        let mut dial = Dial::new();
        let instructions = read_instructions("sample.txt").unwrap();
        let want_crossings = [1, 0, 1, 0, 1, 1, 0, 1, 0, 1];
        let want_pos = [82, 52, 0, 95, 55, 0, 99, 0, 14, 32];

        for (i, instr)in instructions.iter().enumerate() {
            assert_eq!(want_crossings[i], dial.execute(instr), "At instruction {i}");
            assert_eq!(want_pos[i], dial.pos, "At instruction {i}");
        }
    }

    #[test]
    fn test_count_zeroes() {
        let mut dial = Dial::new();
        let instructions = read_instructions("sample.txt").unwrap();

        assert_eq!(3, dial.count_zeroes(&instructions));
    }

    #[test]
    fn test_count_crossing_zeroes() {
        let mut dial = Dial::new();
        let instructions = read_instructions("sample.txt").unwrap();

        assert_eq!(6, dial.count_crossing_zeroes(&instructions));
    }

    #[test]
    fn test_count_crossing_zeroes_at_zero() {
        let mut dial = Dial::new();
        let instructions = vec![Instruction::new(Direction::Left, 50)];

        assert_eq!(1, dial.count_crossing_zeroes(&instructions));
    }

    #[test]
    fn test_count_crossing_zeroes_multiple_rounds() {
        let mut dial = Dial::new();
        let instructions = vec![Instruction::new(Direction::Left, 1000)];

        assert_eq!(10, dial.count_crossing_zeroes(&instructions));
    }

    #[test]
    fn test_count_crossing_zeroes_multiple_rounds_land_on_zero() {
        let mut dial = Dial::new();
        dial.pos = 83;
        let instructions = vec![Instruction::new(Direction::Left, 583)];

        assert_eq!(6, dial.count_crossing_zeroes(&instructions));
    }
}
