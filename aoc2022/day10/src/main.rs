use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Cpu {
    x: i64,
    cycles: i64,
    signal_strength: i64,
    crt_lines: Vec<String>,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu{x: 1, cycles: 0, signal_strength: 0, crt_lines: vec!()}
    }

    fn next_cycle(&mut self) {
        let pos = self.cycles % 40;
        if pos == 0 {
            self.crt_lines.push(String::new());
        }
        let c = if (self.x-pos).abs() <= 1 { '\u{2588}' } else { ' ' };
        self.crt_lines.last_mut().unwrap().push(c);
        self.cycles += 1;
        if self.cycles % 40 == 20 {
            //println!("At cycle {}, X={}, adding {}", self.cycles, self.x, self.cycles*self.x);
            self.signal_strength += self.cycles * self.x;
        }
    }

    fn addx(&mut self, param: i64) {
        self.x += param;
    }

    fn print_screen(&self) {
        for l in self.crt_lines.iter() {
            println!("{}", l);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>>  {
    let filename = "sample.txt";
    //let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut cpu = Cpu::new();

    for l in lines {
        let l = l?;
        let mut parts = l.split(' ');
        let instr = parts.next().unwrap();
        match instr {
            "noop" => {
                cpu.next_cycle();
            },
            "addx" => {
                cpu.next_cycle();
                let param = parts.next().unwrap().parse::<i64>()?;
                cpu.next_cycle();
                cpu.addx(param);
            },
            _ => {
                println!("ERROR: Unsupported instruction {}", instr);
            }
        }
    }

    println!("");
    println!("Signal strength: {}", cpu.signal_strength);
    println!("");
    cpu.print_screen();

    Ok(())
}
