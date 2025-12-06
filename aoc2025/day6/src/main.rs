use argh::FromArgs;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 6 of Advent of Code 2025.
struct Day6Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Eq, PartialEq)]
struct Problem {
    operands1: Vec<u64>,
    operands2: Vec<u64>,
    op: Operation,
}

impl Problem {
    fn new(elements: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        let last = elements.len()-1;
        let op = match elements[last].trim() {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            o => return Err(format!("Unsupported operation '{o}'").into()),
        };

        let operands1 = elements[..elements.len()-1].iter()
            .map(|e| e.trim())
            .map(|e| e.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        let operands2 = (0..elements[0].len())
            .map(|i| {
                (0..elements.len()).map(|j| elements[j].chars().nth(i).unwrap())
                                   .filter(char::is_ascii_digit)
                                   .collect::<String>()
            })
            .filter(|e| !e.is_empty())
            .map(|e| e.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            operands1,
            operands2,
            op,
        })
    }

    fn solve1(&self) -> u64 {
        match self.op {
            Operation::Add => self.operands1.iter().sum(),
            Operation::Multiply => self.operands1.iter().product(),
        }
    }

    fn solve2(&self) -> u64 {
        match self.op {
            Operation::Add => self.operands2.iter().sum(),
            Operation::Multiply => self.operands2.iter().product(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Sheet {
    problems: Vec<Problem>,
}

impl Sheet {
    fn parse(repr: &str) -> Result<Sheet, Box<dyn Error>> {
        let lines = repr.split('\n')
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let last = lines.len()-1;
        let widths = lines[last].split(|c| c != ' ')
            .map(|e| e.len()+1)
            .skip(1)
            .collect::<Vec<_>>();

        let elements = lines.iter()
            .map(|line| {
                    let mut elems = vec!();
                    let mut start = 0;
                    for w in widths.iter() {
                        elems.push(&line[start..start+w]);
                        start += w;
                    }
                    elems
                })
            .collect::<Vec<_>>();

        let mut problems = vec!();
        for i in 0..elements[0].len() {
            let problem_elements = (0..elements.len()).map(|j| elements[j][i]).collect::<Vec<_>>();
            problems.push(Problem::new(problem_elements)?);
        }

        Ok(Sheet{ problems })
    }

    fn load(filename: &str) -> Result<Sheet, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn part1(&self) -> u64 {
        self.problems.iter().map(|p| p.solve1()).sum()
    }

    fn part2(&self) -> u64 {
        self.problems.iter().map(|p| p.solve2()).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day6Opts = argh::from_env();
    let sheet = Sheet::load(opts.filename.as_str())?;

    println!("Part 1: {}", sheet.part1());
    println!("Part 2: {}", sheet.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_new_add() {
        let want = Problem{
            operands1: vec![3, 7, 9],
            operands2: vec![379],
            op: Operation::Add,
        };

        assert_eq!(want, Problem::new(vec!["3", "7", "9", "+"]).unwrap());
    }

    #[test]
    fn test_problem_new_mul() {
        let want = Problem{
            operands1: vec![42, 51, 13],
            operands2: vec![451, 213],
            op: Operation::Multiply,
        };

        assert_eq!(want, Problem::new(vec!["42", "51", "13", "* "]).unwrap());
    }

    #[test]
    fn test_sheet_parse() {
        let want = Sheet {
            problems: vec![
                Problem{
                    operands1: vec![13, 7, 123],
                    operands2: vec![1, 12, 373],
                    op: Operation::Multiply,
                },
                Problem{
                    operands1: vec![42, 158, 1],
                    operands2: vec![1, 45, 281],
                    op: Operation::Add,
                },
            ],
        };

        assert_eq!(want, Sheet::parse(" 13  42\n  7 158\n123   1\n*   +  \n").unwrap());
    }

    #[test]
    fn test_solve1_problem() {
        assert_eq!(201, Problem::new(vec![" 42", "158", "  1", "+  "]).unwrap().solve1());
        assert_eq!(11193, Problem::new(vec![" 13", "  7", "123", "*  "]).unwrap().solve1());
    }

    #[test]
    fn test_solve2_problem() {
        assert_eq!(327, Problem::new(vec![" 42", "158", "  1", "+  "]).unwrap().solve2());
        assert_eq!(4476, Problem::new(vec![" 13", "  7", "123", "*  "]).unwrap().solve2());
    }

    #[test]
    fn test_part1() {
        let sheet = Sheet::load("sample.txt").unwrap();
        assert_eq!(4277556, sheet.part1());
    }

    #[test]
    fn test_part2() {
        let sheet = Sheet::load("sample.txt").unwrap();
        assert_eq!(3263827, sheet.part2());
    }
}
