use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Range {
    left: i64,
    right: i64,
}

impl Range {
    fn parse(range: &str) -> Result<Range, Box<dyn Error>> {
        let mut bounds = range.split('-');
        let left = bounds.next().unwrap().to_string().parse::<i64>()?;
        let right = bounds.next().unwrap().to_string().parse::<i64>()?;
        Ok(Range { left, right })
    }


    fn contains(&self, other: &Range) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.left <= other.right && other.left <= self.right 
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let filename = "sample.txt";
    let filename = "my_input.txt";
    
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut contained = 0;
    let mut overlaps = 0;
    for l in lines {
        if let Ok(l) = l {
            let mut ranges = l.split(',');
            let r1 = Range::parse(ranges.next().unwrap())?;
            let r2 = Range::parse(ranges.next().unwrap())?;

            println!("Range 1: {:?} - Range 2: {:?}", r1, r2);
            if r1.contains(&r2) || r2.contains(&r1) {
                println!("Contained");
                contained += 1;
            }
            if r1.overlaps(&r2) {
                println!("Overlap");
                overlaps += 1;
            }
        }
    }

    println!("Contained ranges: {}", contained);
    println!("Overlapping ranges: {}", overlaps);

    Ok(())
}
