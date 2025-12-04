use argh::FromArgs;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(FromArgs)]
/// Solve day 4 of Advent of Code 2025.
struct Day4Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Roll,
}

impl Cell {
    fn parse(c: char) -> Result<Cell, Box<dyn Error>> {
        match c {
            '.' => Ok(Cell::Empty),
            '@' => Ok(Cell::Roll),
            _ => Err(format!("").into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Grid {
    content: Vec<Vec<Cell>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.content.iter() {
            for cell in line {
                match cell {
                    Cell::Empty => write!(f, ".")?,
                    Cell::Roll => write!(f, "@")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(repr: &str) -> Result<Grid, Box<dyn Error>> {
        Ok(Grid{
            content: repr.split('\n')
                         .filter(|line| !line.is_empty())
                         .map(|line| {
                line.chars()
                    .map(|c| Cell::parse(c))
                    .collect::<Result<Vec<_>, _>>()
            }).collect::<Result<Vec<Vec<_>>, _>>()?
        })
    }

    fn load(filename: &str) -> Result<Grid, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn width(&self) -> usize {
        if self.content.len() == 0 {
            0
        } else {
            self.content[0].len()
        }
    }

    fn height(&self) -> usize {
        self.content.len()
    }

    fn is_within(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.width() as isize && y < self.height() as isize
    }

    fn is_accessible(&self, x: usize, y: usize) -> bool {
        let mut count = 0;
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let x = x as isize + dx;
                let y = y as isize + dy;
                if !self.is_within(x, y) {
                    continue;
                }
                if self.content[y as usize][x as usize] == Cell::Roll {
                    count += 1;
                }
            }
        }
        count < 4
    }

    fn count_accessibles(&self) -> usize {
        self.content.iter()
                    .enumerate()
                    .map(|(y, line)| {
                        line.iter()
                            .enumerate()
                            .filter(|(_, cell)| **cell == Cell::Roll)
                            .filter(|(x, _)| self.is_accessible(*x, y))
                            .count()

                    })
                    .sum()
    }

    fn remove_accessibles(&mut self) {
        let accessibles = (0..self.height()).flat_map(|y| {
            (0..self.width()).map(|x| (x, y))
                .filter(|(x, y)| self.is_accessible(*x,  *y))
                .collect::<Vec<_>>()
        }).collect::<Vec<_>>();
        for (x, y) in accessibles {
            self.content[y][x] = Cell::Empty;
        }
    }

    fn count_removables(&mut self) -> usize {
        let mut count = 0;
        loop {
            let accessibles = self.count_accessibles();
            if accessibles == 0 {
                break;
            }
            count += accessibles;
            self.remove_accessibles();
        }
        count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day4Opts = argh::from_env();
    let mut grid = Grid::load(opts.filename.as_str())?;

    println!("Part 1: {}", grid.count_accessibles());
    println!("Part 2: {}", grid.count_removables());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell() {
        assert_eq!(Cell::Empty, Cell::parse('.').unwrap());
        assert_eq!(Cell::Roll, Cell::parse('@').unwrap());
        assert!(Cell::parse('!').is_err());
    }

    #[test]
    fn test_parse_grid() {
        let want = Grid {
            content: vec![
                 vec![Cell::Empty, Cell::Roll, Cell::Empty],
                 vec![Cell::Roll, Cell::Empty, Cell::Roll],
                 vec![Cell::Roll, Cell::Empty, Cell::Roll],
            ],
        };
        assert_eq!(want, Grid::parse(".@.\n@.@\n@.@").unwrap());
    }

    #[test]
    fn test_grid_size() {
        let cases = vec![
            (".@.\n@.@\n@.@", 3, 3),
            (".@.\n@.@\n@.@\n", 3, 3),
            (".", 1, 1),
            ("", 0, 0),
        ];

        for (repr, height, width) in cases {
            let grid = Grid::parse(repr).unwrap();

            assert_eq!(height, grid.height());
            assert_eq!(width, grid.width());
        }
    }

    #[test]
    fn test_is_accessible() {
        let grid = Grid::load("sample.txt").unwrap();

        // First line
        assert!(grid.is_accessible(2, 0));
        assert!(grid.is_accessible(3, 0));
        assert!(grid.is_accessible(5, 0));
        assert!(grid.is_accessible(6, 0));
        assert!(!grid.is_accessible(7, 0));
        assert!(grid.is_accessible(8, 0));
        // Second line
        assert!(grid.is_accessible(0, 1));
        assert!(!grid.is_accessible(1, 1));
        assert!(!grid.is_accessible(2, 1));
        assert!(!grid.is_accessible(4, 1));
        assert!(!grid.is_accessible(6, 1));
        assert!(!grid.is_accessible(8, 1));
        assert!(!grid.is_accessible(9, 1));
    }

    #[test]
    fn test_count_accessibles() {
        let grid = Grid::load("sample.txt").unwrap();

        assert_eq!(13, grid.count_accessibles());
    }

    #[test]
    fn test_remove_accessibles() {
        let mut grid = Grid::load("sample.txt").unwrap();
        let step2 = Grid::load("sample_step2.txt").unwrap();
        let step3 = Grid::load("sample_step3.txt").unwrap();
        let step4 = Grid::load("sample_step4.txt").unwrap();

        grid.remove_accessibles();
        assert_eq!(grid, step2);

        grid.remove_accessibles();
        assert_eq!(grid, step3);

        grid.remove_accessibles();
        assert_eq!(grid, step4);
    }

    #[test]
    fn test_count_removables() {
        let mut grid = Grid::load("sample.txt").unwrap();

        assert_eq!(43, grid.count_removables());
    }
}
