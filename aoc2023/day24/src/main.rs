use argh::FromArgs;
use nalgebra::Vector3;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 24 of Advent of Code 2023.
struct Day24Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day24Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day24Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day24Opts {
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
        if Day24Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Hail {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

fn is_separator(c: char) -> bool {
    c == '@' || c == ','
}

impl Hail {
    fn read(content: &str) -> Result<Vec<Hail>, Box<dyn Error>> {
        let mut result = vec!();
        for line in content.split('\n') {
            if line.is_empty() {
                break;
            }
            let mut line = String::from(line);
            line.retain(|c| !c.is_whitespace());
            let elems = line.split(is_separator).map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
            result.push(Hail{
                p: Vector3::new(elems[0] as f64, elems[1] as f64, elems[2] as f64),
                v: Vector3::new(elems[3] as f64, elems[4] as f64, elems[5] as f64),
            })
        }
        Ok(result)
    }

    fn is_future(&self, x: f64, y: f64) -> bool {
        if self.v.x > 0. && x < self.p.x {
            //log_verbose!("x={} in the past (1) for x={} and vx={}", x, self.x, self.vx);
            return false;
        }
        if self.v.x < 0. && x > self.p.x {
            //log_verbose!("x={} in the past (2) for x={} and vx={}", x, self.x, self.vx);
            return false;
        }
        if self.v.y > 0. && y < self.p.y {
            //log_verbose!("y={} in the past (1) for y={} and vy={}", y, self.y, self.vy);
            return false;
        }
        if self.v.y < 0. && y > self.p.y {
            //log_verbose!("y={} in the past (2) for y={} and vy={}", y, self.y, self.vy);
            return false;
        }
        return true;
    }

    // Returns the coefficients of the equation of the trajectory: a*x + b*y = c.
    fn coefs_2d(&self) -> (f64, f64, f64) {
        (-self.v.y as f64, self.v.x as f64, (self.v.x * self.p.y - self.v.y * self.p.x) as f64)
    }

    fn pos_at(&self, t: f64) -> Vector3<f64> {
        self.p + t*self.v
    }

    fn adjust(&self, h0: &Hail) -> Hail {
        Hail{
            p: self.p - h0.p,
            v: self.v - h0.v,
        }
    }
}

// Resolves the system of equations:
//  a*x + b*y = c
//  d*x + e*y = d
// Returns (x, y)
fn solve_system((a, b, c): (f64, f64, f64), (d, e, f): (f64, f64, f64)) -> (f64, f64) {
    ((e*c-b*f)/(a*e-b*d), (a*f-c*d)/(a*e-b*d))
}

fn part1(hail: &Vec<Hail>, min: f64, max: f64) -> usize {
    let mut part1 = 0;
    for (i, a) in hail.iter().enumerate() {
        for b in hail[i+1..].iter() {
            //log_verbose!("Hailstone A: {}, {}, {} @ {}, {}, {}", a.x, a.y, a.z, a.vx, a.vy, a.vz);
            //log_verbose!("Hailstone B: {}, {}, {} @ {}, {}, {}", b.x, b.y, b.z, b.vx, b.vy, b.vz);
            let (x, y) = solve_system(a.coefs_2d(), b.coefs_2d());
            let mut in_or_out = "outside";
            if !a.is_future(x, y) || !b.is_future(x, y) {
                in_or_out = "past";
            } else if x > min && x < max && y > min && y < max {
                in_or_out = "inside";
                part1 += 1;
            }
            log_verbose!("Hailstones' paths cross at: x={}, y={} ({})", x, y, in_or_out);
        }
    }
    part1
}

fn times_h1_and_h2(h: &Vec<Hail>) -> (f64, f64) {
    let t1 = - h[1].p.cross(&h[2].p).dot(&h[2].v) / h[1].v.cross(&h[2].p).dot(&h[2].v);
    let t2 = - h[1].p.cross(&h[2].p).dot(&h[1].v) / h[1].p.cross(&h[2].v).dot(&h[1].v);
    let (t1, t2) = (t1.round(), t2.round());
    log_verbose!("Path crosses h1 at t={t1} and h2 at t={t2}");
    (t1, t2)
}

fn part2(hail: &Vec<Hail>) -> f64 {
    let hail_in_ref0 = hail.iter().map(|h| h.adjust(&hail[0])).collect::<Vec<_>>();
    let (t1, t2) = times_h1_and_h2(&hail_in_ref0);
    let v = (hail[1].pos_at(t1) - hail[2].pos_at(t2)) / (t1 - t2);
    let p = hail[1].pos_at(t1) - v*t1;
    p.x + p.y + p.z
}

fn main() -> Result<(), Box<dyn Error>> {
    Day24Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day24Opts::get_opts().filename.as_str())?;
    let hail = Hail::read(content.as_str())?;
    log_verbose!("Hail: {:?}", hail);

    let (min, max) = if Day24Opts::get_opts().filename == "sample.txt" {
        (7., 27.)
    } else {
        (200000000000000., 400000000000000.)
    };
    println!("Part 1 result: {}", part1(&hail, min, max));

    println!("Part 2 result: {}", part2(&hail));

    Ok(())
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use super::*;

    #[test]
    fn test_hail() {
        let hail = Hail::read("19, 13, 30 @ -2,  1, -2\n18, 19, 22 @ -1, -1, -2").unwrap();

        assert_eq!(2, hail.len());

        assert_relative_eq!(19., hail[0].p.x);
        assert_relative_eq!(13., hail[0].p.y);
        assert_relative_eq!(30., hail[0].p.z);
        assert_relative_eq!(-2., hail[0].v.x);
        assert_relative_eq!(1., hail[0].v.y);
        assert_relative_eq!(-2., hail[0].v.z);

        assert_relative_eq!(18., hail[1].p.x);
        assert_relative_eq!(19., hail[1].p.y);
        assert_relative_eq!(22., hail[1].p.z);
        assert_relative_eq!(-1., hail[1].v.x);
        assert_relative_eq!(-1., hail[1].v.y);
        assert_relative_eq!(-2., hail[1].v.z);

        assert_relative_eq!(17., hail[0].pos_at(1.).x);
        assert_relative_eq!(14., hail[0].pos_at(1.).y);
        assert_relative_eq!(28., hail[0].pos_at(1.).z);
        assert_relative_eq!(15., hail[0].pos_at(2.).x);
        assert_relative_eq!(15., hail[0].pos_at(2.).y);
        assert_relative_eq!(26., hail[0].pos_at(2.).z);

        let new_hail = hail[1].adjust(&hail[0]);
        assert_relative_eq!(-1., new_hail.p.x);
        assert_relative_eq!(6., new_hail.p.y);
        assert_relative_eq!(-8., new_hail.p.z);
        assert_relative_eq!(1., new_hail.v.x);
        assert_relative_eq!(-2., new_hail.v.y);
        assert_relative_eq!(0., new_hail.v.z);
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let hail = Hail::read(content.as_str()).unwrap();

        assert_eq!(2, part1(&hail, 7., 27.));
        assert_eq!(47., part2(&hail));
    }
}
