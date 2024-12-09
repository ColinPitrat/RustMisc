use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 9 of Advent of Code 2024.
struct Day9Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day9Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day9Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day9Opts {
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
        if Day9Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
enum BlockType {
    Free,
    File(usize),
}

#[derive(Clone,Debug)]
struct Block {
    kind: BlockType,
    length: usize,
}

#[derive(Clone,Debug)]
struct DiscMap {
    blocks: Vec<Block>,
}

impl fmt::Display for DiscMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in &self.blocks {
            match b.kind {
                BlockType::Free => write!(f, "{} ", std::iter::repeat('.').take(b.length).collect::<String>())?,
                BlockType::File(id) => write!(f, "{} ", std::iter::repeat(format!("{id},")).take(b.length).collect::<String>())?,
            }
        }
        Ok(())
    }
}

impl DiscMap {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut blocks = vec!();
        let mut id = 0;
        let mut file = true;
        for c in content.chars() {
            if !c.is_digit(10) {
                continue;
            }
            let length = c.to_string().parse::<usize>()?;
            let kind = if file {
                BlockType::File(id)
            } else {
                BlockType::Free
            };
            if file {
                id += 1;
            }
            blocks.push(Block{kind, length});
            file = !file;
        }
        Ok(DiscMap{blocks})
    }

    fn find_first_free_block(&self) -> Option<usize> {
        for (i, b) in self.blocks.iter().enumerate() {
            if b.length == 0 {
                continue
            }
            match b.kind {
                BlockType::Free => return Some(i),
                BlockType::File(_) => continue,
            }
        }
        None
    }

    fn find_first_free_block_large_enough(&self, length: usize) -> Option<usize> {
        for (i, b) in self.blocks.iter().enumerate() {
            if b.length < length {
                continue
            }
            match b.kind {
                BlockType::Free => return Some(i),
                BlockType::File(_) => continue,
            }
        }
        None
    }

    fn find_last_file_block(&self) -> Option<usize> {
        for (i, b) in self.blocks.iter().enumerate().rev() {
            if b.length == 0 {
                continue
            }
            match b.kind {
                BlockType::Free => continue,
                BlockType::File(_) => return Some(i),
            }
        }
        None
    }

    fn find_file_block(&self, want_id: usize) -> Option<usize> {
        for (i, b) in self.blocks.iter().enumerate().rev() {
            match b.kind {
                BlockType::Free => continue,
                BlockType::File(id) => if id == want_id { return Some(i) },
            }
        }
        None
    }

    fn compress1(&mut self) {
        loop {
            let first_free_block = self.find_first_free_block().unwrap();
            let last_file_block = self.find_last_file_block().unwrap();
            if last_file_block <= first_free_block {
                break;
            }
            let ffb = self.blocks[first_free_block].clone();
            let lfb = self.blocks[last_file_block].clone();
            if ffb.length > lfb.length {
                // The free block is bigger: we need to split it in two.
                // This means reducing the existing free block to the size of the file block,
                // switching it's kind and inserting a free block after that of the remaining size.
                let remaining_length = ffb.length - lfb.length;
                self.blocks[first_free_block].length = lfb.length;
                self.blocks[first_free_block].kind = lfb.kind.clone();
                self.blocks[last_file_block].kind = BlockType::Free;
                self.blocks.insert(first_free_block+1, Block{kind: BlockType::Free, length: remaining_length});
            } else if ffb.length < lfb.length {
                // The file block is bigger: we need to split it in two.
                // This means changing the free block to file block and splitting the existing file
                // block in the remaining size followed by a free block. 
                let remaining_length = lfb.length - ffb.length;
                self.blocks[first_free_block].kind = lfb.kind.clone();
                self.blocks[last_file_block].length = remaining_length;
                self.blocks.insert(last_file_block+1, Block{kind: BlockType::Free, length: ffb.length});
            } else {
                // The two blocks are the same size, we can just switch their kind.
                self.blocks[first_free_block].kind = self.blocks[last_file_block].kind.clone();
                self.blocks[last_file_block].kind = BlockType::Free;
            }
        }
    }

    fn compress2(&mut self) {
        let last_file_block = self.find_last_file_block().unwrap();
        let lfb = self.blocks[last_file_block].clone();
        if let BlockType::File(mut current_file_id) = lfb.kind {
            while current_file_id > 0 {
                let last_file_block = self.find_file_block(current_file_id).unwrap();
                let lfb = self.blocks[last_file_block].clone();
                let first_free_block = self.find_first_free_block_large_enough(lfb.length);
                if let Some(first_free_block) = first_free_block {
                    if last_file_block > first_free_block {
                        let ffb = self.blocks[first_free_block].clone();
                        if ffb.length > lfb.length {
                            // The free block is bigger: we need to split it in two.
                            // This means reducing the existing free block to the size of the file block,
                            // switching it's kind and inserting a free block after that of the remaining size.
                            let remaining_length = ffb.length - lfb.length;
                            self.blocks[first_free_block].length = lfb.length;
                            self.blocks[first_free_block].kind = lfb.kind.clone();
                            self.blocks[last_file_block].kind = BlockType::Free;
                            self.blocks.insert(first_free_block+1, Block{kind: BlockType::Free, length: remaining_length});
                        } else if ffb.length < lfb.length {
                            println!("WTF? This shouldn't happen!");
                        } else {
                            // The two blocks are the same size, we can just switch their kind.
                            self.blocks[first_free_block].kind = self.blocks[last_file_block].kind.clone();
                            self.blocks[last_file_block].kind = BlockType::Free;
                        }
                    }
                }
                log_verbose!("       After moving {}: {}", current_file_id, self);
                current_file_id -= 1;
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut result = 0;
        let mut idx = 0;
        for b in self.blocks.iter() {
            for _ in 0..b.length {
                if let BlockType::File(id) = b.kind {
                    result += idx * id;
                }
                idx += 1;
            }
        }
        result
    }
}

fn part1(mut map: DiscMap) -> usize {
    log_verbose!("Part 1 starting map  : {}", map);
    map.compress1();
    log_verbose!("Part 1 compressed map: {}", map);
    map.checksum()
}

fn part2(mut map: DiscMap) -> usize {
    log_verbose!("Part 2 starting map  : {}", map);
    map.compress2();
    log_verbose!("Part 2 compressed map: {}", map);
    map.checksum()
}

fn main() -> Result<(), Box<dyn Error>> {
    Day9Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day9Opts::get_opts().filename.as_str())?;
    let map = DiscMap::read(content.as_str())?;

    println!("Part 1: {}", part1(map.clone()));
    println!("Part 2: {}", part2(map.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = DiscMap::read(content.as_str()).unwrap();

        assert_eq!(1928, part1(map.clone()));
        assert_eq!(2858, part2(map.clone()));
    }
}
