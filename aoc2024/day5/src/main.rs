use argh::FromArgs;
use std::error::Error;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 2 of Advent of Code 2024.
struct Day5Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day5Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day5Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day5Opts {
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
        if Day5Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Pages {
    rules: Vec<(u32, u32)>,
    orders: Vec<Vec<u32>>,
}

impl Pages {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let lines = content.split("\n");
        let mut rules = vec!();
        let mut start_of_pages = 0;
        for (i, line) in lines.clone().enumerate() {
            if line.len() == 0 {
                start_of_pages = i+1;
                break;
            }
            log_verbose!("Reading rule '{}'", line);
            let elems = line.split("|").collect::<Vec<_>>();
            rules.push((elems[0].parse()?, elems[1].parse()?));
        }
        let mut orders = vec!();
        for line in lines.skip(start_of_pages) {
            if line.len() == 0 {
                break;
            }
            log_verbose!("Reading order '{}'", line);
            orders.push(line.split(",").map(|e| e.parse::<u32>()).collect::<Result<Vec<_>,_>>()?);
        }
        Ok(Self{rules, orders})
    }

    fn is_ordered(&self, order: &Vec<u32>) -> bool {
        log_verbose!("Checking order of: {:?}", order);
        for rule in self.rules.iter() {
            let mut found_second = false;
            for &page in order.iter() {
                if page == rule.0 {
                    if found_second {
                        // Found second then first, this fails this rule.
                        // Let's try next order.
                        log_verbose!("  Not in order for {:?}: {:?}", rule, order);
                        return false;
                    } else {
                        // Found first page (either before second or second is absent):
                        // We're fine. Let's check next rule.
                        log_verbose!("  In order for {:?}: {:?}", rule, order);
                        continue;
                    }
                }
                if page == rule.1 {
                    // Found second page, it's fine unless first page comes after.
                    found_second = true;
                }
            }
        }
        return true;
    }

    fn ordered_and_disordered(&self) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
        let mut ordered = vec!();
        let mut disordered = vec!();
        for order in self.orders.iter() {
            if self.is_ordered(order) {
                // All rules were satisfied, this is in order.
                log_verbose!("In order for all rules: {:?}", order);
                ordered.push(order.clone());
            } else {
                disordered.push(order.clone());
            }
        }
        (ordered, disordered)
    }

    fn applyable_rules(&self, order: &Vec<u32>) -> Vec<(u32, u32)> {
        let mut result = vec!();
        for &rule in self.rules.iter() {
            let (mut has_first, mut has_second) = (false, false);
            for &e in order.iter() {
                if rule.0 == e {
                    has_first = true;
                    if has_second {
                        break;
                    }
                }
                if rule.1 == e {
                    has_second = true;
                    if has_first {
                        break;
                    }
                }
            }
            if has_first && has_second {
                result.push(rule);
            }
        }
        result
    }

    fn pick_next(&self, order: &Vec<u32>, applyable_rules: &Vec<(u32, u32)>) -> Option<u32> {
        'outer: for &e in order.iter() {
            for &r in applyable_rules.iter() {
                if e == r.1 {
                    continue 'outer
                }
            }
            return Some(e);
        }
        None
    }

    fn reorder_one(&self, mut order: Vec<u32>) -> Vec<u32> {
        let mut applyable_rules = self.applyable_rules(&order);
        let mut result = vec!();
        loop {
            log_verbose!("Applyable rules: {:?}", applyable_rules);
            log_verbose!("Order: {:?}", order);
            if order.is_empty() {
                break;
            }
            if let Some(e) = self.pick_next(&order, &applyable_rules) {
                log_verbose!("Picked {}", e);
                result.push(e);
                order.retain(|&x| x != e);
                applyable_rules.retain(|&(x,y)| x != e && y != e);
            } else {
                log_verbose!("Failed to pick next for {:?}", order);
                break;
            }
        }
        result
    }

    fn reorder(&self, orders: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
        let mut result = vec!();
        for (i, order) in orders.iter().enumerate() {
            log_verbose!("Reordering {} of {}", i, orders.len());
            result.push(self.reorder_one(order.clone()));
        }
        result
    }

}

fn score(orders: &Vec<Vec<u32>>) -> u32 {
    let mut result = 0;
    for order in orders {
        if order.len() == 0 {
            // This never happens, but just to be safe.
            continue;
        }
        let mid = order.len()/2;
        log_verbose!("Adding {} which is the middle of {:?}", order[mid], order);
        result += order[mid];
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    Day5Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day5Opts::get_opts().filename.as_str())?;
    let pages = Pages::read(content.as_str())?;
    let (ordered, disordered) = pages.ordered_and_disordered();
    println!("Score of orders already in order: {:?}", score(&ordered));
    let reordered = pages.reorder(disordered);
    println!("Score of orders that had to be reordered: {:?}", score(&reordered));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(0, score(&vec![]));

        // It's unclear what the behaviour should be for an even number of
        // elements in a vector.
        // This never happens but let's be sure we at least don't break on it.
        assert_eq!(0, score(&vec![vec![]]));
        assert_eq!(2, score(&vec![vec![1, 2]]));
        assert_eq!(3, score(&vec![vec![1, 2, 3, 4]]));

        // These are the cases we really care about.
        assert_eq!(47, score(&vec![vec![23, 47, 55]]));
        assert_eq!(59, score(&vec![
                    vec![23, 12, 55],
                    vec![23, 47, 55],
        ]));
        assert_eq!(118, score(&vec![
                    vec![23, 12, 55],
                    vec![23, 47, 55],
                    vec![23, 47, 59, 44, 32],
        ]));
    }

    #[test]
    fn test_order_total_order() {
        let pages = Pages{
            // Total order: 2, 4, 6, 8, 1, 3, 5, 7
            rules: vec![
               (2, 4), (2, 6), (2, 8), (2, 1), (2, 3), (2, 5), (2, 7),
               (4, 6), (4, 8), (4, 1), (4, 3), (4, 5), (4, 7),
               (6, 8), (6, 1), (6, 3), (6, 5), (6, 7),
               (8, 1), (8, 3), (8, 5), (8, 7),
               (1, 3), (1, 5), (1, 7),
               (3, 5), (3, 7),
               (5, 7),
            ],
            orders: vec![
                // In order.
                vec![2, 6, 3],
                vec![1, 3, 5, 7],
                vec![2, 4, 5, 7],

                // Not in order
                vec![2, 6, 3, 1],
                vec![1, 3, 5, 7, 2],
                vec![6, 2, 4, 5, 7],
            ],
        };

        assert_eq!(true, pages.is_ordered(&pages.orders[0]));
        assert_eq!(true, pages.is_ordered(&pages.orders[1]));
        assert_eq!(true, pages.is_ordered(&pages.orders[2]));

        assert_eq!(false, pages.is_ordered(&pages.orders[3]));
        assert_eq!(false, pages.is_ordered(&pages.orders[4]));
        assert_eq!(false, pages.is_ordered(&pages.orders[5]));

        let want_ordered = vec![
            pages.orders[0].clone(),
            pages.orders[1].clone(),
            pages.orders[2].clone(),
        ];
        let want_disordered = vec![
            pages.orders[3].clone(),
            pages.orders[4].clone(),
            pages.orders[5].clone(),
        ];
        assert_eq!((want_ordered, want_disordered.clone()), pages.ordered_and_disordered());

        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[3].clone())));
        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[4].clone())));
        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[5].clone())));

        // Thanks to the total order we are guaranteed that the output will be in this order.
        assert_eq!(vec![2, 6, 1, 3], pages.reorder_one(pages.orders[3].clone()));
        assert_eq!(vec![2, 1, 3, 5, 7], pages.reorder_one(pages.orders[4].clone()));
        assert_eq!(vec![2, 4, 6, 5, 7], pages.reorder_one(pages.orders[5].clone()));

        let want_reordered = vec![
            vec![2, 6, 1, 3],
            vec![2, 1, 3, 5, 7],
            vec![2, 4, 6, 5, 7],
        ];
        assert_eq!(want_reordered, pages.reorder(want_disordered));
    }

    #[test]
    fn test_order_partial_order() {
        let pages = Pages{
            // Partial order: 
            //  - 2, 4, 6, 8
            //  - 1, 3, 5, 7
            rules: vec![
               (2, 4), (2, 6), (2, 8),
               (4, 6), (4, 8),
               (6, 8),
               (1, 3), (1, 5), (1, 7),
               (3, 5), (3, 7),
               (5, 7),
            ],
            orders: vec![
                // In order.
                vec![2, 6, 3],
                vec![2, 3, 6],
                vec![3, 2, 6],
                vec![1, 3, 5, 7],
                vec![2, 4, 5, 7],
                vec![5, 7, 2, 4],
                vec![2, 5, 4, 7],

                // Not in order
                vec![6, 2, 1, 3],
                vec![2, 6, 3, 1],
                vec![4, 1, 3, 5, 7, 2],
            ],
        };

        assert_eq!(true, pages.is_ordered(&pages.orders[0]));
        assert_eq!(true, pages.is_ordered(&pages.orders[1]));
        assert_eq!(true, pages.is_ordered(&pages.orders[2]));
        assert_eq!(true, pages.is_ordered(&pages.orders[3]));
        assert_eq!(true, pages.is_ordered(&pages.orders[4]));
        assert_eq!(true, pages.is_ordered(&pages.orders[5]));
        assert_eq!(true, pages.is_ordered(&pages.orders[6]));

        assert_eq!(false, pages.is_ordered(&pages.orders[7]));
        assert_eq!(false, pages.is_ordered(&pages.orders[8]));
        assert_eq!(false, pages.is_ordered(&pages.orders[9]));

        let want_ordered = vec![
            pages.orders[0].clone(),
            pages.orders[1].clone(),
            pages.orders[2].clone(),
            pages.orders[3].clone(),
            pages.orders[4].clone(),
            pages.orders[5].clone(),
            pages.orders[6].clone(),
        ];
        let want_disordered = vec![
            pages.orders[7].clone(),
            pages.orders[8].clone(),
            pages.orders[9].clone(),
        ];
        assert_eq!((want_ordered, want_disordered), pages.ordered_and_disordered());

        // Because there's no total order, we can't check the exact result but the important part
        // is that it's ordered.
        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[7].clone())));
        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[8].clone())));
        assert_eq!(true, pages.is_ordered(&pages.reorder_one(pages.orders[9].clone())));
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let pages = Pages::read(content.as_str()).unwrap();
        let (ordered, disordered) = pages.ordered_and_disordered();
        assert_eq!(143, score(&ordered));
        let reordered = pages.reorder(disordered);
        assert_eq!(123, score(&reordered));
    }
}
