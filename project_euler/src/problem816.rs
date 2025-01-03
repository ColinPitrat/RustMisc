use std::cmp::Ordering;

struct PRNG {
    state: u64,
    modulo: u64,
}

impl PRNG {
    fn new(state: u64, modulo: u64) -> Self {
        Self{state, modulo}
    }

    fn next(&mut self) -> u64 {
        let val = self.state;
        self.state = (self.state*self.state) % self.modulo;
        val
    }
}

fn points(n: u64) -> Vec<(f64, f64)> {
    let mut prng = PRNG::new(290797, 50515093);
    let mut result = vec!();
    for _ in 0..n {
        result.push((prng.next() as f64, prng.next() as f64))
    }
    result
}

fn distance_square(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2) 
}

fn round(val: f64, places: u8) -> f64 {
    let p = 10_f64.powi(places as i32);
    (val * p)/p
}

/// Naively computes the shortest distance between any two points in `points`.
/// other. This has O(n^2) complexity.
fn min_d_naive(points: &[(f64, f64)]) -> f64 {
    if points.len() < 2 {
        return f64::MAX;
    }
    let mut min_d = distance_square(points[0], points[1]);
    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i+1..].iter() {
            let d = distance_square(*p1, *p2);
            if d < min_d {
                min_d = d;
            }
        }
    }
    return min_d;
}

/// Solves the shortest distance problem by measuring the distance of each point against each
/// other. This has O(n^2) complexity.
/// This takes about 1h for 2 millions points.
pub fn solve_naive(n: u64) -> f64 {
    let points = points(n);
    let min_d = min_d_naive(&points);
    round(min_d.sqrt(), 9)
}

/// Returns the index of the first point which has a x greater or equal to val.
fn bisect(points: &[(f64, f64)], val: f64) -> usize {
    let mut low = 0;
    let mut high = points.len()-1;
    while low < high {
        let mid = (high+low)/2;
        if points[mid].0 < val {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Divide & conquer approach to compute the shortest distance between points.
/// This is very well described in the french wikipedia article:
/// https://fr.wikipedia.org/wiki/Recherche_des_deux_points_les_plus_rapproch%C3%A9s
/// This assumes that points is sorted by the X coordinate.
fn min_d(points: &[(f64, f64)]) -> f64 {
    let length = points.len();
    // Base case, apply naive algorithm.
    if length < 4 {
        return min_d_naive(points);
    }

    // Divide: find minimum for the 2 halves of the list.
    let d1 = min_d(&points[..length/2]);
    let d2 = min_d(&points[length/2..]);
    let mut min_d = if d1 > d2 { d2 } else { d1 };

    // Combine: find the points that are in the band of min_d around the frontier.
    // These are candidates to be closer to each other than the current minimum.
    let delta = if d1 > d2 { d1 } else { d2 };
    let delta = delta.sqrt();
    let low = bisect(points, points[length/2].0 - delta);
    let low = if low > 0 { low - 1 } else { low };
    let high = bisect(points, points[length/2+1].0 + delta);

    // Build a list of those points, ordered by their Y coordinate.
    let mut points_y = vec!();
    for p in points[low..=high].iter() {
        points_y.push(p);
    }
    points_y.sort_by(|x, y| if x.1 < y.1 { Ordering::Less } else if x.1 > y.1 { Ordering::Greater } else { Ordering::Equal });

    // Compare each point with the 7 next points.
    // This is enough because if we take a 2*delta x delta rectangle around the frontier, there can
    // only be 8 points in it (the rectangle can be divided in 8 delta x delta squares).
    // We need to compare each point with the 7 before and the 7 after, but the comparison with the
    // points before is already done as we did it earlier when checking those points.
    let neighbors = 7;
    for i in 0..points_y.len() {
        let end = if i+neighbors+1 < points_y.len() { i+neighbors+1 } else { points_y.len() };
        for j in i..end {
            if i == j {
                continue;
            }
            let d = distance_square(*points_y[i], *points_y[j]);
            if d < min_d { min_d = d; }
        }
    }

    min_d
}

pub fn solve(n: u64) -> f64 {
    let mut points = points(n);

    // Step 1, sort points by their X coordinate.
    points.sort_by(|x, y| if x.0 < y.0 { Ordering::Less } else if x.0 > y.0 { Ordering::Greater } else { Ordering::Equal });

    let min_d = min_d(&points);

    round(min_d.sqrt(), 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(546446.466846479, solve(14));
    }

    #[test]
    fn test_solves_agree() {
        for n in 2..100 {
            assert_eq!(solve_naive(n), solve(n), "solve_naive and solve disagree for n={n}");
        }
    }
}
