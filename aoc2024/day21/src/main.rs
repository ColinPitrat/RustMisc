use argh::FromArgs;
use std::collections::{HashSet,HashMap,VecDeque};
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::{Deref,DerefMut};
use std::sync::{LazyLock,RwLock};

#[derive(Clone, Default, FromArgs)]
/// Solve day 20 of Advent of Code 2024.
struct Day20Opts {
    /// the file to use as input
    #[argh(option)]
    filename: Option<String>,

    /// verbose output
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// explore mode: displays the shortest result for the simplest sentences
    #[argh(switch, short = 'e')]
    explore: bool,
}

impl Day20Opts {
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
        if Day20Opts::get_opts().verbose {
            println!($($arg)*);
        }
    }};
}

// A static OPTIONS used to provide access to options like 'verbose' everywhere without needing to
// pass it to all functions.
// Ideally this should be private in a separate crate together with Day20Opts definition so that
// this can only be accessed through get_opts & set_opts.
static OPTIONS: LazyLock<RwLock<Option<Day20Opts>>> = std::sync::LazyLock::new(|| RwLock::new(None));

/// A position in a 2D map (e.g. Vec<Vec<_>>).
#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Pos{
    x: usize,
    y: usize
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Pos {
    /// Creates a new position at (x,y).
    fn new(x: usize, y: usize) -> Self {
        Pos{x, y}
    }
}

/// A direction in a grid world, diagonal movements not allowed.
#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Direction::Up => "^",
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
        };
        write!(f, "{c}")
    }
}

impl Direction {
    /// Returns all the supported directions.
    fn all() -> Vec<Self> {
        vec!(Direction::Up, Direction::Right, Direction::Down, Direction::Left)
    }

    /// Returns the displacement corresponding to a direction.
    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        }
    }
}

/// Represents a sequence of displacements in a grid world.
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
struct Path(Vec<Direction>);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.iter().map(|d| d.to_string()).collect::<String>())
    }
}

impl Deref for Path {
    type Target = Vec<Direction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents a keypad on a grid, actuated by a robot.
/// The keypad doesn't have to be rectangular: some cells can be empty.
/// Each key represents a char.
struct Keypad {
    keys: Vec<Vec<Option<char>>>,
    pos: Pos,
}

impl fmt::Display for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, line) in self.keys.iter().enumerate() {
            for (x, k) in line.iter().enumerate() {
                let (before, after) = if Pos::new(x, y) == self.pos {
                    ('[', ']')
                } else {
                    (' ', ' ')
                };
                match k {
                    Some(c) => write!(f, "{before}{c}{after}")?,
                    None => write!(f, "{before} {after}")?,
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

/// Returns the displacement direction and its amplitude for a 1D displacement.
fn unit_displacement_1d(delta: i64, neg_direction: Direction, pos_direction: Direction) -> (Direction, usize) {
    let direction = if delta >= 0 {
        pos_direction
    } else {
        neg_direction
    };
    if delta != 0 {
        (direction, delta.abs() as usize)
    } else {
        (direction, 0)
    }
}

/// Returns the displacement directions and their amplitudes for a 2D displacement.
fn unit_displacement(delta: (i64, i64)) -> ((Direction, usize), (Direction, usize)) {
    (
     unit_displacement_1d(delta.0, Direction::Left, Direction::Right),
     unit_displacement_1d(delta.1, Direction::Up, Direction::Down)
    )
}

impl Keypad {
    /// Returns a numeric keypad with digits from 0 to 9 in a standard arrangement and a A key at
    /// the bottom right, next to the 0.
    ///
    /// The keypad looks like this:
    /// +---+---+---+
    /// | 7 | 8 | 9 |
    /// +---+---+---+
    /// | 4 | 5 | 6 |
    /// +---+---+---+
    /// | 1 | 2 | 3 |
    /// +---+---+---+
    ///     | 0 | A |
    ///     +---+---+
    fn numeric() -> Keypad {
        Keypad{
            keys: vec!(
                vec!(Some('7'), Some('8'), Some('9')),
                vec!(Some('4'), Some('5'), Some('6')),
                vec!(Some('1'), Some('2'), Some('3')),
                vec!(None,      Some('0'), Some('A')),
            ),
            pos: Pos::new(2, 3),
        }
    }

    /// Returns a directional keypad similar to the arrows on a keyboard and an A key at the top right.
    ///
    /// The keypad looks like this:
    ///     +---+---+
    ///     | ^ | A |
    /// +---+---+---+
    /// | < | v | > |
    /// +---+---+---+
    fn directional() -> Keypad {
        Keypad{
            keys: vec!(
                vec!(None,      Some('^'), Some('A')),
                vec!(Some('<'), Some('v'), Some('>')),
            ),
            pos: Pos::new(2, 0),
        }
    }

    /// Returns all the keys on a keypad as a single list containing the position of the key and
    /// its value.
    fn keys(&self) -> Vec<(Pos, char)> {
        let mut result = vec!();
        for (y, line) in self.keys.iter().enumerate() {
            for (x, &k) in line.iter().enumerate() {
                if let Some(c) = k {
                    result.push((Pos::new(x, y), c));
                }
            }
        }
        result
    }

    /// Returns all the shortest paths from a given start position to a given end position.
    /// There can be many different path, for example on a numeric keypad to go from 1 to 6, there
    /// are 3 paths: ^>>, >^>, >>^.
    fn find_shortest_paths(&self, start: Pos, end: Pos) -> Vec<Path> {
        let mut result = vec!();
        let mut visited = HashSet::new();
        let mut to_visit = VecDeque::new();
        let mut best = usize::MAX;
        to_visit.push_back(Step{
            pos: start,
            path: Path(vec!()),
        });
        while !to_visit.is_empty() {
            let current = to_visit.pop_front().unwrap();
            visited.insert(current.pos);
            if current.pos == end {
                if current.path.len() <= best {
                    best = current.path.len();
                    result.push(current.path.clone());
                }
            }
            //log_verbose!("At {:?}", current.pos);
            for d in Direction::all() {
                //log_verbose!("  Trying {d}");
                let (dx, dy) = d.delta();
                let (x, y) = (current.pos.x as i64 + dx, current.pos.y as i64 + dy);
                if x < 0 || y < 0 {
                    continue;
                }
                let (x, y) = (x as usize, y as usize);
                if y >= self.keys.len() || x >= self.keys[y].len() {
                    continue;
                }
                let k = self.keys[y][x];
                if k.is_none() {
                    continue;
                }
                let new_pos = Pos::new(x, y);
                if visited.contains(&new_pos) {
                    continue;
                }
                let mut path = current.path.clone();
                path.push(d);
                if path.len() < best {
                    to_visit.push_back(Step{
                        pos: new_pos,
                        path: path,
                    });
                }
            }
        }
        result
    }

    fn is_valid_key(&self, x: i64, y: i64) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let (x, y) = (x as usize, y as usize);
        if y >= self.keys.len() || x >= self.keys[y].len() {
            return false;
        }
        let k = self.keys[y][x];
        if k.is_none() {
            return false;
        }
        true
    }

    fn is_valid(&self, mut start: Pos, path: &Path) -> bool {
        for d in path.iter() {
            let (dx, dy) = d.delta();
            if !self.is_valid_key(start.x as i64 + dx, start.y as i64 + dy) {
                return false;
            }
            start.x = (start.x as i64 + dx) as usize;
            start.y = (start.y as i64 + dy) as usize;
        }
        true
    }

    /// Returns all the direct paths from a given start position to a given end position.
    /// A direct path is a path that doesn't alternate horiztonal and vertical moves.
    /// There can be up to 2 paths, for example on a numeric keypad to go from 1 to 6, there
    /// are 2 paths: ^>>, >>^.
    fn find_direct_paths(&self, start: Pos, end: Pos) -> Vec<Path> {
        let mut result = vec!();
        let (dx, dy) = (end.x as i64 - start.x as i64, end.y as i64 - start.y as i64);
        let ((dir1, dist1), (dir2, dist2)) = unit_displacement((dx, dy));
        let path1 = Path(std::iter::repeat(dir1).take(dist1).collect());
        let path2 = Path(std::iter::repeat(dir2).take(dist2).collect());
        // From playing with `explore` we notitce that:
        //  - v> produces shorter sequences than >v
        //  - ^> produces shorter sequences than >^
        // So in the end, the only case when we want to change the order is if we have right first.
        let (path1, path2) = match (dir1, dir2) {
            (Direction::Right, _) => (path2, path1),
            _ => (path1, path2),
        };
        let mut path12 = path1.clone();
        path12.extend(path2.iter());
        if self.is_valid(start, &path12) {
            result.push(path12);
        }
        if !path1.is_empty() && !path2.is_empty() {
            let mut path21 = path2.clone();
            path21.extend(path1.iter());
            if self.is_valid(start, &path21) {
                result.push(path21);
            }
        }
        result
    }

    /// Returns all the shortest paths from any key to any other key.
    /// This includes the trivial (empty) paths of a key to itself.
    #[allow(dead_code)]
    fn compute_all_paths(&self) -> HashMap<(char, char), Vec<Path>> {
        let mut result = HashMap::new();
        for (start_pos, start) in self.keys() {
            for (end_pos, end) in self.keys() {
                result.insert((start, end), self.find_shortest_paths(start_pos, end_pos));
            }
        }
        result
    }

    /// Returns all the direct paths from any key to any other key.
    /// This includes the trivial (empty) paths of a key to itself.
    /// A direct path is a path that doesn't alternate horiztonal and vertical moves.
    fn compute_direct_paths(&self) -> HashMap<(char, char), Vec<Path>> {
        let mut result = HashMap::new();
        for (start_pos, start) in self.keys() {
            for (end_pos, end) in self.keys() {
                result.insert((start, end), self.find_direct_paths(start_pos, end_pos));
            }
        }
        result
    }

    /// Returns all the shortest codes to enter on a directional keypad controlling this keypad's
    /// robot to compose the given code.
    fn compose_code(&self, code: &Code, paths: &HashMap<(char, char), Vec<Path>>) -> Codes {
        let mut start = 'A';
        let mut result = vec!(Code{code: vec!()});
        for &end in code.code.iter() {
            let mut new_result = vec!();
            for r in result.iter() {
                //log_verbose!("Looking for path from {start} to {end} in {:?}", paths.keys());
                if !paths.contains_key(&(start, end)) {
                    continue;
                }
                //log_verbose!("  Found path: {:?}", paths[&(start, end)]);
                for p in paths[&(start, end)].iter() {
                    let seq = format!("{r}{p}A").chars().collect::<Vec<_>>();
                    new_result.push(Code{code: seq});
                }
            }
            result = new_result;
            start = end;
        }
        Codes(result)
    }

    /// Returns one of the shortest codes to enter on a directional keypad controlling this keypad's
    /// robot to compose the given code.
    fn compose_code_fast(&self, code: &Code, paths: &HashMap<(char, char), Vec<Path>>) -> Code {
        let mut start = 'A';
        let mut result = String::new();
        for &end in code.code.iter() {
            //log_verbose!("Looking for path from {start} to {end} in {:?}", paths.keys());
            if !paths.contains_key(&(start, end)) {
                continue;
            }
            result = format!("{result}{}A", paths[&(start, end)][0]);
            //log_verbose!("  Found path: {:?}", paths[&(start, end)]);
            start = end;
        }
        Code::read(result.as_str())
    }
}

/// A structure that holds information for a step of the BFS.
/// Used in find_shortest_paths
#[derive(Clone,Debug,Eq,PartialEq,Hash)]
struct Step {
    pos: Pos,
    path: Path,
}

/// Holds a single code.
#[derive(Clone,Debug,Eq,PartialEq)]
struct Code {
    code: Vec<char>,
}

impl Code {
    /// Reads a single code from a string.
    fn read(line: &str) -> Self {
        Self{
            code: line.chars().collect::<Vec<_>>(),
        }
    }

    /// Returns the value of the code.
    /// Only the leading digits are used to compute the code.
    fn value(&self) -> usize {
        let mut result = 0;
        for c in self.code.iter() {
            if c.is_digit(10) {
                result *= 10;
                result += c.to_string().parse::<usize>().unwrap();
            } else {
                break;
            }
        }
        result
    }
}

impl Deref for Code {
    type Target = Vec<char>;

    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.code.iter().collect::<String>())
    }
}

/// Holds multiple codes.
struct Codes(Vec<Code>);

impl Deref for Codes {
    type Target = Vec<Code>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Codes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for code in self.0.iter() {
            writeln!(f, "{code}")?;
        }
        Ok(())
    }
}

impl Codes {
    /// Read codes to compose from an input string.
    /// Different codes are separated by a new line.
    fn read(content: &str) -> Self {
        let mut codes = vec!();
        for line in content.split('\n') {
            if line.is_empty() {
                continue;
            }
            codes.push(Code::read(line));
        }
        Self(codes)
    }
}

// We control a directional keypad for a robot
// that types on the directional keypad of a robot (temperature)
// that types on the directional keypad of a robot (radiation)
// that types on the numerical keypad of the door (depressurized)

/// A first attempt at solving the problem.
/// This works fine for part1 but the code is not generic enough to be used for part2.
/// This is a brute force approach that tries all sequences except for the last level.
/// Even once generalized, it turns out to be _way_ too slow.
#[allow(dead_code)]
fn part1(codes: &Codes) -> usize {
    let mut result = 0;
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();
    log_verbose!("{numeric}");
    log_verbose!("{directional}");

    let numeric_paths = numeric.compute_direct_paths();
    let directional_paths = directional.compute_direct_paths();

    for code in codes.iter() {
        log_verbose!(" ##############");
        log_verbose!(" # Code: {code} #");
        log_verbose!(" ##############");
        let sequences1 = numeric.compose_code(code, &numeric_paths);
        log_verbose!("Numeric (1) sequences: {}\n{sequences1}", sequences1.len());
        let mut l3_length = usize::MAX;

        for seq in sequences1.iter() {
            let sequences2 = directional.compose_code(seq, &directional_paths);
            log_verbose!("Directional (2) sequences: {}", sequences2.len());
            log_verbose!("Directional (2) sequence 0 (len={}):\n{}", sequences2[0].len(), sequences2[0]);

            for seq in sequences2.iter() {
                let sequences3 = directional.compose_code(seq, &directional_paths);
                log_verbose!("Directional (3) sequence 0 (len={}):\n{}", sequences3[0].len(), sequences3[0]);
                // We can do this because all sequences always have the same length.
                l3_length = std::cmp::min(l3_length, sequences3[0].len());
            }
        }

        result += code.value() * l3_length;
        log_verbose!("Part 1 += {}*{}", code.value(), l3_length);
    }
    result
}

/// A generalization of the `part1` approach except:
///  - using compose_code turned out to be too slow, so rely on direct paths & compose_code_fast
///  instead
///  - even with compose_code_fast this is too slow to do more than 10 iterations and we need 25.
///  The problem is that sequences length grows exponentially.
/// It turned out that compose_code_fast was initially incorrect as not all paths are of the same
/// length even when grouping the same operations together. When right is involved, doing it after
/// the vertical is faster (see find_direct_paths).
#[allow(dead_code)]
fn complexity_too_slow(codes: &Codes, iterations: usize) -> usize {
    let mut result = 0;
    let numeric = Keypad::numeric();
    let directional = Keypad::directional();
    log_verbose!("{numeric}");
    log_verbose!("{directional}");

    let numeric_paths = numeric.compute_direct_paths();
    let directional_paths = directional.compute_direct_paths();

    for code in codes.iter() {
        log_verbose!(" ##############");
        log_verbose!(" # Code: {code} #");
        log_verbose!(" ##############");
        let mut sequence = numeric.compose_code_fast(code, &numeric_paths);

        for i in 0..iterations {
            log_verbose!("Iteration {i}: {}", sequence.len());
            sequence = directional.compose_code_fast(&sequence, &directional_paths);
        }

        let length = sequence.len();

        result += code.value() * length;
        log_verbose!("Result += {}*{}", code.value(), length);
    }
    result
}

/// We build a hashmap from one pattern (a bunch of ^v<> and a A at the end) to the list of
/// patterns it generates and their counts.
/// One iteration then consists in transforming a HashMap<Pattern, Count> to the HashMap<Pattern,
/// Count> for the next step.
fn complexity_patterns(mut patterns: HashMap<String, usize>, iterations: usize) -> usize {
    let directional = Keypad::directional();
    let directional_paths = directional.compute_direct_paths();
    for i in 0..iterations {
        log_verbose!("Iteration {i}: {}", patterns.len());
        log_verbose!("Patterns: {patterns:?}");
        let mut new_patterns = HashMap::new();
        for (pattern, count) in patterns.iter() {
            let sequence = directional.compose_code_fast(&Code::read(pattern.as_str()), &directional_paths);
            log_verbose!("{}Sequence for pattern {pattern}: {sequence}", std::iter::repeat("  ").take(i).collect::<String>());
            let this_patterns = sequence.to_string()
                // We need to remove the last A as we don't want to have an empty pattern at the
                // end.
                .strip_suffix("A").unwrap()
                .split('A').map(|pattern| {
                    pattern.to_string() + "A"
                })
                .fold(HashMap::new(), |mut map, seq| {
                    *map.entry(seq).or_insert(0_usize) += count;
                    map
                });
            // TODO: Instead of merging this_patterns with new_patterns, we could directly insert in
            // new_patterns in the `fold` above (or some equivalent function).
            for (this_pattern, this_count) in this_patterns.iter() {
                *new_patterns.entry(this_pattern.clone()).or_insert(0_usize) += this_count;
            }
        }
        patterns = new_patterns;
    }
    log_verbose!("Patterns: {patterns:?}");
    patterns.iter().map(|(k, v)| v*k.len()).sum()
}

/// The final solution to the problem.
/// We don't compute the full code, we split sequences in patterns that start and end at A. We
/// build a hashmap from one pattern to the list of patterns it generates and their counts.
/// One iteration then consists in transforming a HashMap<Pattern, Count> to the HashMap<Pattern,
/// Count> for the next step.
fn complexity(codes: &Codes, iterations: usize) -> usize {
    let mut result = 0;
    let numeric = Keypad::numeric();

    let numeric_paths = numeric.compute_direct_paths();

    for code in codes.iter() {
        log_verbose!(" ##############");
        log_verbose!(" # Code: {code} #");
        log_verbose!(" ##############");
        let sequence = numeric.compose_code_fast(code, &numeric_paths);
        log_verbose!("Sequence: {sequence}");

        // TODO: extract this in a function as this is duplicated above too.
        let patterns = sequence.to_string()
            // We need to remove the last A as we don't want to have an empty pattern at the
            // end.
            .strip_suffix("A").unwrap()
            .split('A').map(|pattern| {
                pattern.to_string() + "A"
            })
            .fold(HashMap::new(), |mut map, pattern| {
                *map.entry(pattern).or_insert(0_usize) += 1;
                map
            });

        let length = complexity_patterns(patterns, iterations);

        result += code.value() * length;
        log_verbose!("Result += {}*{}", code.value(), length);
    }
    result
}

fn explore() {
    let directional = Keypad::directional();
    let directional_paths = directional.compute_direct_paths();
    for pattern in vec!("^>A", ">^A", "^<A", "<^A", "v>A", ">vA", "v<A", "<vA") {
        let mut shortest = usize::MAX;
        println!("Shortest pattern for {pattern}:");
        let sequences = directional.compose_code(&Code::read(pattern.to_string().as_str()), &directional_paths);
        for seq1 in sequences.iter() {
            log_verbose!("  Directional (1) sequence (len={}): {}", seq1.len(), seq1);
            let sequences2 = directional.compose_code(&Code::read(seq1.to_string().as_str()), &directional_paths);

            for seq2 in sequences2.iter() {
                log_verbose!("  Directional (2) sequence (len={}): {}", seq2.len(), seq2);
                let sequences3 = directional.compose_code(seq2, &directional_paths);
                let seq3 = &sequences3[0];
                log_verbose!("  Directional (3) sequence (len={}): {seq3}", seq3.len());
                if sequences3[0].len() < shortest {
                    shortest = sequences3[0].len();
                    println!("  Shortest 3-sequence so far for {pattern} is {shortest} long");
                    println!("    - {seq1}");
                    println!("    - {seq2}");
                    println!("    - {seq3}");
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Day20Opts::set_opts(argh::from_env());

    if Day20Opts::get_opts().explore {
        explore();
    } else {
        let filename = Day20Opts::get_opts().filename.ok_or("Required options not provided: --filename or --explore")?;
        let content = fs::read_to_string(filename.as_str())?;
        let codes = Codes::read(content.as_str());
        log_verbose!("Codes to type:\n{codes}");

        println!("Part 1: {}", complexity(&codes, 2));
        println!("Part 2: {}", complexity(&codes, 25));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keypad() {
        let numeric = Keypad::numeric();

        assert_eq!(" 7  8  9 \n 4  5  6 \n 1  2  3 \n    0 [A]\n", numeric.to_string());

        let directional = Keypad::directional();

        assert_eq!("    ^ [A]\n <  v  > \n", directional.to_string());
    }

    #[test]
    fn test_all_numerical_paths() {
        let paths = Keypad::numeric().compute_all_paths();

        let from_a_to_0 = &paths[&('A', '0')];
        assert_eq!(1, from_a_to_0.len());
        assert_eq!(Path(vec!(Direction::Left)), from_a_to_0[0]);

        let from_0_to_2 = &paths[&('0', '2')];
        assert_eq!(1, from_0_to_2.len());
        assert_eq!(Path(vec!(Direction::Up)), from_0_to_2[0]);

        let from_2_to_9 = &paths[&('2', '9')];
        assert_eq!(3, from_2_to_9.len());
        assert_eq!(Path(vec!(Direction::Up, Direction::Up, Direction::Right)), from_2_to_9[0]);
        assert_eq!(Path(vec!(Direction::Up, Direction::Right, Direction::Up)), from_2_to_9[1]);
        assert_eq!(Path(vec!(Direction::Right, Direction::Up, Direction::Up)), from_2_to_9[2]);

        let from_9_to_a = &paths[&('9', 'A')];
        assert_eq!(1, from_9_to_a.len());
        assert_eq!(Path(vec!(Direction::Down, Direction::Down, Direction::Down)), from_9_to_a[0]);

        let from_5_to_5 = &paths[&('5', '5')];
        assert_eq!(1, from_5_to_5.len());
        assert_eq!(Path(vec!()), from_5_to_5[0]);
    }

    #[test]
    fn test_all_directional_paths() {
        let paths = Keypad::directional().compute_all_paths();

        let from_left_to_right = &paths[&('<', '>')];
        assert_eq!(1, from_left_to_right.len());
        assert_eq!(Path(vec!(Direction::Right, Direction::Right)), from_left_to_right[0]);

        let from_left_to_up = &paths[&('<', '^')];
        assert_eq!(1, from_left_to_up.len());
        assert_eq!(Path(vec!(Direction::Right, Direction::Up)), from_left_to_up[0]);

        let from_right_to_a = &paths[&('>', 'A')];
        assert_eq!(1, from_right_to_a.len());
        assert_eq!(Path(vec!(Direction::Up)), from_right_to_a[0]);

        let from_up_to_up = &paths[&('^', '^')];
        assert_eq!(1, from_up_to_up.len());
        assert_eq!(Path(vec!()), from_up_to_up[0]);
    }

    #[test]
    fn test_direct_numerical_paths() {
        let paths = Keypad::numeric().compute_direct_paths();

        let from_a_to_0 = &paths[&('A', '0')];
        assert_eq!(1, from_a_to_0.len());
        assert_eq!(Path(vec!(Direction::Left)), from_a_to_0[0]);

        let from_0_to_2 = &paths[&('0', '2')];
        assert_eq!(1, from_0_to_2.len());
        assert_eq!(Path(vec!(Direction::Up)), from_0_to_2[0]);

        let from_2_to_9 = &paths[&('2', '9')];
        assert_eq!(2, from_2_to_9.len());
        assert_eq!(Path(vec!(Direction::Up, Direction::Up, Direction::Right)), from_2_to_9[0]);
        assert_eq!(Path(vec!(Direction::Right, Direction::Up, Direction::Up)), from_2_to_9[1]);

        let from_9_to_a = &paths[&('9', 'A')];
        assert_eq!(1, from_9_to_a.len());
        assert_eq!(Path(vec!(Direction::Down, Direction::Down, Direction::Down)), from_9_to_a[0]);

        let from_5_to_5 = &paths[&('5', '5')];
        assert_eq!(1, from_5_to_5.len());
        assert_eq!(Path(vec!()), from_5_to_5[0]);
    }

    #[test]
    fn test_direct_directional_paths() {
        let paths = Keypad::directional().compute_direct_paths();

        let from_left_to_right = &paths[&('<', '>')];
        assert_eq!(1, from_left_to_right.len());
        assert_eq!(Path(vec!(Direction::Right, Direction::Right)), from_left_to_right[0]);

        let from_left_to_up = &paths[&('<', '^')];
        assert_eq!(1, from_left_to_up.len());
        assert_eq!(Path(vec!(Direction::Right, Direction::Up)), from_left_to_up[0]);

        let from_right_to_a = &paths[&('>', 'A')];
        assert_eq!(1, from_right_to_a.len());
        assert_eq!(Path(vec!(Direction::Up)), from_right_to_a[0]);

        let from_up_to_up = &paths[&('^', '^')];
        assert_eq!(1, from_up_to_up.len());
        assert_eq!(Path(vec!()), from_up_to_up[0]);
    }

    #[test]
    fn test_code() {
        // Basic test case
        let code = Code::read("029A");

        assert_eq!("029A", code.to_string());
        assert_eq!(29, code.value());

        // Check that `value` only look at leading digits.
        let code = Code::read("029A456");

        assert_eq!("029A456", code.to_string());
        assert_eq!(29, code.value());

        // A non-numeric code.
        let code = Code::read("^>A");

        assert_eq!("^>A", code.to_string());
        assert_eq!(0, code.value());
    }

    #[test]
    fn test_compose_code() {
        let numeric = Keypad::numeric();
        let paths = numeric.compute_all_paths();

        let sequences = numeric.compose_code(&Code::read("029A"), &paths);

        assert_eq!(Code::read("<A^A^^>AvvvA"), sequences[0]);
        assert_eq!(Code::read("<A^A^>^AvvvA"), sequences[1]);
        assert_eq!(Code::read("<A^A>^^AvvvA"), sequences[2]);
    }

    #[test]
    fn test_sample_part1() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let codes = Codes::read(content.as_str());

        assert_eq!(126384, part1(&codes));
        assert_eq!(126384, complexity(&codes, 2));
    }

    #[test]
    fn test_sample_part2() {
        let content = fs::read_to_string("sample.txt").unwrap();
        let codes = Codes::read(content.as_str());

        // Not given by the problem statement, but as my result on my input is good, I'll assume
        // this one is too.
        assert_eq!(154115708116294, complexity(&codes, 25));
    }
}
