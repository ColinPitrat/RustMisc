use derive_more::Display;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::{Peekable, zip};
use std::str::Chars;

#[derive(Debug, Display, Clone)]
struct ParseError(String);

impl Error for ParseError {}

#[derive(Clone, Debug)]
enum Chunk {
    Integer { val: i64 },
    List { vals: Vec<Chunk> },
}

impl Chunk {
    fn parse_int(chars: &mut Peekable<Chars>) -> Result<Chunk, Box<dyn Error>> {
        let mut int_string = String::new();
        loop {
            let ch = chars.peek().unwrap();
            //println!("Consume char 4: {}", ch);
            match ch {
                '0'..='9' => {
                    int_string.push(*ch);
                    chars.next();
                },
                ',' | ']' => break,
                _ => return Err(Box::new(ParseError(format!("Unexpected char in integer: '{}'", ch)))),
            }
        }
        if int_string.is_empty() {
            return Err(Box::new(ParseError("Empty integer'".to_string())));
        }
        Ok(Chunk::Integer{val: int_string.parse::<i64>()?})
    }

    fn parse_list(chars: &mut Peekable<Chars>) -> Result<Chunk, Box<dyn Error>> {
        let mut vals = vec!();
        let ch = chars.next().unwrap();
        //println!("Consume char 1: {}", ch);
        if ch != '[' {
            return Err(Box::new(ParseError(format!("List should start with '[', got '{}'", ch))));
        }
        loop {
            let ch = chars.peek().unwrap();
            //println!("Read char 2: {}", ch);
            match ch {
                ']' => {
                    chars.next();
                    break;
                },
                '[' => vals.push(Self::parse_list(chars)?),
                '0'..='9' => vals.push(Self::parse_int(chars)?),
                _ => return Err(Box::new(ParseError(format!("Unexpected element starting with '{}'", ch)))),
            }
            let ch = chars.next().unwrap();
            //println!("Consume char 3: {}", ch);
            if ch == ']' {
                break;
            }
            if ch != ',' {
                return Err(Box::new(ParseError(format!("Elements should be separated by ',', got '{}'", ch))));
            }
        }
        Ok(Chunk::List{vals})
    }

    fn to_string(&self) -> String {
        match self {
            Chunk::List{vals} => {
                String::from("[") + vals.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(",").as_str() + "]"
            },
            Chunk::Integer{val} => {
                format!("{}", val)
            }
        }
    }

    fn cmp(&self, other: &Chunk) -> Ordering {
        match (self, other) {
            (Chunk::List{vals}, Chunk::List{vals: other_vals}) => {
                for (left, right) in zip(vals, other_vals) {
                    match left.cmp(right) {
                        Less => return Less,
                        Equal => {},
                        Greater => return Greater,
                    };
                }
                // If we get out of the loop, the lists were identicals up to the end of the
                // shortest. Compare length.
                vals.len().cmp(&other_vals.len())
            },
            (Chunk::Integer{val}, Chunk::Integer{val: other_val}) => {
                val.cmp(other_val)
            }
            (Chunk::List{..}, Chunk::Integer{..}) => {
                self.cmp(&Chunk::List{vals: vec!(other.clone())})
            }
            (Chunk::Integer{..}, Chunk::List{..}) => {
                Chunk::List{vals: vec!(self.clone())}.cmp(other)
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    chunk: Chunk
}

impl PartialEq for Packet {
    fn eq(&self, other: &Packet) -> bool {
        self.chunk.cmp(&other.chunk).is_eq()
    }

    fn ne(&self, other: &Packet) -> bool {
        self.chunk.cmp(&other.chunk).is_ne()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Packet) -> Option<Ordering> {
        Some(self.chunk.cmp(&other.chunk))
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Packet) -> Ordering {
        self.chunk.cmp(&other.chunk)
    }
}

impl Packet {
    fn parse(s: &str) -> Result<Packet, Box<dyn Error>> {
        let chunk = Chunk::parse_list(&mut s.chars().peekable())?;
        Ok(Packet{chunk})
    }

    fn to_string(&self) -> String {
        self.chunk.to_string()
    }

    fn comes_before(&self, other: &Packet) -> bool {
        match self.chunk.cmp(&other.chunk) {
            Less => true,
            Equal => true, // Not clear from the instructions if it should be true or false, but false doesn't seem to make sense.
            Greater => false,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let mut correct_indices_sum = 0;
    let mut i = 1;
    let mut all_packets = vec!();
    loop {
        let line = lines.next().unwrap()?;
        let p1 = Packet::parse(&line)?;
        let line = lines.next().unwrap()?;
        let p2 = Packet::parse(&line)?;
        all_packets.push(p1.clone());
        all_packets.push(p2.clone());

        //println!("Packets: {:?} and {:?}", p1, p2);
        println!("Packets: {} and {}", p1.to_string(), p2.to_string());
        println!("Order good? {}", p1.comes_before(&p2));

        if p1.comes_before(&p2) {
            correct_indices_sum += i;
        }

        if lines.next().is_none() {
            break;
        }
        i += 1;
    }

    // Part 1
    println!("Sum of correct indices: {}", correct_indices_sum);

    // Part 2
    let marker1 = Packet::parse("[[2]]")?;
    let marker2 = Packet::parse("[[6]]")?;
    all_packets.push(marker1.clone());
    all_packets.push(marker2.clone());

    all_packets.sort();

    for p in all_packets.iter() {
        println!("{}", p.to_string());
    }

    let idx1 = all_packets.iter().position(|p| p.eq(&marker1)).unwrap() + 1;
    let idx2 = all_packets.iter().position(|p| p.eq(&marker2)).unwrap() + 1;

    println!("Decoder key: {}", idx1 * idx2);

    Ok(())
}
