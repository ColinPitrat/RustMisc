use derive_more::Display;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Peekable;

#[derive(Debug, Display, Clone)]
struct NotAMonkeyError(String);

impl Error for NotAMonkeyError {}

#[derive(Debug, Default, PartialEq)]
enum Operation {
		#[default] Add,
		Substract,
		Multiply,
		Divide,
		Square,
}

impl TryFrom<&str> for Operation {
		type Error = String;

		fn try_from(s: &str) -> Result<Self, Self::Error> {
				match s {
						"+" => Ok(Operation::Add),
						"-" => Ok(Operation::Substract),
						"*" => Ok(Operation::Multiply),
						"/" => Ok(Operation::Divide),
						_ =>  Err(format!("Unsupported operation '{}'", s)),
				}
		}
}

#[derive(Debug, Default)]
struct Monkey {
    id: i64,
		items: Vec<i64>,
    operation: Operation,
		operand: i64,
	  divisor: i64,
    true_target: usize,
		false_target: usize,
		nb_inspected: i64,
}

impl Monkey {
		fn parse(lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<Monkey, Box<dyn Error>> {
				let mut m = Monkey{..Default::default()};
				for l in lines {
						let l = l?;
						if l.is_empty() {
								break;
						}
						if l.starts_with("Monkey") {
								let tokens = l.split(' ').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								m.id = tokens[1].split(':').collect::<Vec<_>>()[0].parse::<i64>()?;
						}
						if l.starts_with("  Starting items: ") {
								let tokens = l.split(':').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								let items = tokens[1].split(',').collect::<Vec<_>>().iter().map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
								m.items = items;
						}
						if l.starts_with("  Operation: new = old ") {
								let mut tokens = l.split(' ').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								let operand = tokens.pop().unwrap();
								if operand == "old" {
										let operation = Operation::try_from(tokens.pop().unwrap())?;
										if operation != Operation::Multiply {
												panic!("Unsupported operation: {:?}", tokens);
										}
										m.operation = Operation::Square;
								} else {
										m.operand = operand.parse::<i64>()?;
										m.operation = Operation::try_from(tokens.pop().unwrap())?;
								}
						}
						if l.starts_with("  Test: divisible by ") {
								let mut tokens = l.split(' ').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								m.divisor = tokens.pop().unwrap().parse::<i64>()?;
						}
						if l.starts_with("    If true: throw to monkey ") {
								let mut tokens = l.split(' ').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								m.true_target = tokens.pop().unwrap().parse::<usize>()?;
						}
						if l.starts_with("    If false: throw to monkey ") {
								let mut tokens = l.split(' ').collect::<Vec<_>>();
								println!("Parse {:?}", tokens);
								m.false_target = tokens.pop().unwrap().parse::<usize>()?;
						}
				}
				Ok(m)
		}

		fn operate(&self, worry: i64) -> i64 {
				match self.operation {
					Operation::Add => worry + self.operand,
					Operation::Substract => worry - self.operand,
					Operation::Multiply => worry * self.operand,
					Operation::Divide => worry / self.operand,
					Operation::Square => worry * worry,
				}
		}
}

#[derive(Debug)]
struct Tribe {
		monkeys: Vec<Monkey>,
		worry_divider: i64,
		tribe_divisor: i64,
}

impl Tribe {
		fn parse(lines: &mut Peekable<Lines<BufReader<File>>>) -> Result<Tribe, Box<dyn Error>> {
				let mut monkeys = vec!();
				while lines.peek().is_some() {
						monkeys.push(Monkey::parse(lines)?);
				}
				let worry_divider = 1;
        // The tribe_divisor is an optimization to support part 2. What matters is divisibility by
        // monkey.divisor so the result won't change if we divide by product(monkey.divisor forall
        // monkey). This allows worry level to stay reasonably low and avoids overflow.
				let tribe_divisor = monkeys.iter().map(|m| m.divisor).fold(1, |a, b| a*b);
				Ok(Tribe{monkeys, worry_divider, tribe_divisor})
		}

		fn process(&mut self) {
				for i in 0..self.monkeys.len() {
						println!("Monkey {}", i);
						self.monkeys[i].items.reverse();
						while self.monkeys[i].items.len() > 0 {
							let item = self.monkeys[i].items.pop().unwrap();
							self.monkeys[i].nb_inspected += 1;
							println!("  Inspects worry level {}", item);
							let item = self.monkeys[i].operate(item);
							println!("    Updated to worry level {}", item);
							let item = item / self.worry_divider;
							let item = item % self.tribe_divisor;
							println!("    Divide by 3 to {}", item);
							let target = match (item % self.monkeys[i].divisor) == 0 {
									true => {
											println!("    Divisible by {}, sending to {}", self.monkeys[i].divisor, self.monkeys[i].true_target);
											self.monkeys[i].true_target
									},
									false => {
											println!("    Not divisible by {}, sending to {}", self.monkeys[i].divisor, self.monkeys[i].false_target);
											self.monkeys[i].false_target
									},
							};
							let receiver = &mut self.monkeys[target];
							receiver.items.push(item);
						}
				}
		}
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines().peekable();

		let mut tribe = Tribe::parse(&mut lines)?;

		println!("Monkeys: {:?}", tribe);

		//let nb_rounds = 20; // Part 1
		//tribe.worry_divider = 3; // Part 1

		let nb_rounds = 10000; // Part 2
		for round in 1..nb_rounds+1 {
				tribe.process();
				println!("After round {}: {:?}", round, tribe);
		}

		let mut inspected = tribe.monkeys.iter().map(|m| m.nb_inspected).collect::<Vec<_>>();
		inspected.sort();
		inspected.reverse();

		println!("Monkey business: {}", inspected[0]*inspected[1]);

    Ok(())
}
