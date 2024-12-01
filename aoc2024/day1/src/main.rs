use argh::FromArgs;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 1 of Advent of Code 2024.
struct Day1Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

fn read_lists(filename: &str) -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    println!("Reading data from '{}'", filename);
    let content = fs::read_to_string(filename)?;

    let mut list1 = vec!();
    let mut list2 = vec!();
    for line in content.split("\n") {
        //println!("Read line: '{}'", line);
        // Ignore blank lines.
        if line.len() == 0 {
            println!("Ignoring blank line: '{}'", line);
            continue;
        }

        let mut elems = line.split("   ");
        if let Some(e1) = elems.next() {
            list1.push(e1.parse::<i64>()?);
        }
        if let Some(e2) = elems.next() {
            list2.push(e2.parse::<i64>()?);
        }
    }

    Ok((list1, list2))
}

// Return the sum of the distances between the ordered elements of the 2 lists.
fn compute_distance(mut list1: Vec<i64>, mut list2: Vec<i64>) -> i64 {
    list1.sort();
    list2.sort();

    // Sum the distance between elements from the 2 lists.
    list1.iter().zip(list2.iter()).map(|(e1, e2)| (e1 - e2).abs()).sum()
}

// Computes the similarity of the 2 lists as defined in the problem statement.
// That is multiply each element of the first list by the number of times it appears in the second.
fn compute_similarity(list1: Vec<i64>, list2: Vec<i64>) -> i64 {
    // This is very sub-optimal. Sorting the second list would allow for a much faster lookup of
    // the range we're interested in.
    // This is fast enough though, and very easy to understand.
    list1.iter().map(
        |&e1| e1 * list2.iter().filter(|&&e2| e2 == e1).count() as i64
    ).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day1Opts = argh::from_env();

    let (list1, list2) = read_lists(opts.filename.as_str())?;
    println!("Distance: {}", compute_distance(list1.clone(), list2.clone()));
    println!("Similarity: {}", compute_similarity(list1, list2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lists() {
        // TODO: Ideally we'd want to test various error cases: number that doesn't parse, a single
        // element on a line, more than 1 element, etc... but the parsing has not been made robust
        // anyway.
        let got = read_lists("sample.txt");
        if let Ok((list1, list2)) = got {
            let expected1 = vec!(3, 4, 2, 1, 3, 3);
            assert_eq!(list1, expected1);

            let expected2 = vec!(4, 3, 5, 3, 9, 3);
            assert_eq!(list2, expected2);
        } else {
            panic!("read_lists: Expected a valid result, got {:?}", got)
        }
    }

    #[test]
    fn test_compute_distance() {
        let list1 = vec!();
        let list2 = vec!();
        assert_eq!(0, compute_distance(list1, list2));

        let list1 = vec!(3, 4, 2, 1, 3, 3);
        let list2 = vec!(4, 3, 5, 3, 9, 3);
        assert_eq!(11, compute_distance(list1.clone(), list2.clone()));
        assert_eq!(11, compute_distance(list2, list1));

        // Lists of different sizes are not considered an error, but maybe it should!
        let list1 = vec!(3, 4, 2);
        let list2 = vec!(4, 3, 5, 3, 9, 3);
        assert_eq!(2, compute_distance(list1.clone(), list2.clone()));
        assert_eq!(2, compute_distance(list2, list1));

        let list1 = vec!(3, 4, 2, 1, 3, 3);
        let list2 = vec!(4, 3, 5);
        assert_eq!(6, compute_distance(list1.clone(), list2.clone()));
        assert_eq!(6, compute_distance(list2, list1));
    }

    #[test]
    fn test_compute_similarity() {
        let list1 = vec!();
        let list2 = vec!();
        assert_eq!(0, compute_similarity(list1, list2));

        let list1 = vec!();
        let list2 = vec!(4, 3, 5, 3, 9, 3);
        assert_eq!(0, compute_similarity(list1, list2));

        let list1 = vec!(3, 4, 2, 1, 3, 3);
        let list2 = vec!();
        assert_eq!(0, compute_similarity(list1, list2));

        let list1 = vec!(3, 4, 2, 1, 3, 3);
        let list2 = vec!(4, 3, 5, 3, 9, 3);
        assert_eq!(31, compute_similarity(list1, list2));

        // Here again lists of different sizes are not considered an error, but it makes more
        // sense...
        let list1 = vec!(3, 4, 2);
        let list2 = vec!(4, 3, 5, 3, 9, 3);
        assert_eq!(13, compute_similarity(list1.clone(), list2.clone()));
        assert_eq!(13, compute_similarity(list2, list1));

        let list1 = vec!(3, 4, 2, 1, 3, 3);
        let list2 = vec!(4, 3, 5);
        assert_eq!(13, compute_similarity(list1.clone(), list2.clone()));
        assert_eq!(13, compute_similarity(list2, list1));
    }
}
