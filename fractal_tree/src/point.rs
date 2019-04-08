#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point{
        Point{x, y}
    }

    pub fn distance(p1: &Point, p2: &Point) -> f64 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx*dx + dy*dy).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance() {
        let cases = vec!(
                (Point::new(0.0, 0.0), Point::new(0.0, 0.0), 0.0),
                (Point::new(4.0, 8.0), Point::new(4.0, 8.0), 0.0),
                (Point::new(0.0, 0.0), Point::new(1.0, 0.0), 1.0),
                (Point::new(3.0, 0.0), Point::new(0.0, 4.0), 5.0),
                (Point::new(-3.0, 0.0), Point::new(0.0, 4.0), 5.0),
                (Point::new(3.0, 0.0), Point::new(0.0, -4.0), 5.0),
                (Point::new(-3.0, 0.0), Point::new(0.0, -4.0), 5.0),
                );
        for c in cases {
            let got = Point::distance(&c.0, &c.1);
            assert_eq!(got, c.2, "Unexpected distance between {:?} and {:?}, got {}, expected {}", c.0, c.1, got, c.2);
        }
    }
}
