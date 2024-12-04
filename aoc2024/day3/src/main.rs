use argh::FromArgs;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2024.
struct Day3Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// verbose output
    #[argh(switch)]
    use_nom: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day3Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day3Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day3Opts {
    fn get_opts() -> Day3Opts {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Day3Opts{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Day3Opts) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day3Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

//////////
// Manual parsing. Another interesting option would be to use the crate `nom`.
//////////
fn find_token(content: &[u8], mut idx: usize, token: &str) -> (bool, usize) {
    'outer: while idx < content.len() {
        for c in token.chars() {
            idx += 1;
            if content[idx-1] as char != c {
                continue 'outer;
            }
        }
        return (true, idx)
    }
    return (false, idx)
}

fn find_mul(content: &[u8], idx: usize) -> (bool, usize) {
    find_token(content, idx, "mul(")
}

fn find_do(content: &[u8], idx: usize) -> (bool, usize) {
    find_token(content, idx, "do()")
}

fn find_dont(content: &[u8], idx: usize) -> (bool, usize) {
    find_token(content, idx, "don't()")
}

fn is_token(content: &[u8], idx: usize, token: &str) -> bool {
    for (i, c) in token.chars().enumerate() {
        if content[idx+i] as char != c {
            return false;
        }
    }
    return true;
}

fn is_mul(content: &[u8], idx: usize) -> bool {
    is_token(content, idx, "mul(")
}

fn is_do(content: &[u8], idx: usize) -> bool {
    is_token(content, idx, "do()")
}

fn is_dont(content: &[u8], idx: usize) -> bool {
    is_token(content, idx, "don't()")
}

#[derive(Debug,PartialEq,Eq)]
enum Instr {
    Do,
    Dont,
    Mul,
    NotFound,
}

fn find_instr(content: &[u8], mut idx: usize) -> (Instr, usize) {
    while idx < content.len() {
        if is_mul(content, idx) {
            let (_, end) = find_mul(content, idx);
            return (Instr::Mul, end);
        }
        if is_do(content, idx) {
            let (_, end) = find_do(content, idx);
            return (Instr::Do, end);
        }
        if is_dont(content, idx) {
            let (_, end) = find_dont(content, idx);
            return (Instr::Dont, end);
        }
        idx += 1
    }
    return (Instr::NotFound, idx)
}

fn read_number(content: &[u8], mut idx: usize) -> (Option<i32>, usize) {
    if !(content[idx] as char).is_digit(10) {
        return (None, idx);
    }
    let mut number = 0;
    while idx < content.len() && (content[idx] as char).is_digit(10) {
        number = number*10 + (content[idx] as char).to_digit(10).unwrap() as i32;
        idx += 1;
    }
    (Some(number), idx)
}

fn read_char(content: &[u8], idx: usize, c: char) -> (bool ,usize) {
    if content[idx] as char != c {
        return (false, idx)
    }
    return (true, idx+1)
}

fn read_comma(content: &[u8], idx: usize) -> (bool, usize) {
    read_char(content, idx, ',')
}

fn read_right_parenthesis(content: &[u8], idx: usize) -> (bool, usize) {
    read_char(content, idx, ')')
}

fn add_mul(content: &[u8]) -> Result<i32, Box<dyn Error>> {
    log_verbose!("Parsing {}", std::str::from_utf8(content)?);
    let mut idx = 0;
    let mut result = 0;
    let mut a ;
    let mut b ;
    let mut found ;
    let mut maybe_a ;
    let mut maybe_b ;
    while idx < content.len() {
        (found, idx) = find_mul(content, idx);
        if !found {
            continue;
        }
        (maybe_a, idx) = read_number(content, idx);
        if let Some(read_a) = maybe_a {
            a = read_a;
        } else {
            continue;
        }
        (found, idx) = read_comma(content, idx);
        if !found {
            continue;
        }
        (maybe_b, idx) = read_number(content, idx);
        if let Some(read_b) = maybe_b {
            b = read_b;
        } else {
            continue;
        }
        (found, idx) = read_right_parenthesis(content, idx);
        if !found {
            continue;
        }
        result += a * b;
    }
    Ok(result)
}

fn add_mul_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    add_mul(content.as_bytes())
}

fn add_mul_do(content: &[u8]) -> Result<i32, Box<dyn Error>> {
    log_verbose!("Parsing {}", std::str::from_utf8(content)?);
    let mut idx = 0;
    let mut result = 0;
    let mut found ;
    let mut instr;
    let (mut maybe_a, mut maybe_b);
    let (mut a, mut b);
    let mut do_mul = true;
    while idx < content.len() {
        (instr, idx) = find_instr(content, idx);
        match instr {
            Instr::Do => {
                do_mul = true;
                continue;
            }
            Instr::Dont => {
                do_mul = false;
                continue;
            }
            Instr::Mul => {}
            Instr::NotFound => {
                continue;
            }
        }
        (maybe_a, idx) = read_number(content, idx);
        if let Some(read_a) = maybe_a {
            a = read_a;
        } else {
            continue;
        }
        (found, idx) = read_comma(content, idx);
        if !found {
            continue;
        }
        (maybe_b, idx) = read_number(content, idx);
        if let Some(read_b) = maybe_b {
            b = read_b;
        } else {
            continue;
        }
        (found, idx) = read_right_parenthesis(content, idx);
        if !found {
            continue;
        }
        if do_mul {
            result += a * b;
        }
    }
    Ok(result)
}

fn add_mul_do_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    add_mul_do(content.as_bytes())
}

//////////
// Parsing using nom.
//////////

use nom::branch::alt;
use nom::bytes::streaming::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::sequence::tuple;
use nom::IResult;

fn instr_from_u8(input: &[u8]) -> Result<Instr, Box<dyn Error>> {
    match std::str::from_utf8(input).unwrap() {
        "do()" => Ok(Instr::Do),
        "don't()" => Ok(Instr::Dont),
        "mul(" => Ok(Instr::Mul),
        _ => Ok(Instr::NotFound),
    }
}

fn nom_instr(content: &[u8]) -> IResult<&[u8], Instr> {
    let mut alt_instr = map_res(
        alt((
            tag("mul("),
            tag("do()"),
            tag("don't()"),
            nom::bytes::complete::take(1u8),
        )),
        instr_from_u8
    );
    alt_instr(content)
}

fn mul_from_u8(input: (&[u8], &[u8], &[u8], &[u8])) -> Result<Option<(i32, i32)>, Box<dyn Error>> {
    Ok(
      Some((
        i32::from_str_radix(std::str::from_utf8(input.0)?, 10)?,
        i32::from_str_radix(std::str::from_utf8(input.2)?, 10)?
      ))
    )
}

fn nom_mul(content: &[u8]) -> IResult<&[u8], Option<(i32, i32)>> {
    let mut alt_mul = map_res(
        tuple((
            digit1::<_, nom::error::Error<_>>,
            tag(","),
            digit1,
            tag(")"),
        )),
        mul_from_u8
    );
    alt_mul(content)
}

fn nom_expr(mut content: &[u8], do_dont: bool) -> Result<i32, Box<dyn Error + '_>> {
    let mut do_mul = true;
    let mut result = 0;
    loop {
        if content.len() == 0 {
            break;
        }
        let (remain, instr) = nom_instr(content)?;
        content = remain;
        match instr {
            Instr::Do => do_mul = true,
            Instr::Dont => do_mul = false,
            Instr::Mul => {
                if let Ok((remain, Some((a, b)))) = nom_mul(content) {
                    if do_mul || !do_dont {
                        result += a*b;
                    }
                    content = remain;
                }
            },
            Instr::NotFound => continue,
        }
    }
    Ok(result)
}

fn nom_mul_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    Ok(nom_expr(content.as_bytes(), false).unwrap())
}

fn nom_mul_do_file(filename: &str) -> Result<i32, Box<dyn Error>> {
    let content = fs::read_to_string(filename)?;
    Ok(nom_expr(content.as_bytes(), true).unwrap())
}

fn main() -> Result<(), Box<dyn Error>> {
    Day3Opts::set_opts(argh::from_env());

    if Day3Opts::get_opts().use_nom {
        println!("Sum of valid mul statements: {}", nom_mul_file(Day3Opts::get_opts().filename.as_str())?);
        println!("Sum of valid mul statements with do: {}", nom_mul_do_file(Day3Opts::get_opts().filename.as_str())?);
    } else {
        println!("Sum of valid mul statements: {}", add_mul_file(Day3Opts::get_opts().filename.as_str())?);
        println!("Sum of valid mul statements with do: {}", add_mul_do_file(Day3Opts::get_opts().filename.as_str())?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_mul() {
        assert_eq!((true, 4), find_mul(b"mul(", 0));
        assert_eq!((false, 4), find_mul(b"bad(", 0));
        assert_eq!((true, 4), find_mul(b"mul(3,5)", 0));
        assert_eq!((true, 6), find_mul(b"zzmul(aabb", 0));
        assert_eq!((true, 6), find_mul(b"zzmul(aabbmul(", 0));
        assert_eq!((true, 14), find_mul(b"zzmul(aabbmul(", 6));
    }

    #[test]
    fn test_find_do() {
        assert_eq!((true, 4), find_do(b"do()", 0));
        assert_eq!((false, 7), find_do(b"don't()", 0));
        assert_eq!((true, 4), find_do(b"do()don't()", 0));
        assert_eq!((true, 6), find_do(b"zzdo()abb", 0));
        assert_eq!((true, 6), find_do(b"zzdo()aabbdo()", 0));
        assert_eq!((true, 14), find_do(b"zzdo()aabbdo()", 6));
    }

    #[test]
    fn test_find_dont() {
        assert_eq!((true, 7), find_dont(b"don't()", 0));
        assert_eq!((false, 4), find_dont(b"do()", 0));
        assert_eq!((true, 11), find_dont(b"do()don't()", 0));
        assert_eq!((true, 9), find_dont(b"zzdon't()abb", 0));
        assert_eq!((true, 9), find_dont(b"zzdon't()aabbdon't()", 0));
        assert_eq!((true, 20), find_dont(b"zzdon't()aabbdon't()", 9));
    }

    #[test]
    fn test_find_instr() {
        assert_eq!((Instr::Do, 4), find_instr(b"do()thenmul()ordon't()something", 0));
        assert_eq!((Instr::Do, 8), find_instr(b"somedo()thenmul()ordon't()something", 0));
        assert_eq!((Instr::Mul, 16), find_instr(b"somedo()thenmul()ordon't()something", 8));
        assert_eq!((Instr::Dont, 26), find_instr(b"somedo()thenmul()ordon't()something", 17));
        assert_eq!((Instr::NotFound, 35), find_instr(b"somedo()thenmul()ordon't()something", 27));
    }

    #[test]
    fn test_read_number() {
        assert_eq!((None, 0), read_number(b"mul(3,5)", 0));
        assert_eq!((None, 3), read_number(b"mul(3,5)", 3));
        assert_eq!((Some(3), 5), read_number(b"mul(3,5)", 4));
        assert_eq!((Some(42), 6), read_number(b"mul(42,5)", 4));
    }

    #[test]
    fn test_read_comma() {
        assert_eq!((false, 0), read_comma(b"mul(3,5)", 0));
        assert_eq!((false, 3), read_comma(b"mul(3,5)", 3));
        assert_eq!((true, 6), read_comma(b"mul(3,5)", 5));
    }

    #[test]
    fn test_read_right_parenthesis() {
        assert_eq!((false, 0), read_right_parenthesis(b"mul(3,5)", 0));
        assert_eq!((false, 5), read_right_parenthesis(b"mul(3,5)", 5));
        assert_eq!((true, 8), read_right_parenthesis(b"mul(3,5)", 7));
    }

    #[test]
    fn test_add_mul() {
        assert_eq!(15, add_mul(b"mul(3,5)").unwrap());
        assert_eq!(42, add_mul(b"mul(3,5),mul(3,9)").unwrap());
    }

    #[test]
    fn test_nom_instr() {
        assert_eq!(Ok((&b"thenmul()ordon't()something"[..], Instr::Do)), nom_instr(b"do()thenmul()ordon't()something"));
        assert_eq!(Ok((&b"omedo()thenmul()ordon't()something"[..], Instr::NotFound)), nom_instr(b"somedo()thenmul()ordon't()something"));
    }

    #[test]
    fn test_nom_mul() {
        // TODO: Test case for the error
        //assert_eq!(Ok((&b"mul(3,5)ordon't()something"[..], None)), nom_mul(b"mul(3,5)ordon't()something"));
        assert_eq!(Ok((&b"ordon't()something"[..], Some((3,5)))), nom_mul(b"3,5)ordon't()something"));
    }

    #[test]
    fn test_nom_expr_with_do() {
        assert_eq!(0, nom_expr(b"", true).unwrap());
        assert_eq!(15, nom_expr(b"do()thenmul(3,5)ordon't()thismul(7,4)something", true).unwrap());
        assert_eq!(15, nom_expr(b"thenmul(3,5)ordon't()thismul(7,4)something", true).unwrap());
        assert_eq!(43, nom_expr(b"thenmul(3,5)orthismul(7,4)something", true).unwrap());
        assert_eq!(43, nom_expr(b"thenmul(3,5)orthismul(7,4)", true).unwrap());
        assert_eq!(15, nom_expr(b"thenmul(3,5)orthismul(", true).unwrap());
    }

    #[test]
    fn test_nom_expr_without_do() {
        assert_eq!(0, nom_expr(b"", false).unwrap());
        assert_eq!(43, nom_expr(b"do()thenmul(3,5)ordon't()thismul(7,4)something", false).unwrap());
        assert_eq!(43, nom_expr(b"thenmul(3,5)ordon't()thismul(7,4)something", false).unwrap());
        assert_eq!(43, nom_expr(b"thenmul(3,5)orthismul(7,4)something", false).unwrap());
        assert_eq!(43, nom_expr(b"thenmul(3,5)orthismul(7,4)", false).unwrap());
        assert_eq!(15, nom_expr(b"thenmul(3,5)orthismul(", false).unwrap());
    }

    #[test]
    fn test_sample() {
        assert_eq!(161, add_mul_file("sample1.txt").unwrap());
        assert_eq!(161, add_mul_file("sample2.txt").unwrap());
        assert_eq!(48, add_mul_do_file("sample2.txt").unwrap());
    }
}
