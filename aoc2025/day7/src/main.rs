use argh::FromArgs;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(FromArgs)]
/// Solve day 7 of Advent of Code 2025.
struct Day7Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Start,
    Splitter,
    Beam,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Cell::Empty => '.',
            Cell::Start => 'S',
            Cell::Splitter => '^',
            Cell::Beam => '|',
        })
    }
}

impl Cell {
    fn parse(c: char) -> Result<Cell, Box<dyn Error>> {
        match c {
            '.' => Ok(Cell::Empty),
            'S' => Ok(Cell::Start),
            '^' => Ok(Cell::Splitter),
            '|' => Ok(Cell::Beam),
            _ => Err(format!("Unknown cell type '{c}'").into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Grid {
    content: Vec<Vec<Cell>>
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.content.iter() {
            for cell in line.iter() {
                write!(f, "{cell}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(repr: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Grid {
            content: repr.split('\n')
                         .filter(|line| !line.is_empty())
                         .map(|line| {
                                 line.chars()
                                 .map(|c| {
                                         Cell::parse(c)
                                     })
                                 .collect::<Result<Vec<_>, _>>()
                             })
                         .collect::<Result<Vec<_>, _>>()?
        })
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn part1(&self) -> usize {
        let mut grid = self.clone();
        let mut splitted = 0;
        for i in 0..(grid.content.len()-1) {
            for j in 0..grid.content[i].len() {
                if grid.content[i][j] == Cell::Start || grid.content[i][j] == Cell::Beam {
                    if grid.content[i+1][j] == Cell::Empty {
                        grid.content[i+1][j] = Cell::Beam;
                    } else if grid.content[i+1][j] == Cell::Splitter {
                        splitted += 1;
                        if j > 0 && grid.content[i+1][j-1] == Cell::Empty {
                            grid.content[i+1][j-1] = Cell::Beam;
                        }
                        if j+1 < grid.content[i+1].len() && grid.content[i+1][j+1] == Cell::Empty {
                            grid.content[i+1][j+1] = Cell::Beam;
                        }
                    }
                }
            }
        }
        splitted
    }

    fn start_x(&self) -> usize {
        // TODO: Do not panic if there's no start!
        self.content[0].iter().position(|c| *c == Cell::Start).unwrap()
    }

    fn print_at(&self, at: (usize, usize)) {
        let mut grid = self.clone();
        let (i, j) = at;
        grid.content[i][j] = Cell::Beam;
        println!("{grid}");
    }

    fn part2_rec(&self, at: (usize, usize), cache: &mut HashMap<(usize, usize), usize>) -> usize {
        //self.print_at(at);
        let (i, j) = at;
        if cache.contains_key(&at) {
            *cache.get(&at).unwrap()
        } else if i+1 == self.content.len() {
            1
        } else if self.content[i+1][j] == Cell::Splitter {
            let mut sum = 0;
            if j > 0 && self.content[i+1][j-1] == Cell::Empty {
                sum += self.part2_rec((i+1, j-1), cache);
            }
            if j+1 < self.content[i+1].len() && self.content[i+1][j+1] == Cell::Empty {
                sum += self.part2_rec((i+1, j+1), cache);
            }
            // There's really no point memoizing anywhere else than at the splitters.
            cache.insert(at, sum);
            sum
        } else { // We assume it's empty.
            debug_assert!(self.content[i+1][j] == Cell::Empty);
            self.part2_rec((i+1, j), cache)
        }
    }

    fn part2(&self) -> usize {
        let mut cache = HashMap::new();
        self.part2_rec((0, self.start_x()), &mut cache)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day7Opts = argh::from_env();

    let grid = Grid::load(&opts.filename)?;

    println!("Part 1: {}", grid.part1());
    println!("Part 2: {}", grid.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cell() {
        assert_eq!(Cell::Empty, Cell::parse('.').unwrap());
        assert_eq!(Cell::Start, Cell::parse('S').unwrap());
        assert_eq!(Cell::Splitter, Cell::parse('^').unwrap());
        assert_eq!(Cell::Beam, Cell::parse('|').unwrap());
        assert!(Cell::parse('?').is_err());
    }

    #[test]
    fn test_parse_grid() {
        let want = Grid {
            content: vec![
                 vec![Cell::Empty, Cell::Start, Cell::Empty],
                 vec![Cell::Empty, Cell::Splitter, Cell::Empty],
                 vec![Cell::Empty, Cell::Empty, Cell::Empty],
            ],
        };
        assert_eq!(want, Grid::parse(".S.\n.^.\n...").unwrap());
    }

    #[test]
    fn test_part1() {
        let grid = Grid::load("sample.txt").unwrap();
        assert_eq!(21, grid.part1());
    }

    #[test]
    fn test_part2() {
        let grid = Grid::load("sample.txt").unwrap();
        assert_eq!(40, grid.part2());
    }
}
