use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    //let window_size = 4; // Part 1
    let window_size = 14; // Part 2
    for line in lines {
        let line = line?;
        let chars = line.chars().collect::<Vec<_>>();
        let tokens = chars.windows(window_size);
        for (i, w) in tokens.enumerate() {
            let mut token = w.to_vec();
            token.sort();
            let unique_token = token.into_iter().unique().collect::<String>();
            if unique_token.len() == window_size {
                println!("Pattern {:?} ends at {:?}", w, i+window_size);
                break;
            }
        }
    }

    Ok(())
}
