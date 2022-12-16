use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(PartialEq)]
enum CrateMoverModel {
    CM9000,
    CM9001,
}

fn main() -> Result<(), Box<dyn Error>> {
    //let filename = "sample.txt";
    let filename = "my_input.txt";
    //let model = CrateMoverModel::CM9000;
    let model = CrateMoverModel::CM9001;

		let file = File::open(filename)?;
		let mut lines = io::BufReader::new(file).lines();

		// Read schema
		let mut crates_piles = vec!();
		loop {
			let line = lines.next().unwrap()?;
			if line.is_empty() {
					break;
			}

			// Read one layer of piles
			let crates = line.chars()
					.collect::<Vec<char>>()
					.chunks(4)
					.map(|c| c.iter().collect::<String>())
					.collect::<Vec<String>>();

			// We know how many piles we have, create them
			if crates_piles.is_empty() {
				for _ in &crates {
						crates_piles.push(vec!());
				}
			}
			// Push on the pile
			for (i, e) in crates.into_iter().enumerate() {
					let mut e = e.clone();
					e.retain(|c| c != ' ' && c != '[' && c != ']');
					if !e.is_empty() {
							crates_piles[i].push(e);
					}
			}
		}
		// Remove the piles numbers and put them with top at the end
		for pile in &mut crates_piles {
				pile.pop();
				pile.reverse();
		}

		println!("Crates piles: {:?}", crates_piles);

		// Read and execute instructions
		for instruction in lines {
				let instruction = instruction?;
				println!("Instruction: {}", instruction);
				let mut pieces = instruction.split(' ');
				let _ = pieces.next();
				let count = pieces.next().unwrap().parse::<usize>()?;
				let _ = pieces.next();
				let from = pieces.next().unwrap().parse::<usize>()? - 1;
				let _ = pieces.next();
				let to = pieces.next().unwrap().parse::<usize>()? - 1;

				println!("Move {} from {} to {}", count, from, to);
        if model == CrateMoverModel::CM9000 {
        // Part 1: CrateMover 9000
				for _ in 0..count {
						let item = crates_piles[from].pop().unwrap();
						crates_piles[to].push(item);
				}
        } else {
            // Part 2: CrateMover 9001
            let mut stack = vec!();
            for _ in 0..count {
                let item = crates_piles[from].pop().unwrap();
                stack.push(item);
            }
            stack.reverse();
            for item in stack {
                crates_piles[to].push(item);
            }
        }

				println!("Crates piles: {:?}", crates_piles);
		}

		println!("Crates piles: {:?}", crates_piles);

		let result = crates_piles.into_iter().map(|mut e| e.pop().unwrap()).collect::<Vec<_>>().join("");
		println!("Result: {}", result);

    Ok(())
}
