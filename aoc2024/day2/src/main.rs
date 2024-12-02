use argh::FromArgs;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2024.
struct Day2Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day2Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day2Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day2Opts {
    fn get_opts() -> Day2Opts {
        let o = OPTIONS.read().unwrap();
        if let Some(opts) = o.as_ref() {
            opts.clone()
        } else {
            Day2Opts{
                ..Default::default()
            }
        }
    }

    fn set_opts(opts: Day2Opts) {
        let mut o = OPTIONS.write().unwrap();
        *o = Some(opts);
    }
}

macro_rules! log_verbose {
    ($($arg:tt)*) => {{
        if Day2Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn safe_up(&self, dampening: usize) -> bool {
        log_verbose!("  Is {:?} safe up ({})?", self.levels, dampening);
        for (idx, lvl) in self.levels.iter().enumerate() {
            if idx == 0 {
                continue
            }
            let diff = lvl - self.levels[idx-1];
            if diff < 1 || diff > 3 {
                log_verbose!("    Unsafe: {} - {}", self.levels[idx-1], lvl);
                if dampening > 0 {
                    let mut r1 = Report{levels: self.levels.clone()};
                    r1.levels.remove(idx-1);
                    let mut r2 = Report{levels: self.levels.clone()};
                    r2.levels.remove(idx);
                    return r1.safe_up(dampening-1) || r2.safe_up(dampening-1)
                }
                log_verbose!("  Is {:?} safe up ({})? false", self.levels, dampening);
                return false
            }
        }
        log_verbose!("  Is {:?} safe up ({})? true", self.levels, dampening);
        true
    }

    fn safe_down(&self, dampening: usize) -> bool {
        log_verbose!("  Is {:?} safe up ({})?", self.levels, dampening);
        for (idx, lvl) in self.levels.iter().enumerate() {
            if idx == 0 {
                continue
            }
            let diff = self.levels[idx-1] - lvl;
            if diff < 1 || diff > 3 {
                log_verbose!("    Unsafe: {} - {}", self.levels[idx-1], lvl);
                if dampening > 0 {
                    let mut r1 = Report{levels: self.levels.clone()};
                    r1.levels.remove(idx-1);
                    let mut r2 = Report{levels: self.levels.clone()};
                    r2.levels.remove(idx);
                    return r1.safe_down(dampening-1) || r2.safe_down(dampening-1)
                }
                log_verbose!("  Is {:?} safe down ({})? false", self.levels, dampening);
                return false
            }
        }
        log_verbose!("  Is {:?} safe down ({})? true", self.levels, dampening);
        true
    }

    fn safe(&self, dampening: usize) -> bool {
        log_verbose!("Is {:?} safe ({})?", self.levels, dampening);
        let res = self.safe_up(dampening) || self.safe_down(dampening);
        log_verbose!("Is {:?} safe ({})? {}", self.levels, dampening, res);
        res
    }
}

fn count_safe_reports(reports: &Vec<Report>, dampening: usize) -> usize {
    reports.iter().map(|r| r.safe(dampening)).filter(|&s| s).count()
}

fn read_reports(filename: &str) -> Result<Vec<Report>, Box<dyn Error>> {
    let mut reports = vec!();
    let content = fs::read_to_string(filename)?;
    for line in content.split("\n") {
        // Ignore the last empty line.
        if line.is_empty() {
            continue;
        }
        let levels = line.split(" ").map(|l| l.parse::<i32>()).collect::<Result<Vec<_>,_>>()?;
        reports.push(Report{levels});
    }
    Ok(reports)
}

fn main() -> Result<(), Box<dyn Error>> {
    Day2Opts::set_opts(argh::from_env());

    let reports = read_reports(Day2Opts::get_opts().filename.as_str())?;

    println!("Number of safe reports (part 1): {}", count_safe_reports(&reports, 0));
    println!("Number of safe reports (part 2): {}", count_safe_reports(&reports, 1));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let report = Report{levels: vec!(1, 2, 3, 5, 7)};
        assert_eq!(true, report.safe_up(0));
        assert_eq!(false, report.safe_down(0));
        assert_eq!(true, report.safe(0));

        let report = Report{levels: vec!(7, 6, 4, 3, 1)};
        assert_eq!(false, report.safe_up(0));
        assert_eq!(true, report.safe_down(0));
        assert_eq!(true, report.safe(0));

        let report = Report{levels: vec!(7, 3, 1, 5, 8)};
        assert_eq!(false, report.safe_up(0));
        assert_eq!(false, report.safe_down(0));
        assert_eq!(false, report.safe(0));

        let report = Report{levels: vec!(7, 1, 2, 3, 4)};
        assert_eq!(false, report.safe_up(0));
        assert_eq!(false, report.safe_down(0));
        assert_eq!(false, report.safe(0));

        let report = Report{levels: vec!(1, 7, 6, 5, 4)};
        assert_eq!(false, report.safe_up(0));
        assert_eq!(false, report.safe_down(0));
        assert_eq!(false, report.safe(0));
    }

    #[test]
    fn test_safe_with_dampening_1() {
        // What was safe without dampening is still safe with.
        let report = Report{levels: vec!(1, 2, 3, 5, 7)};
        assert_eq!(true, report.safe_up(1));
        assert_eq!(false, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        let report = Report{levels: vec!(7, 6, 4, 3, 1)};
        assert_eq!(false, report.safe_up(1));
        assert_eq!(true, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // Dampening of 1 is not enough for this one.
        let report = Report{levels: vec!(7, 3, 1, 5, 8)};
        assert_eq!(false, report.safe_up(1));
        assert_eq!(false, report.safe_down(1));
        assert_eq!(false, report.safe(1));

        // First value needs to be removed to be safe up.
        let report = Report{levels: vec!(7, 1, 2, 3, 4)};
        assert_eq!(true, report.safe_up(1));
        assert_eq!(false, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // First value needs to be removed to be safe down.
        let report = Report{levels: vec!(1, 7, 6, 5, 4)};
        assert_eq!(false, report.safe_up(1));
        assert_eq!(true, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // Example from sample.txt.
        let report = Report{levels: vec!(1, 3, 2, 4, 5)};
        assert_eq!(true, report.safe_up(1));
        assert_eq!(false, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // Example from sample.txt.
        let report = Report{levels: vec!(8, 6, 4, 4, 1)};
        assert_eq!(false, report.safe_up(1));
        assert_eq!(true, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // Safe if removing 5.
        let report = Report{levels: vec!(1, 2, 5, 3, 4)};
        assert_eq!(true, report.safe_up(1));
        assert_eq!(false, report.safe_down(1));
        assert_eq!(true, report.safe(1));

        // Safe if removing 1.
        let report = Report{levels: vec!(5, 4, 1, 3, 2)};
        assert_eq!(false, report.safe_up(1));
        assert_eq!(true, report.safe_down(1));
        assert_eq!(true, report.safe(1));
    }

    // TODO: More tests with dampening values higher than 1 (not used in the challenge).

    #[test]
    fn test_sample() {
        let reports = read_reports("sample.txt").unwrap();
        assert_eq!(2, count_safe_reports(&reports, 0));
        assert_eq!(4, count_safe_reports(&reports, 1));
    }
}
