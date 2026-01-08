use argh::FromArgs;
use mathru::algebra::linear::{vector::Vector, matrix::General};
use mathru::algebra::linear::matrix::{Solve};
use microlp::{Problem, OptimizationDirection, ComparisonOp};
use std::collections::VecDeque;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 10 of Advent of Code 2025.
struct Day10Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum LightStatus {
    On,
    Off,
}

impl LightStatus {
    fn parse(repr: char) -> Result<Self, Box<dyn Error>> {
        match repr {
            '.' => Ok(LightStatus::Off),
            '#' => Ok(LightStatus::On),
            _ => Err(format!("Unknown light status: '{repr}'").into()),
        }
    }

    fn toggle(&self) -> Self {
        match self {
            LightStatus::On => LightStatus::Off,
            LightStatus::Off => LightStatus::On,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Button {
    id: usize,
    lights: Vec<usize>,
}

impl Button {
    fn new(id: usize, lights: Vec<usize>) -> Self {
        Button {
            id,
            lights,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Instruction {
    id: usize,
    lights: Vec<LightStatus>,
    buttons: Vec<Button>,
    joltages: Vec<u64>,
}

impl Instruction {
    fn parse(id: usize, repr: &str) -> Result<Self, Box<dyn Error>> {
        let parts = repr.split(' ').collect::<Vec<_>>();
        let lights = parts[0].chars()
            .skip(1)
            .filter(|c| *c != ']')
            .map(|c| LightStatus::parse(c))
            .collect::<Result<Vec<_>, _>>()?;
        let buttons = parts[1..parts.len()-1].iter()
            .enumerate()
            .map(|(id, button)| button.trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|light| light.parse::<usize>())
                    .collect::<Result<Vec<_>, _>>()
                    .map(|lights| Button::new(id, lights)))
            .collect::<Result<Vec<_>, _>>()?;
        let joltages = parts[parts.len()-1].trim_matches(|c| c == '{' || c == '}')
            .split(',')
            .map(|j| j.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Instruction {
            id,
            lights,
            buttons,
            joltages,
        })
    }

    fn part1(&self) -> usize {
        #[derive(Clone, Debug)]
        struct Step {
            moves: usize,
            state: Vec<LightStatus>,
            sequence: Vec<usize>,
        }

        let mut queue = VecDeque::from([Step{moves: 0, state: vec![LightStatus::Off; self.lights.len()], sequence: vec!()}]);
        loop {
            // Queue can never be empty, unless there's no button to press!
            let step = queue.pop_front().unwrap();
            // We skip the buttons lower than the last one pushed, because the same sequence in a
            // different order produces the same result.
            for b in self.buttons.iter().skip(*step.sequence.last().unwrap_or(&0)) {
                let mut new_step = step.clone();
                new_step.moves += 1;
                new_step.sequence.push(b.id);
                for l in b.lights.iter() {
                    new_step.state[*l] = new_step.state[*l].toggle();
                }
                if new_step.state == self.lights {
                    return new_step.moves;
                }
                queue.push_back(new_step);
            }
        }
    }

    #[allow(unused)]
    fn part2_slow(&self) -> usize {
        #[derive(Clone, Debug)]
        struct Step {
            moves: usize,
            state: Vec<u64>,
            sequence: Vec<usize>,
        }

        // TODO: this is too slow! We need to optimize. Ideas:
        //  - identify if one joltage is controlled only by one button => this button must be
        //  pressed this many times
        //    => this is not the case in the first few machines
        //  - identify if a light is always paired with another (e.g. 5 is always paired with 3 in
        //  the first machine)
        //    => how does this help though?
        //  - identify if a light is always paired with another except for one button (e.g. 5 is
        //  always paired with 2 in the first machine except for the first button)
        //    => is this helpful alone? or only in combination with the previous fact?
        let mut queue = VecDeque::from([Step{moves: 0, state: vec![0; self.joltages.len()], sequence: vec!()}]);
        loop {
            // Queue can never be empty, unless there's no button to press!
            let step = queue.pop_front().unwrap();
            // We skip the buttons lower than the last one pushed, because the same sequence in a
            // different order produces the same result.
            for b in self.buttons.iter().skip(*step.sequence.last().unwrap_or(&0)) {
                let mut new_step = step.clone();
                new_step.moves += 1;
                new_step.sequence.push(b.id);
                for l in b.lights.iter() {
                    new_step.state[*l] = new_step.state[*l] + 1;
                }
                if new_step.state == self.joltages {
                    println!("[{}] Solved in {} moves: {:?}", self.id, new_step.moves, new_step.sequence);
                    return new_step.moves;
                }
                // If one of the joltages is higher than the target, this is a dead end.
                if new_step.state.iter().zip(self.joltages.iter()).all(|(current, want)| current <= want) {
                    queue.push_back(new_step);
                }
            }
        }
    }

    // This doesn't work because I couldn't manage to find an easy way to keep only the independent
    // columns in the matrix.
    #[allow(unused)]
    fn part2_mathru(&self) -> usize {
        /*
           [#####.###] (4,6) (0,5,6,8) (0,1,3,5,6,8) (0,1,2,3,4,5,7,8) (2,3) (1,2,3,4,6,7) (0,2,5,6,8) (2,3,4,5) (0,1,2,3,5,6,8) {168,164,176,171,51,173,194,30,168}

           (0 1 1 1 0 0 1 0 1  168)
           (0 0 1 1 0 1 0 0 1  164)
           (0 0 0 1 1 1 1 1 1  176)
           (0 0 1 1 1 1 0 1 1  171)
           (1 0 0 1 0 1 0 1 0   51)
           (0 1 1 1 0 0 1 1 1  173)
           (1 1 1 0 0 1 1 0 1  194)
           (0 0 0 1 0 1 0 0 0   30)
           (0 1 1 1 0 0 1 0 1  168)
       */
        let cols = self.buttons.len();
        let rows = self.joltages.len();
        assert!(rows <= cols);
        let cols = rows;
        let mut v = vec![0.; cols*rows];
        let mut skipped = 0;
        'outer: for (nb, b) in self.buttons.iter().enumerate() {
            if b.id - skipped >= cols {
                break;
            }
            let av: General<f64> = General::new(rows, cols, v.clone());
            println!("V: {av}");
            // Check if another column is equal to this one. If so, skip it.
            for c in 0..nb {
                println!("Comparing {:?} to {:?}", self.buttons[c].lights, b.lights);
                if self.buttons[c].lights == b.lights {
                    println!("Equals");
                    println!("Skipping column {nb} which is identical to column {c}");
                    skipped += 1;
                    continue 'outer;
                }
                println!("Not equals");
            }
            for l in b.lights.iter() {
                v[b.id + cols*l - skipped] = 1.;
            }
        }
        let av: General<f64> = General::new(rows, cols, v.clone());
        println!("V: {av}");
        println!("Skipped {skipped} columns");
        if self.buttons.len() - skipped < cols {
            println!("Not enough independent columns!");
        }
        let a: General<f64> = General::new(rows, cols, v);
        println!("Matrix: {a}");
        let b: Vector<f64> = Vector::new_column(self.joltages.iter().map(|j| *j as f64).collect::<Vec<_>>());
        println!("Solve {a:?}*x = {b:?}");

        // Solve a * x = b
        let x: Vector<f64> = a.solve(&b).unwrap();

        x.iter().sum::<f64>() as usize
    }

    // Manual gaussian pivot.
    // This doesn't work because it finds a solution, but not necessarily the best one.
    #[allow(unused)]
    fn part2_manual(&self) -> usize {
        /*
           [#####.###] (4,6) (0,5,6,8) (0,1,3,5,6,8) (0,1,2,3,4,5,7,8) (2,3) (1,2,3,4,6,7) (0,2,5,6,8) (2,3,4,5) (0,1,2,3,5,6,8) {168,164,176,171,51,173,194,30,168}

           (0 1 1 1 0 0 1 0 1  168)
           (0 0 1 1 0 1 0 0 1  164)
           (0 0 0 1 1 1 1 1 1  176)
           (0 0 1 1 1 1 0 1 1  171)
           (1 0 0 1 0 1 0 1 0   51)
           (0 1 1 1 0 0 1 1 1  173)
           (1 1 1 0 0 1 1 0 1  194)
           (0 0 0 1 0 1 0 0 0   30)
           (0 1 1 1 0 0 1 0 1  168)
       */
        /*
           [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}

           (1 0 1 1 0   7)
           (0 0 0 1 1   5)
           (1 1 0 1 1  12)
           (1 1 0 0 1   7)
           (1 0 1 0 1   2)
        */
        fn pretty_print(v: &Vec<Vec<isize>>) {
            for line in v.iter() {
                println!("{line:?}");
            }
            println!("");
        }

        let cols = self.buttons.len() + 1;
        let rows = self.joltages.len();
        let mut v = (0..rows).map(|_| vec![0 as isize; cols]).collect::<Vec<_>>();
        for (nb, b) in self.buttons.iter().enumerate() {
            for l in b.lights.iter() {
                v[*l][b.id] = 1;
            }
        }
        for (nj, j) in self.joltages.iter().enumerate() {
            v[nj][cols-1] = *j as isize
        }
        //println!("v: {v:#?}");

        let size = std::cmp::min(cols-1, rows);

        // Forward pass: get 1s on the diagonal and 0s below.
        for p in 0..size {
            println!("p = {p}");
            pretty_print(&v);
            let mut found = false;
            for k in p..rows {
                if v[k][p] != 0 {
                    let tmp = v[p].clone();
                    v[p] = v[k].clone();
                    v[k] = tmp;
                    let n = v[p][p];
                    for k in 0..cols {
                        v[p][k] = v[p][k] / n;
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                // If a joltage/button is not found, we can just assume 0 for it.
                // Alternatively, we could exchange columns.
                v[p][p] = 1;
            }
            for k in p+1..rows {
                let m = v[k][p];
                println!("Removing {m} times line {p} from line {k}");
                for c in 0..cols {
                    v[k][c] -= m*v[p][c];
                }
            }
        }

        // Backward pass: get 0s above the diagonal
        for p in 0..size {
            for j in p+1..size {
                let n = v[p][j];
                println!("Removing {n} times line {j} from line {p}");
                for k in j..=size {
                    v[p][k] -= n*v[j][k];
                }
                pretty_print(&v);
            }
        }

        pretty_print(&v);

        v.iter()
            .map(|l| l[size])
            .sum::<isize>() as usize
    }

    // Linear programming: solving the (potentially) underconstrained system while minimizing the
    // sum of the variables.
    fn part2(&self) -> usize {
        // Minimize an objective function sum(variables)
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let mut variables = vec!();
        for _ in self.buttons.iter() {
            variables.push(problem.add_integer_var(1., (0, i32::MAX)));
        }

        for (nj, j) in self.joltages.iter().enumerate() {
            let mut coeffs = vec!();
            for (nb, b) in self.buttons.iter().enumerate() {
                for l in b.lights.iter() {
                    if *l == nj {
                        coeffs.push((variables[nb], 1.));
                        break;
                    }
                }
            }
            problem.add_constraint(&coeffs, ComparisonOp::Eq, *j as f64);
        }

        let solution = problem.solve().unwrap();
        solution.objective().round() as usize
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Manual {
    instructions: Vec<Instruction>,
}

impl Manual {
    fn parse(repr: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Manual {
            instructions: repr.split('\n')
                .filter(|line| !line.is_empty())
                .enumerate()
                .map(|(id, line)| Instruction::parse(id, line))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn part1(&self) -> usize {
        self.instructions.iter().map(|instr| instr.part1()).sum()
    }

    fn part2(&self) -> usize {
        self.instructions.iter().map(|instr| instr.part2()).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day10Opts = argh::from_env();

    let manual = Manual::load(opts.filename.as_str())?;
    println!("Part 1: {}", manual.part1());
    println!("Part 2: {}", manual.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_light() {
        assert_eq!(LightStatus::On, LightStatus::parse('#').unwrap());
        assert_eq!(LightStatus::Off, LightStatus::parse('.').unwrap());
        assert!(LightStatus::parse('?').is_err());
    }

    #[test]
    fn test_parse_instruction() {
        let want = Instruction {
            id: 0,
            lights: vec![
                        LightStatus::Off,
                        LightStatus::On,
                        LightStatus::On,
                        LightStatus::Off
            ],
            buttons: vec![
                Button::new(0, vec![3]),
                Button::new(1, vec![1, 3]),
                Button::new(2, vec![2]),
                Button::new(3, vec![2, 3]),
                Button::new(4, vec![0, 2]),
                Button::new(5, vec![0, 1]),
            ],
            joltages: vec![3, 5, 4, 7],
        };

        assert_eq!(want, Instruction::parse(0, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap());
    }

    #[test]
    fn test_parse_manual() {
        let want = Manual {
            instructions: vec![
              Instruction {
                  id: 0,
                  lights: vec![ LightStatus::Off, LightStatus::On, LightStatus::On ],
                  buttons: vec![
                      Button::new(0, vec![3]),
                      Button::new(1, vec![1, 3]),
                      Button::new(2, vec![2]),
                  ],
                  joltages: vec![3, 5, 4, 7],
              },
              Instruction {
                  id: 1,
                  lights: vec![ LightStatus::Off, LightStatus::Off, LightStatus::On, LightStatus::Off ],
                  buttons: vec![
                      Button::new(0, vec![0, 2, 3, 4]),
                      Button::new(1, vec![2, 3]),
                      Button::new(2, vec![0, 4]),
                  ],
                  joltages: vec![7, 5, 12, 7, 2],
              },
            ],
        };

        assert_eq!(want, Manual::parse("[.##] (3) (1,3) (2) {3,5,4,7}\n[..#.] (0,2,3,4) (2,3) (0,4) {7,5,12,7,2}").unwrap());
        assert_eq!(want, Manual::parse("[.##] (3) (1,3) (2) {3,5,4,7}\n[..#.] (0,2,3,4) (2,3) (0,4) {7,5,12,7,2}\n").unwrap());
    }

    #[test]
    fn test_part1_machine1() {
        let machine = Instruction::parse(0, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(2, machine.part1());
    }

    #[test]
    fn test_part1_machine2() {
        let machine = Instruction::parse(0, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}").unwrap();
        assert_eq!(3, machine.part1());
    }

    #[test]
    fn test_part1_machine3() {
        let machine = Instruction::parse(0, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}").unwrap();
        assert_eq!(2, machine.part1());
    }

    #[test]
    fn test_part1() {
        let manual = Manual::load("sample.txt").unwrap();
        assert_eq!(7, manual.part1());
    }

    #[test]
    fn test_part1_full() {
        let manual = Manual::load("my_input.txt").unwrap();
        assert_eq!(385, manual.part1());
    }

    #[test]
    fn test_part2_machine1() {
        let machine = Instruction::parse(0, "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}").unwrap();
        assert_eq!(10, machine.part2());
    }

    #[test]
    fn test_part2_machine2() {
        let machine = Instruction::parse(0, "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}").unwrap();
        assert_eq!(12, machine.part2());
    }

    #[test]
    fn test_part2_machine3() {
        let machine = Instruction::parse(0, "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}").unwrap();
        assert_eq!(11, machine.part2());
    }

    #[test]
    fn test_part2() {
        let manual = Manual::load("sample.txt").unwrap();
        assert_eq!(33, manual.part2());
    }

    #[test]
    fn test_solve_sample1() {
        use mathru::vector;
        use mathru::algebra::linear::{vector::Vector, matrix::General};
        use mathru::algebra::linear::matrix::{Solve};

        /*
           [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

           (0 0 0 0 1 1 3)
           (0 1 0 0 0 1 5)
           (0 0 1 1 1 0 4)
           (1 1 0 1 0 0 7)
       */
        let a: General<f64> = General::new(7, 4, vec![
           0., 0., 0., 0., 1., 1.,
           0., 1., 0., 0., 0., 1.,
           0., 0., 1., 1., 1., 0.,
           1., 1., 0., 1., 0., 0.,
        ]);
        let a: General<f64> = General::new(4, 4, vec![
           0., 0., 0., 0.,
           0., 1., 0., 0.,
           0., 0., 1., 1.,
           1., 1., 0., 1.,
        ]);
        println!("Matrix: {a}");

        let b: Vector<f64> = vector![3.; 5.; 4.; 7.];

        // Solve a * x = b
        let x: Vector<f64> = a.solve(&b).unwrap();

        println!("x = {x:?}");
        assert_eq!(x.iter().sum::<f64>(), 10.);
    }

    #[test]
    fn test_solve_1() {
        use mathru::vector;
        use mathru::algebra::linear::{vector::Vector, matrix::General};
        use mathru::algebra::linear::matrix::{Solve};

        /*
           [##...#] (1,3,4,5) (2,3,5) (0,2,3) (0,2,3,4,5) (1,2,4) (0,1,2,3) {24,27,40,30,25,17}

           (0 0 1 1 0 1) 24
           (1 0 0 0 1 1) 27
           (0 1 1 1 1 1) 40
           (1 1 1 1 0 1) 30
           (1 0 0 1 1 0) 25
           (1 1 0 1 0 0) 17
       */
        let a: General<f64> = General::new(6, 6, vec![
           0., 1., 0., 1., 1., 1.,
           0., 0., 1., 1., 0., 1.,
           1., 0., 1., 1., 0., 0.,
           1., 0., 1., 1., 1., 1.,
           0., 1., 1., 0., 1., 0.,
           1., 1., 1., 1., 0., 0.,
        ]);
        let b: Vector<f64> = vector![24.; 27.; 40.; 30.; 25.; 17.];

        // Solve a * x = b
        let x: Vector<f64> = a.solve(&b).unwrap();

        println!("x = {x:?}");
        assert_eq!(x.iter().sum::<f64>(), 42.);
    }

    #[test]
    fn test_solve_3() {
        use mathru::vector;
        use mathru::algebra::linear::{vector::Vector, matrix::General};
        use mathru::algebra::linear::matrix::{Solve};

        /*
           [#####.###] (4,6) (0,5,6,8) (0,1,3,5,6,8) (0,1,2,3,4,5,7,8) (2,3) (1,2,3,4,6,7) (0,2,5,6,8) (2,3,4,5) (0,1,2,3,5,6,8) {168,164,176,171,51,173,194,30,168}

           (0 1 1 1 0 0 1 0 1  168)
           (0 0 1 1 0 1 0 0 1  164)
           (0 0 0 1 1 1 1 1 1  176)
           (0 0 1 1 1 1 0 1 1  171)
           (1 0 0 1 0 1 0 1 0   51)
           (0 1 1 1 0 0 1 1 1  173)
           (1 1 1 0 0 1 1 0 1  194)
           (0 0 0 1 0 1 0 0 0   30)
           (0 1 1 1 0 0 1 0 1  168)
       */
        let a: General<f64> = General::new(9, 9, vec![
           0., 0., 0., 0., 1., 0., 1., 0., 0.,
           1., 0., 0., 0., 0., 1., 1., 0., 1.,
           1., 1., 0., 1., 0., 1., 1., 0., 1.,
           1., 1., 1., 1., 1., 1., 0., 1., 1.,
           0., 0., 1., 1., 0., 0., 0., 0., 0.,
           0., 1., 1., 1., 1., 0., 1., 1., 0.,
           1., 0., 1., 0., 0., 1., 1., 0., 1.,
           0., 0., 1., 1., 1., 1., 0., 0., 0.,
           1., 1., 1., 1., 0., 1., 1., 0., 1.,
        ]);
        let b: Vector<f64> = vector![168.; 164.; 176.; 171.; 51.; 173.; 194.; 30.; 168.];

        // Solve a * x = b
        let x: Vector<f64> = a.solve(&b).unwrap();

        println!("x = {x:?}");
        assert_eq!(x.iter().sum::<f64>(), 211.);
    }
}
