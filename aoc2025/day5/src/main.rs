use argh::FromArgs;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 5 of Advent of Code 2025.
struct Day5Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
struct Range {
    min: u64,
    max: u64,
}

impl Range {
    fn parse(repr: &str) -> Result<Range, Box<dyn Error>> {
        let parts = repr.split('-').collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Range doesn't have 2 boundaries: '{repr}'").into());
        }
        Ok(Range{
            min: parts[0].parse()?,
            max: parts[1].parse()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RangeSet {
    ranges: Vec<Range>,
}

impl RangeSet {
    fn parse(repr: &str) -> Result<RangeSet, Box<dyn Error>> {
        let mut initial_ranges = repr.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| Range::parse(line))
            .collect::<Result<Vec<_>, _>>()?;

        // Sort ranges by minimum.
        initial_ranges.sort_by(|r1, r2| r1.min.cmp(&r2.min));

        let mut ranges = vec!();
        let mut current: Option<Range> = None;
        for range in initial_ranges {
            if current.is_some() && range.min <= current.as_ref().unwrap().max {
                // The new range may extend the current one or be included in it.
                current.as_mut().unwrap().max = std::cmp::max(range.max, current.as_ref().unwrap().max);
                continue;
            }
            if current.is_some() {
                // The new range is after the current range, close this one.
                ranges.push(current.unwrap());
            }
            // Start a new current range.
            current = Some(range);
        }
        if current.is_some() {
            ranges.push(current.unwrap());
        }

        Ok(RangeSet{ranges})
    }

    fn is_spoiled(&self, ingredient: u64) -> bool {
        for r in self.ranges.iter() {
            if ingredient >= r.min && ingredient <= r.max {
                // In a fresh range, ingredient is not spoiled.
                return false;
            }
            if ingredient < r.min {
                // Ranges are ordered and this ingredient is before this one,
                // so not fresh.
                return true;
            }
        }
        true
    }

    fn number_fresh_ingredients(&self) -> u64 {
        self.ranges.iter().map(|r| r.max+1-r.min).sum()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Ingredients {
    ids: Vec<u64>,
}

impl Ingredients {
    fn parse(repr: &str) -> Result<Ingredients, Box<dyn Error>> {
        Ok(Ingredients {
            ids: repr.split('\n')
                     .filter(|line| !line.is_empty())
                     .map(|line| line.parse())
                     .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Database {
    ranges: RangeSet,
    ingredients: Ingredients,
}

impl Database {
    fn parse(repr: &str) -> Result<Database, Box<dyn Error>> {
        let parts = repr.split("\n\n").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Database doesn't have 2 parts, found {} part(s)", parts.len()).into());
        }
        Ok(Database{
            ranges: RangeSet::parse(parts[0])?,
            ingredients: Ingredients::parse(parts[1])?,
        })
    }

    fn load(filename: &str) -> Result<Database, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn count_fresh(&self) -> usize {
        self.ingredients.ids.iter().filter(|i| !self.ranges.is_spoiled(**i)).count()
    }

    fn number_fresh_ingredients(&self) -> u64 {
        self.ranges.number_fresh_ingredients()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day5Opts = argh::from_env();
    let database = Database::load(opts.filename.as_str())?;

    println!("Part 1: {}", database.count_fresh());
    println!("Part 2: {}", database.number_fresh_ingredients());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        assert_eq!(Range{min: 3, max: 5}, Range::parse("3-5").unwrap());
        assert!(Range::parse("3").is_err());
        assert!(Range::parse("3,5").is_err());
    }

    #[test]
    fn test_parse_range_set_no_overlap() {
        let want = RangeSet{
            ranges: vec![
                Range{min: 3, max: 5},
                Range{min: 10, max: 14},
                Range{min: 16, max: 20},
            ],
        };

        assert_eq!(want, RangeSet::parse("3-5\n10-14\n16-20").unwrap());
        assert_eq!(want, RangeSet::parse("3-5\n10-14\n16-20\n").unwrap());
        assert!(RangeSet::parse("3\n-\n16").is_err());
    }

    #[test]
    fn test_parse_range_set_with_overlap() {
        let want = RangeSet{
            ranges: vec![
                Range{min: 3, max: 5},
                Range{min: 10, max: 20},
            ],
        };

        assert_eq!(want, RangeSet::parse("3-5\n10-14\n16-20\n12-18").unwrap());
        assert_eq!(want, RangeSet::parse("3-5\n10-14\n16-20\n12-18\n").unwrap());
    }

    #[test]
    fn test_range_set_is_spoiled() {
        let ranges = RangeSet::parse("3-5\n10-14\n16-20\n12-18").unwrap();

        assert!(ranges.is_spoiled(1));
        assert!(!ranges.is_spoiled(5));
        assert!(ranges.is_spoiled(8));
        assert!(!ranges.is_spoiled(11));
        assert!(!ranges.is_spoiled(17));
        assert!(ranges.is_spoiled(32));
    }

    #[test]
    fn test_parse_ingredients() {
        let want = Ingredients{
            ids: vec![1, 5, 8, 11, 17, 32],
        };

        assert_eq!(want, Ingredients::parse("1\n5\n8\n11\n17\n32").unwrap());
        assert_eq!(want, Ingredients::parse("1\n5\n8\n11\n17\n32\n").unwrap());
        assert!(Ingredients::parse("a").is_err());
    }

    #[test]
    fn test_parse_database() {
        let want = Database {
            ranges: RangeSet{
                ranges: vec![
                    Range{min: 3, max: 5},
                    Range{min: 10, max: 20},
                ],
            },
            ingredients: Ingredients{
                ids: vec![1, 5, 8, 11, 17, 32],
            },
        };

        assert_eq!(want, Database::parse("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n").unwrap());
    }

    #[test]
    fn test_load_database() {
        let want = Database {
            ranges: RangeSet{
                ranges: vec![
                    Range{min: 3, max: 5},
                    Range{min: 10, max: 20},
                ],
            },
            ingredients: Ingredients{
                ids: vec![1, 5, 8, 11, 17, 32],
            },
        };

        assert_eq!(want, Database::load("sample.txt").unwrap());
    }

    #[test]
    fn test_count_spoiled() {
        let database = Database::load("sample.txt").unwrap();
        assert_eq!(3, database.count_fresh());
    }

    #[test]
    fn test_number_fresh_ingredients() {
        let database = Database::load("sample.txt").unwrap();
        assert_eq!(14, database.number_fresh_ingredients());
    }
}
