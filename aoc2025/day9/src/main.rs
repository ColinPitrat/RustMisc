use argh::FromArgs;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(FromArgs)]
/// Solve day 9 of Advent of Code 2025.
struct Day9Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn neighbors(&self) -> Vec<Point> {
        let mut result = vec![
            Point::new(self.x+1, self.y),
            Point::new(self.x, self.y+1),
        ];
        if self.x > 0 {
            result.push(Point::new(self.x-1, self.y))
        }
        if self.y > 0 {
            result.push(Point::new(self.x, self.y-1))
        }
        result
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Rect {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Rect {
    #[allow(unused)]
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect { x, y, width, height }
    }

    fn between(p1: &Point, p2: &Point) -> Self {
        let x = std::cmp::min(p1.x, p2.x);
        let y = std::cmp::min(p1.y, p2.y);
        let width = std::cmp::max(p1.x, p2.x) - x;
        let height = std::cmp::max(p1.y, p2.y) - y;
        Rect { x, y, width, height }
    }

    fn area(&self) -> usize {
        (self.width + 1) * (self.height + 1)
    }

    fn max_x(&self) -> usize {
        self.x + self.width
    }

    fn max_y(&self) -> usize {
        self.y + self.height
    }

    fn contains_strict(&self, p: &Point) -> bool {
        p.x > self.x && p.x < self.max_x() && p.y > self.y && p.y < self.max_y()
    }

    fn contains(&self, p: &Point) -> bool {
        p.x >= self.x && p.x <= self.max_x() && p.y >= self.y && p.y <= self.max_y()
    }

    fn is_corner(&self, p: &Point) -> bool {
        p.x == self.x && p.y == self.y ||
        p.x == self.x && p.y == self.y + self.height ||
        p.x == self.x + self.width && p.y == self.y ||
        p.x == self.x + self.width && p.y == self.y + self.height
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    red_list: Vec<Point>,
    red: HashSet<Point>,
    green: HashSet<Point>,
    min_x: usize,
    min_y: usize,
    width: usize,
    height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Point::new(x, y);
                write!(f, "{}", 
                        if self.is_red(&p) {
                            "#"
                        } else if self.is_green(&p) {
                            "X"
                        } else {
                            "."
                        }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(content: &str) -> Result<Grid, Box<dyn Error>> {
        let red_list = content.split('\n')
            .filter(|line| !line.is_empty())
            .map(|point| {
                let parts = point.split(',').collect::<Vec<_>>();
                if parts.len() != 2 {
                    Err(format!("Point doesn't have 2 parts: '{point}'").into())
                } else {
                    let (x, y) = (parts[0].parse::<usize>()?, parts[1].parse::<usize>()?);
                    Ok(Point::new(x, y))
                }
            })
            .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
        let (min_x, min_y, width, height) = red_list.iter()
            .fold((usize::MAX, usize::MAX, 0, 0), |(mut mx, mut my, mut w, mut h), p| {
                if p.x < mx {
                    mx = p.x;
                }
                if p.y < my {
                    my = p.y;
                }
                if p.x >= w {
                    w = p.x + 2
                }
                if p.y >= h {
                    h = p.y + 2
                }
                (mx, my, w, h)
        });
        let mut green = HashSet::new();
        for v in red_list.iter().cycle().take(red_list.len()+1).collect::<Vec<_>>().windows(2) {
            let p1 = v[0];
            let p2 = v[1];
            if p1.x < p2.x {
                let y = p1.y;
                for x in p1.x+1..p2.x {
                    green.insert(Point::new(x, y));
                }
            } else if p1.x > p2.x {
                let y = p1.y;
                for x in p2.x+1..p1.x {
                    green.insert(Point::new(x, y));
                }
            } else if p1.y < p2.y {
                let x = p1.x;
                for y in p1.y+1..p2.y {
                    green.insert(Point::new(x, y));
                }
            } else if p1.y > p2.y {
                let x = p1.x;
                for y in p2.y+1..p1.y {
                    green.insert(Point::new(x, y));
                }
            } else {
                panic!("Invalid points combination: {p1:?} and {p2:?}");
            }
        }
        let red = HashSet::from_iter(red_list.iter().cloned());

        Ok(Grid{
            red_list,
            red,
            green,
            min_x,
            min_y,
            width,
            height,
        })
    }

    fn load(filename: &str) -> Result<Grid, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn is_red(&self, pos: &Point) -> bool {
        self.red.contains(pos)
    }

    fn is_green(&self, pos: &Point) -> bool {
        self.green.contains(pos)
    }

    fn part1(&self) -> usize {
        let mut max_area = 0;
        for tl in self.red.iter() {
            for dr in self.red.iter() {
                let rect = Rect::between(tl, dr);
                let area = rect.area();
                if area > max_area {
                    max_area = area;
                }
            }
        }
        max_area as usize
    }

    fn is_part2_rect(&self, rect: &Rect) -> bool {
        // The rectangle must not contain any red tile.
        if self.red.iter().any(|p| rect.contains_strict(p)) {
            return false;
        }
        // The rectangle must not contain any green tile either.
        if self.green.iter().any(|p| rect.contains_strict(p)) {
            return false;
        }
        // Try up to 10 random lines to determine whether the rectangle is
        // inside or outside.
        for _ in 0..10 {
            let y = rand::random_range(rect.y..=rect.y+rect.height);
            // Skip any line that contains corners.
            if self.red.iter().any(|p| p.y == y) {
                continue;
            }
            let mut borders = self.green.iter().filter(|p| p.y == y).collect::<Vec<_>>();
            borders.sort_by(|p1, p2| p1.x.cmp(&p2.x));
            let mut is_inside = false;
            for pts in borders.windows(2) {
                is_inside = !is_inside;
                let (p1, p2) = (pts[0], pts[1]);
                if p1.x <= rect.x && p2.x > rect.x {
                    return is_inside && p2.x >= rect.x + rect.width;
                }
            }
        }
        println!("Inconclusive for {rect:?}");
        true
    }

    fn part2(&self) -> usize {
        let mut max_area = 0;
        for tl in self.red.iter() {
            for dr in self.red.iter() {
                let rect = Rect::between(tl, dr);
                let area = rect.area();
                if area > max_area {
                    if !self.is_part2_rect(&rect) {
                        continue
                    }
                    max_area = area;
                }
            }
        }
        max_area as usize
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day9Opts = argh::from_env();

    let grid = Grid::load(opts.filename.as_str())?;
    println!("Part 1: {}", grid.part1());

    println!("Part 2: {}", grid.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect() {
        let rect1 = Rect::new(7, 1, 4, 2);
        let p1 = Point::new(7, 1);
        let p2 = Point::new(11, 3);
        let rect2 = Rect::between(&p1, &p2);
        let rect3 = Rect::between(&p1, &p2);

        assert_eq!(rect1, rect2);
        assert_eq!(rect1, rect3);

        assert!(!rect1.contains_strict(&p1));
        assert!(!rect1.contains_strict(&p2));
        assert!(rect1.contains(&p1));
        assert!(rect1.contains(&p2));

        let p3 = Point::new(8, 2);
        let p4 = Point::new(10, 2);
        assert!(rect1.contains_strict(&p3));
        assert!(rect1.contains_strict(&p4));
        assert!(rect1.contains(&p3));
        assert!(rect1.contains(&p4));

        let p5 = Point::new(6, 2);
        let p6 = Point::new(12, 2);
        assert!(!rect1.contains_strict(&p5));
        assert!(!rect1.contains_strict(&p6));
        assert!(!rect1.contains(&p5));
        assert!(!rect1.contains(&p6));
    }

    #[test]
    fn test_rect_area() {
        let rect1 = Rect::new(7, 1, 4, 2);

        assert_eq!(15, rect1.area());
    }

    #[test]
    fn test_parse_grid() {
        let red_list = vec![Point::new(7, 1), Point::new(11, 1)];
        let want = Grid {
            red_list: red_list.clone(),
            red: HashSet::from_iter(red_list.iter().cloned()),
            green: HashSet::from([Point::new(8, 1), Point::new(9, 1), Point::new(10, 1)]),
            min_x: 7,
            min_y: 1,
            width: 13,
            height: 3,
        };

        assert_eq!(want, Grid::parse("7,1\n11,1").unwrap());
        assert_eq!(want, Grid::parse("7,1\n11,1\n").unwrap());
    }

    #[test]
    fn test_load_grid() {
        let red_list = vec![
            Point::new(7, 1), Point::new(11, 1), Point::new(11, 7),
            Point::new(9, 7), Point::new(9, 5), Point::new(2, 5),
            Point::new(2, 3), Point::new(7, 3)
        ];
        let want = Grid {
            red_list: red_list.clone(),
            red: HashSet::from_iter(red_list.iter().cloned()),
            green: HashSet::from([
                    Point::new(8, 1), Point::new(9, 1), Point::new(10, 1),
                    Point::new(11, 2), Point::new(11, 3), Point::new(11, 4), Point::new(11, 5), Point::new(11, 6),
                    Point::new(10, 7),
                    Point::new(9, 6),
                    Point::new(8, 5), Point::new(7, 5), Point::new(6, 5), Point::new(5, 5), Point::new(4, 5), Point::new(3, 5),
                    Point::new(2, 4),
                    Point::new(3, 3), Point::new(4, 3), Point::new(5, 3), Point::new(6, 3),
                    Point::new(7, 2),
            ]),
            min_x: 2,
            min_y: 1,
            width: 13,
            height: 9,
        };

        assert_eq!(want, Grid::load("sample.txt").unwrap());
    }

    #[test]
    fn test_display_grid() {
        let grid = Grid::load("sample.txt").unwrap();
        let want = ".............\n.......#XXX#.\n.......X...X.\n..#XXXX#...X.\n..X........X.\n..#XXXXXX#.X.\n.........X.X.\n.........#X#.\n.............\n";

        assert_eq!(want, format!("{grid}"));
    }

    #[test]
    fn test_part1() {
        let grid = Grid::load("sample.txt").unwrap();
        assert_eq!(50, grid.part1());
    }

    #[test]
    fn test_part2() {
        let grid = Grid::load("sample.txt").unwrap();

        assert_eq!(24, grid.part2());
    }

    #[test]
    fn test_part2_large() {
        let grid = Grid::load("large_sample.txt").unwrap();

        assert_eq!(14009001, grid.part2());
    }
}
