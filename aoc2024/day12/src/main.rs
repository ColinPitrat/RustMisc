use argh::FromArgs;
use std::collections::{HashMap, HashSet,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 12 of Advent of Code 2024.
struct Day12Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day12Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day12Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

impl Day12Opts {
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
        if Day12Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

#[derive(Clone,Debug)]
struct Map {
    plants: Vec<Vec<char>>,
}

#[derive(Clone,Debug)]
struct Regions{
    // Map from coordinates of a plant to ID of its region.
    regions: HashMap<(usize, usize), usize>,
    // Perimeters of the regions.
    perimeters: HashMap<usize, usize>,
    // Areas of the regions.
    areas: HashMap<usize, usize>,
    // Sides of the regions.
    sides: HashMap<usize, usize>,
}

impl Regions {
    fn new() -> Self {
        Self{
            regions: HashMap::new(),
            perimeters: HashMap::new(),
            areas: HashMap::new(),
            sides: HashMap::new(),
        }
    }

    fn insert(&mut self, pos: (usize, usize), id: usize) {
        self.regions.insert(pos, id);
    }

    fn set_perimeter(&mut self, id: usize, perimeter: usize) {
        self.perimeters.insert(id, perimeter);
    }

    fn set_area(&mut self, id: usize, area: usize) {
        self.areas.insert(id, area);
    }

    fn add_side(&mut self, pos: &(usize, usize)) {
        let id = self.regions[pos];
        if !self.sides.contains_key(&id) {
            self.sides.insert(id, 0);
        }
        *self.sides.get_mut(&id).unwrap() += 1;
    }

    fn price_with_reduction(&self) -> usize {
        let mut result = 0;
        for id in self.areas.keys() {
            result += self.areas[id] * self.sides[id];
        }
        result
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.plants.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    fn read(content: &str) -> Result<Self, Box<dyn Error>> {
        let mut plants = vec!();
        for line in content.split("\n") {
            if line.is_empty() {
                continue;
            }
            let mut row = vec!();
            for c in line.chars() {
                row.push(c);
            }
            plants.push(row);
        }
        Ok(Self{plants})
    }

    fn in_map(&self, x: i64, y: i64) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width() && (y as usize) < self.height()
    }

    fn at(&self, x: i64, y: i64) -> char {
        if self.in_map(x, y) {
            self.plants[y as usize][x as usize]
        } else {
            '.'
        }
    }

    fn explore_region(&self, regions: &mut Regions, id: usize, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> (usize, usize) {
        let directions = vec!((-1, 0), (0, -1), (0, 1), (1, 0));
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        let mut perimeter = 0;
        let mut area = 0;
        log_verbose!("Exploring region {} at {x}, {y}", self.plants[y][x]);
        while !queue.is_empty() {
            let (x, y) = queue.pop_back().unwrap();
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            regions.insert((x, y), id);
            let mut borders = 4;
            for (dx, dy) in directions.iter() {
                let nx = x as i64+dx;
                let ny = y as i64+dy;
                if !self.in_map(nx, ny) {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);
                if self.plants[ny][nx] == self.plants[y][x] {
                    borders -= 1;
                } else {
                    continue;
                }
                if visited.contains(&(nx, ny)) {
                    continue;
                }
                queue.push_back((nx, ny));
            }
            log_verbose!("  Point at {x}, {y} gives 1 area and {borders} perimeter");
            area += 1;
            perimeter += borders;
        }
        log_verbose!("Region has a total perimeter of {perimeter} and area of {area}");
        regions.set_perimeter(id, perimeter);
        regions.set_area(id, area);
        (perimeter, area)
    }

    fn find_regions(&self, regions: &mut Regions) -> (usize, usize, usize) {
        let mut visited = HashSet::new();
        let mut total_perimeter = 0;
        let mut total_area = 0;
        let mut total_price = 0;
        let mut id = 0;
        for (y, line) in self.plants.iter().enumerate() {
            for (x, _) in line.iter().enumerate() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                let (perimeter, area) = self.explore_region(regions, id, x, y, &mut visited);
                id += 1;
                log_verbose!("Region {} starting at {}, {} has perimeter={} and area={} - price = {}", self.plants[y][x], x, y, perimeter, area, area*perimeter);
                total_perimeter += perimeter;
                total_area += area;
                total_price += area * perimeter;
            }
        }
        (total_perimeter, total_area, total_price)
    }

    /*
    fn top_sides(&self) -> usize {
        let mut result = 0;
        let mut prev = '.';
        for x in 0..self.width() {
            if self.plants[0][x] != prev {
                result += 1;
                prev = self.plants[0][x];
            }
        }
        result
    }

    fn bottom_sides(&self) -> usize {
        let mut result = 0;
        let mut prev = '.';
        let y = self.height()-1;
        for x in 0..self.width() {
            if self.plants[y][x] != prev {
                result += 1;
                prev = self.plants[y][x];
            }
        }
        result
    }

    fn left_sides(&self) -> usize {
        let mut result = 0;
        let mut prev = '.';
        for y in 0..self.height() {
            if self.plants[y][0] != prev {
                result += 1;
                prev = self.plants[y][0];
            }
        }
        result
    }

    fn right_sides(&self) -> usize {
        let mut result = 0;
        let mut prev = '.';
        let x = self.width()-1;
        for y in 0..self.height() {
            if self.plants[y][x] != prev {
                result += 1;
                prev = self.plants[y][x];
            }
        }
        result
    }

    fn inter_horizontal_sides(&self) -> usize {
        let mut result = 0;
        println!("{self}");
        // Below the line.
        println!("Below the line");
        for y in 0..self.height()-1 {
            let mut prev = '.';
            let mut prev_below = '.';
            for x in 0..self.width() {
                let current = self.plants[y][x];
                let below = self.plants[y+1][x];
                if current != below && (prev != current || (prev_below != below && (below == current || prev_below == current))) {
                    println!("New side at {},{} : prev={}, current={}, prev_below={}, below={}", x, y, prev, current, prev_below, below);
                    result += 1;
                }
                prev = current;
                prev_below = below;
            }
        }
        println!("Above the line");
        // Above the line.
        for y in 1..self.height() {
            let mut prev = '.';
            let mut prev_above = '.';
            for x in 0..self.width() {
                let current = self.plants[y][x];
                let above = self.plants[y-1][x];
                if current != above && (prev != current || (prev_above != above && (above == current || prev_above == current))) {
                    println!("New side at {},{} : prev={}, current={}, prev_above={}, above={}", x, y, prev, current, prev_above, above);
                    result += 1;
                }
                prev = current;
                prev_above = above;
            }
        }
        result
    }

    fn inter_vertical_sides(&self) -> usize {
        let mut result = 0;
        println!("{self}");
        // Right of the column.
        println!("Right of the column");
        for x in 0..self.width()-1 {
            let mut prev = '.';
            let mut prev_right = '.';
            for y in 0..self.height() {
                let current = self.plants[y][x];
                let right = self.plants[y][x+1];
                if current != right && (prev != current || (prev_right != right && (right == current || prev_right == current))) {
                    println!("New side at {},{} : prev={}, current={}, prev_right={}, right={}", x, y, prev, current, prev_right, right);
                    result += 1;
                }
                prev = current;
                prev_right = right;
            }
        }
        println!("Left of the column");
        // Left of the column.
        for x in 1..self.width() {
            let mut prev = '.';
            let mut prev_left = '.';
            for y in 0..self.height() {
                let current = self.plants[y][x];
                let left = self.plants[y][x-1];
                if current != left &&                // Necessary condition for having a border
                    (prev != current ||              // A new border starts as before that, this was a different plant.
                     (prev_left != left && (left == current || prev_left == current))
                                                     // Alternatively, a new border starts if the
                                                     // plant left changes from or to the current
                                                     // one.
                    ) {
                    println!("New side at {},{} : prev={}, current={}, prev_left={}, left={}", x, y, prev, current, prev_left, left);
                    result += 1;
                }
                prev = current;
                prev_left = left;
            }
        }
        result
    }
    */

    // TODO: Deduplicate these two functions.
    // TODO: Once done, pass it a filled Regions (from call to find_regions) and a new
    // RegionsSides and fill information about it.
    fn horizontal_sides(&self, regions: &mut Regions) -> usize {
        let mut result = 0;
        // Below the line.
        for y in 0..self.height()+1 {
            let mut prev = '.';
            let mut prev_below = '.';
            for x in 0..self.width() {
                let current = self.at(x as i64, y as i64);
                let below = self.at(x as i64, y as i64+1);
                if current != below && (prev != current || (prev_below != below && (below == current || prev_below == current))) {
                    result += 1;
                    regions.add_side(&(x,y));
                }
                prev = current;
                prev_below = below;
            }
        }
        // Above the line.
        for y in 0..self.height() {
            let mut prev = '.';
            let mut prev_above = '.';
            for x in 0..self.width() {
                let current = self.at(x as i64, y as i64);
                let above = self.at(x as i64, y as i64-1);
                if current != above && (prev != current || (prev_above != above && (above == current || prev_above == current))) {
                    result += 1;
                    regions.add_side(&(x,y));
                }
                prev = current;
                prev_above = above;
            }
        }
        result
    }

    fn vertical_sides(&self, regions: &mut Regions) -> usize {
        let mut result = 0;
        // Right of the column.
        for x in 0..self.width()+1 {
            let mut prev = '.';
            let mut prev_right = '.';
            for y in 0..self.height() {
                let current = self.at(x as i64, y as i64);
                let right = self.at(x as i64+1, y as i64);
                if current != right && (prev != current || (prev_right != right && (right == current || prev_right == current))) {
                    result += 1;
                    regions.add_side(&(x,y));
                }
                prev = current;
                prev_right = right;
            }
        }
        // Left of the column.
        for x in 0..self.width() {
            let mut prev = '.';
            let mut prev_left = '.';
            for y in 0..self.height() {
                let current = self.at(x as i64, y as i64);
                let left = self.at(x as i64-1, y as i64);
                if current != left &&                // Necessary condition for having a border
                    (prev != current ||              // A new border starts as before that, this was a different plant.
                     (prev_left != left && (left == current || prev_left == current))
                                                     // Alternatively, a new border starts if the
                                                     // plant left changes from or to the current
                                                     // one.
                    ) {
                    result += 1;
                    regions.add_side(&(x,y));
                }
                prev = current;
                prev_left = left;
            }
        }
        result
    }

    /*
    fn all_sides_old(&self) -> usize {
        self.top_sides() + self.inter_horizontal_sides() + self.bottom_sides() + 
        self.left_sides() + self.inter_vertical_sides() + self.right_sides()
    }
    */

    fn all_sides(&self, regions: &mut Regions) -> usize {
        self.vertical_sides(regions) + self.horizontal_sides(regions)
    }

    fn height(&self) -> usize {
        self.plants.len()
    }

    fn width(&self) -> usize {
        if self.plants.len() == 0 {
            0
        } else {
            self.plants[0].len()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day12Opts::set_opts(argh::from_env());

    let content = fs::read_to_string(Day12Opts::get_opts().filename.as_str())?;
    let map = Map::read(content.as_str())?;
    log_verbose!("Map:\n{map}");
    let mut regions = Regions::new();
    let (perimeter, area, price) = map.find_regions(&mut regions);
    println!("Total perimeter: {perimeter} - Total area: {area} - Total price: {price}");
    map.all_sides(&mut regions);
    let price = regions.price_with_reduction();
    println!("Total price with reduction: {price}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let map = Map::read("AAAA\nBBCD\nBBCC\nEEEC\n").unwrap();

        assert_eq!("AAAA\nBBCD\nBBCC\nEEEC\n", format!("{map}"));

        assert_eq!(4, map.width());
        assert_eq!(4, map.height());

        let mut visited = HashSet::new();
        let mut regions = Regions::new();
        assert_eq!((10, 4), map.explore_region(&mut regions, 0, 0, 0, &mut visited));
        assert_eq!((8, 4), map.explore_region(&mut regions, 1, 0, 1, &mut visited));
        assert_eq!((10, 4), map.explore_region(&mut regions, 2, 2, 1, &mut visited));
        assert_eq!((4, 1), map.explore_region(&mut regions, 3, 3, 1, &mut visited));
        assert_eq!((8, 3), map.explore_region(&mut regions, 4, 0, 3, &mut visited));
        assert_eq!(16, visited.len());

        let mut regions = Regions::new();
        assert_eq!((40, 16, 140), map.find_regions(&mut regions));
    }

    #[test]
    fn test_example2() {
        let map = Map::read("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n").unwrap();

        assert_eq!("OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO\n", format!("{map}"));

        assert_eq!(5, map.width());
        assert_eq!(5, map.height());

        let mut regions = Regions::new();
        assert_eq!((52, 25, 772), map.find_regions(&mut regions));
    }

    #[test]
    fn test_non_square() {
        let map = Map::read("AAAA\nBBCD").unwrap();

        assert_eq!("AAAA\nBBCD\n", format!("{map}"));

        assert_eq!(4, map.width());
        assert_eq!(2, map.height());

        assert_eq!(false, map.in_map(-1, -1));
        assert_eq!(true, map.in_map(0, 0));
        assert_eq!(true, map.in_map(1, 1));
        assert_eq!(true, map.in_map(3, 1));
        assert_eq!(false, map.in_map(1, 2));
        assert_eq!(false, map.in_map(4, 1));

        let mut visited = HashSet::new();
        let mut regions = Regions::new();
        assert_eq!((10, 4), map.explore_region(&mut regions, 0, 0, 0, &mut visited));
        assert_eq!(4, visited.len());

        let mut regions = Regions::new();
        assert_eq!((24, 8, 60), map.find_regions(&mut regions));
    }

    #[test]
    fn test_sample() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let map = Map::read(content.as_str()).unwrap();

        assert_eq!(10, map.width());
        assert_eq!(10, map.height());

        let mut regions = Regions::new();
        assert_eq!((176, 100, 1930), map.find_regions(&mut regions));
    }

    #[test]
    fn test_find_sides() {
        let map = Map::read("XOOXA\nXXXXX\nOXXOB\nXXXXB\n").unwrap();

        assert_eq!(5, map.width());
        assert_eq!(4, map.height());

        /*
        assert_eq!(4, map.top_sides());
        assert_eq!(2, map.bottom_sides());
        assert_eq!(13, map.inter_horizontal_sides());
        assert_eq!(3, map.left_sides());
        assert_eq!(3, map.right_sides());
        assert_eq!(13, map.inter_vertical_sides());

        assert_eq!(19, map.horizontal_sides());
        assert_eq!(19, map.vertical_sides());
        */

        let mut regions = Regions::new();
        assert_eq!((48, 20, 348), map.find_regions(&mut regions));
        assert_eq!(38, map.all_sides(&mut regions));
        assert_eq!(18, regions.sides[&0]);
    }
}
