use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::sequence::tuple;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Clone,Debug)]
enum Value {
    Immediate(i64),
    Addition(String, String),
    Substraction(String, String),
    Multiplication(String, String),
    Division(String, String),
}

impl Value {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
          Self::parse_immediate,
          Self::parse_addition,
          Self::parse_substraction,
          Self::parse_multiplication,
          Self::parse_division,
        ))(i)
    }

    fn parse_immediate(i: &str) -> IResult<&str, Self> {
        let (i, value) = nom::character::complete::i64(i)?;
        Ok((i, Value::Immediate(value)))
    }

    fn parse_addition(i: &str) -> IResult<&str, Self> {
        let (i, (v1, _, v2)) = tuple((
            alphanumeric1,
            tag(" + "),
            alphanumeric1,
        ))(i)?;
        Ok((i, Value::Addition(String::from(v1), String::from(v2))))
    }

    fn parse_substraction(i: &str) -> IResult<&str, Self> {
        let (i, (v1, _, v2)) = tuple((
            alphanumeric1,
            tag(" - "),
            alphanumeric1,
        ))(i)?;
        Ok((i, Value::Substraction(String::from(v1), String::from(v2))))
    }

    fn parse_multiplication(i: &str) -> IResult<&str, Self> {
        let (i, (v1, _, v2)) = tuple((
            alphanumeric1,
            tag(" * "),
            alphanumeric1,
        ))(i)?;
        Ok((i, Value::Multiplication(String::from(v1), String::from(v2))))
    }

    fn parse_division(i: &str) -> IResult<&str, Self> {
        let (i, (v1, _, v2)) = tuple((
            alphanumeric1,
            tag(" / "),
            alphanumeric1,
        ))(i)?;
        Ok((i, Value::Division(String::from(v1), String::from(v2))))
    }
}

#[derive(Debug)]
struct Monkey {
    name: String,
    yell: Value,
}

impl Monkey {
    fn parse(i: &str) -> IResult<&str, Self> {
        let (i, (name, _, yell)) = tuple((
            alphanumeric1,
            tag(": "),
            Value::parse,
        ))(i)?;
        Ok((i, Monkey{
            name: String::from(name),
            yell,
        }))
    }
}

#[derive(Debug)]
struct Tribe {
    monkeys: HashMap<String, Monkey>
}

impl Tribe {
    fn parse(lines: &mut Lines<BufReader<File>>) -> Result<Tribe, Box<dyn Error>> {
        let monkeys = lines
            .map(|l| Monkey::parse(l.unwrap().as_str()).unwrap().1)
            .map(|m| (m.name.clone(), m))
            .collect::<HashMap<String, Monkey>>();
        Ok(Tribe {
            monkeys,
        })
    }

    fn eval(&self, name: &str) -> i64 {
        match self.monkeys[name].yell.clone() {
            Value::Immediate(i) => i,
            Value::Addition(m1, m2) => self.eval(&m1) + self.eval(&m2),
            Value::Substraction(m1, m2) => self.eval(&m1) - self.eval(&m2),
            Value::Multiplication(m1, m2) => self.eval(&m1) * self.eval(&m2),
            Value::Division(m1, m2) => self.eval(&m1) / self.eval(&m2),
        }
    }

    fn eval_humn(&self, name: &str, humn: i64) -> i64 {
        if name == "humn" {
            return humn;
        }
        match self.monkeys[name].yell.clone() {
            Value::Immediate(i) => i,
            Value::Addition(m1, m2) => self.eval_humn(&m1, humn) + self.eval_humn(&m2, humn),
            Value::Substraction(m1, m2) => self.eval_humn(&m1, humn) - self.eval_humn(&m2, humn),
            Value::Multiplication(m1, m2) => self.eval_humn(&m1, humn) * self.eval_humn(&m2, humn),
            Value::Division(m1, m2) => self.eval_humn(&m1, humn) / self.eval_humn(&m2, humn),
        }
    }

    fn format(&self, name: &str) -> String {
        if name == "humn" {
            return String::from("humn");
        }
        match self.monkeys[name].yell.clone() {
            Value::Immediate(i) => i.to_string(),
            Value::Addition(m1, m2) => format!("({} + {})", self.format(&m1), self.format(&m2)),
            Value::Substraction(m1, m2) => format!("({} - {})", self.format(&m1), self.format(&m2)),
            Value::Multiplication(m1, m2) => format!("({} * {})", self.format(&m1), self.format(&m2)),
            Value::Division(m1, m2) => format!("({} / {})", self.format(&m1), self.format(&m2)),
        }
    }

    fn part2_fail(&self) -> i64 {
        // I was assuming this would be linear, but it turns out it's not (maybe because using
        // i64?)
        if let Value::Addition(l, r) = self.monkeys["root"].yell.clone() {
            println!("Left: {}", self.format(&l));
            println!("Right: {}", self.format(&r));
            for i in 0..100 {
                println!("f({}) - f({}) = {}", i+1, i, self.eval_humn(&l, i+1) - self.eval_humn(&l, i));
            }
            // If humn appears only once, this is linear, so l - r is of the form a*x + b
            let b = self.eval_humn(&l, 0) - self.eval_humn(&r, 0);
            println!("b = {}", b);
            let a = self.eval_humn(&l, 100) - self.eval_humn(&r, 100) - b;
            println!("a = {}", a);
            // And the solution is x = -b/a
            let x = -100*b/a;
            println!("x = {}", x);
            println!("a*x + b = {}", a*x + b);
            // Let's verify:
            let l_val = self.eval_humn(&l, x);
            let r_val = self.eval_humn(&r, x);
            println!("{} =? {}?", l_val, r_val);
            if l_val != r_val {
                panic!("This is not linear!");
            }
            return x;
        }
        -1
    }

    fn part2(&self) -> i64 {
        if let Value::Addition(l, r) = self.monkeys["root"].yell.clone() {
            let mut v1 = -100;
            let mut v2 = 100;
            loop {
                let z1 = self.eval_humn(&l, v1) - self.eval_humn(&r, v1);
                let z2 = self.eval_humn(&l, v2) - self.eval_humn(&r, v2);
                println!("[{}, {}] => [{}, {}]", v1, v2, z1, z2);
                let delta = v2 - v1;
                if z1 == 0 {
                    println!("Solution is {}", v1);
                    println!("Verify: {} =? {}", self.eval_humn(&l, v1), self.eval_humn(&r, v1));
                    return v1;
                }
                if z2 == 0 {
                    println!("Solution is {}", v2);
                    println!("Verify: {} =? {}", self.eval_humn(&l, v2), self.eval_humn(&r, v2));
                    return v2;
                }
                // We can get stuck in an infinite loop with a small interval.
                // In this case, let's just brute force it.
                // This also allow to identify when there are multiple solutions (like in
                // sample.txt).
                if delta < 20 {
                    let mut first_solution = None;
                    for i in v1..v2 {
                        if self.eval_humn(&l, i) == self.eval_humn(&r, i) {
                            println!("Solution is {}", i);
                            if let None = first_solution {
                                first_solution = Some(i);
                            }
                        }
                    }
                    if let Some(v) = first_solution {
                        return v;
                    }
                }
                if z1 > 0 && z2 > 0 ||
                   z1 < 0 && z2 < 0 {
                    v1 -= delta;
                    v2 += delta;
                }
                if z1 < 0 && z2 > 0 ||
                   z1 > 0 && z2 < 0 {
                    let ratio = z2.abs() as f64 / z1.abs() as f64;
                    v1 += ((1.0/ratio)*delta as f64/4.0) as i64;
                    v2 -= (ratio*delta as f64/4.0) as i64;
                }
            }
        }
        -1
    }

    fn part1(&self) -> i64 {
        self.eval("root")
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let tribe =  Tribe::parse(&mut lines)?;

    println!("Tribe: {:?}", tribe);
    println!("Part 1: {}", tribe.part1());
    println!("Part 2: {}", tribe.part2());

    Ok(())
}
