use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn element_priority(c: char) -> i64 {
		let mut priority = 0;
		let e = c as i64;
		if e >= 'a' as i64 && e <= 'z' as i64 {
				priority += e - 'a' as i64 + 1;
		}
		if e >= 'A' as i64 && e <= 'Z' as i64 {
				priority += e - 'A' as i64 + 27;
		}
		priority
}

fn main() -> Result<(), Box<dyn Error>>  {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

		// Part 1
		{
				let file = File::open(filename)?;
				let lines = io::BufReader::new(file).lines();

				let mut total_priority = 0;
				for l in lines {
						let l = l?;
						if l.is_empty() {
								continue;
						}
						//println!("Line: {}", l);
						let mid = l.len()/2;
						let compartment1 = &l[..mid].to_string();
						let compartment2 = &l[mid..].to_string();
						//println!("First compartment: {} - Second compartment: {}", compartment1, compartment2);

						let compartment1 = compartment1.chars().collect::<HashSet<_>>();
						let compartment2 = compartment2.chars().collect::<HashSet<_>>();

						//println!("Common items: {}", compartment1.intersection(&compartment2).collect::<String>());

						for c in compartment1.intersection(&compartment2) {
								let priority = element_priority(*c);
								//println!("Priority for {}: {}", c, priority);
								total_priority += priority;
						}
				}
				println!("Part 1: Total priority: {}", total_priority);
		}

		// Part 2
		{
				let file = File::open(filename)?;
				let mut lines = io::BufReader::new(file).lines();
				let mut total_priority = 0;
				loop {
						let l1 = lines.next();
						if let None = l1 {
								break;
						}
						let l1 = l1.unwrap().unwrap();
						let l2 = lines.next().unwrap().unwrap();
						let l3 = lines.next().unwrap().unwrap();

						let l1 = l1.chars().collect::<HashSet<_>>();
						let l2 = l2.chars().collect::<HashSet<_>>();
						let l3 = l3.chars().collect::<HashSet<_>>();

						let i1 = l1.intersection(&l2).map(|x| *x).collect::<HashSet<_>>();
						for c in l3.intersection(&i1) {
								let priority = element_priority(*c);
								//println!("Priority for {}: {}", c, priority);
								total_priority += priority;
						}
				}
				println!("Part 2: Total priority: {}", total_priority);
		}

		Ok(())
}
