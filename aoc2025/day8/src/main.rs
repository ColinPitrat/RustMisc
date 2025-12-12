use argh::FromArgs;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(FromArgs)]
/// Solve day 8 of Advent of Code 2025.
struct Day8Opts {
    /// the file to use as input
    #[argh(option)]
    filename: String,
}

#[derive(Debug, Eq, PartialEq)]
struct JunctionBox {
    x: usize, 
    y: usize, 
    z: usize,
}

impl JunctionBox {
    fn parse(repr: &str) -> Result<Self, Box<dyn Error>> {
        let parts = repr.split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        if parts.len() != 3 {
            return Err(format!("Expected 3 comma-separated integers, got '{repr}'").into());
        }
        let (x, y, z) = (parts[0], parts[1], parts[2]);
        Ok(JunctionBox { x, y, z })
    }

    fn distance(&self, other: &JunctionBox) -> f64 {
        ((
          (self.x as isize - other.x as isize).pow(2) +
          (self.y as isize - other.y as isize).pow(2) +
          (self.z as isize - other.z as isize).pow(2)
          ) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Network {
    boxes: Vec<JunctionBox>,
    distances: Vec<(usize, usize, f64)>,
    circuits: HashMap<usize, usize>,
    next_circuit: usize,
}

impl PartialEq for Network {
    fn eq(&self, other: &Self) -> bool {
        self.boxes == other.boxes
    }
}

impl Eq for Network {}

impl Network {
    fn parse(repr: &str) -> Result<Self, Box<dyn Error>> {
        let boxes =  repr.split('\n')
                .filter(|line| !line.is_empty())
                .map(|line| JunctionBox::parse(line)).collect::<Result<Vec<_>, _>>()?;
        let mut distances = boxes.iter().enumerate()
            .flat_map(|(i1, b1)| {
                boxes.iter().enumerate()
                .filter(|(i2, _)| i1 < *i2)
                .map(|(i2, b2)| {
                    (i1, i2, b1.distance(b2))
                })
                .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        distances.sort_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).expect("Can't compare {d1} and {d2}"));

        let next_circuit = boxes.len();
        let circuits = (0..boxes.len()).map(|i| (i, i)).collect();

        Ok(Network{
            boxes,
            distances,
            circuits,
            next_circuit,
        })
    }

    fn load(filename: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(filename)?;
        Self::parse(&content)
    }

    fn in_same_circuit(&self, i: usize, j: usize) -> bool {
        self.circuits.contains_key(&i) && self.circuits.contains_key(&j) && self.circuits[&i] == self.circuits[&j]
    }

    #[allow(dead_code)]
    fn find_closest_pair(&self) -> Option<(usize, usize)> {
        let mut min_distance = f64::MAX;
        let mut closest = None;
        for i in 0..self.boxes.len() {
            for j in 0..self.boxes.len() {
                if i == j {
                    continue;
                }
                if self.in_same_circuit(i, j) {
                    continue
                }
                let d = self.boxes[i].distance(&self.boxes[j]);
                if d < min_distance {
                    min_distance = d;
                    closest = Some((i, j));
                }
            }
        }
        closest
    }

    fn connect(&mut self, (a, b): (usize, usize)) {
        match (self.circuits.contains_key(&a), self.circuits.contains_key(&b)) {
            (true, true) => {
                // Merge the two circuits:
                // we move all elements from circuit of b to circuit of a.
                let c = self.circuits[&a];
                let to_update = self.circuits.iter()
                    .filter(|(_, v)| **v == self.circuits[&b])
                    .map(|(k, _)| *k)
                    .collect::<Vec<_>>();
                for k in to_update.iter() {
                    self.circuits.insert(*k, c);
                }
            },
            (true, false) => {
                self.circuits.insert(b, self.circuits[&a]);
            },
            (false, true) => {
                self.circuits.insert(a, self.circuits[&b]);
            },
            (false, false) => {
                self.circuits.insert(a, self.next_circuit);
                self.circuits.insert(b, self.next_circuit);
                self.next_circuit += 1;
            },
        }
    }

    fn part1(&mut self, n: usize) -> usize {
        let distances = self.distances.clone();
        let mut i = 0;
        for (i1, i2, _) in distances.iter() {
            self.connect((*i1, *i2));
            i += 1;
            if i >= n {
                break;
            }
        }

        let sizes = self.circuits.iter()
            .fold(HashMap::new(), |mut sizes, (_, v)| {
                    sizes.entry(v).and_modify(|count| *count += 1).or_insert(1 as usize);
                    sizes
            });
        let mut sizes = sizes.values().collect::<Vec<_>>();
        sizes.sort_by(|x, y| y.cmp(&x));

        sizes.iter()
            .take(3)
            .fold(1, |mut product, e| {
                product *= *e;
                product
            })
    }

    fn part2(&mut self) -> usize {
        let distances = self.distances.clone();
        let n = self.boxes.len();
        let mut i = 0;
        let mut last_x1 = 0;
        let mut last_x2 = 0;
        for (i1, i2, _) in distances.iter() {
            if self.in_same_circuit(*i1, *i2) {
                continue;
            }
            self.connect((*i1, *i2));
            last_x1 = self.boxes[*i1].x;
            last_x2 = self.boxes[*i2].x;
            i += 1;
            if i >= n {
                break;
            }
        }

        last_x1 * last_x2
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts : Day8Opts = argh::from_env();

    let mut network = Network::load(opts.filename.as_str())?;
    println!("Part 1: {}", network.part1(1000));

    let mut network = Network::load(opts.filename.as_str())?;
    println!("Part 2: {}", network.part2());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_junction_box() {
        assert_eq!(JunctionBox{ x: 162, y: 817, z: 812 }, JunctionBox::parse("162,817,812").unwrap());
        assert!(JunctionBox::parse("").is_err());
        assert!(JunctionBox::parse("162,817").is_err());
        assert!(JunctionBox::parse("162,817,812,444").is_err());
        assert!(JunctionBox::parse("a,817,812").is_err());
    }

    macro_rules! assert_almost_eq {
        ($a: expr, $b:expr, $c:expr) => {{
            assert!(($a - $b).abs() < $c, "{} == {} != {} (within {})", stringify!($a), $a, $b, $c);
        }};
    }

    #[test]
    fn test_junction_box_distance() {
        let j1 = JunctionBox::parse("162,817,812").unwrap();
        let j2 = JunctionBox::parse("425,690,689").unwrap();

       assert_almost_eq!(j1.distance(&j2), 316.9, 1e-2);
    }

    #[test]
    fn test_parse_network() {
        let want = Network {
            boxes: vec![
                       JunctionBox{ x: 162, y: 817, z: 812 },
                       JunctionBox{ x: 57, y: 618, z: 57 },
            ],
            // The equality of the distances and circuits is not checked.
            distances: vec![],
            circuits: HashMap::new(),
            next_circuit: 0,
        };
        assert_eq!(want, Network::parse("162,817,812\n57,618,57").unwrap());
        assert_eq!(want, Network::parse("162,817,812\n57,618,57\n").unwrap());
        assert!(Network::parse("162,817,812\n57,618\n").is_err());
        assert!(Network::parse("162,817,812\n57,a,57\n").is_err());
    }

    #[test]
    fn test_find_closest_pair() {
        let mut network = Network::load("sample.txt").unwrap();

        assert_eq!(Some((0, 19)), network.find_closest_pair());
        network.connect((0, 19));
        assert_eq!(0, network.circuits[&19]);

        assert_eq!(Some((0, 7)), network.find_closest_pair());
        network.connect((0, 7));
        assert_eq!(0, network.circuits[&7]);
        assert_eq!(0, network.circuits[&19]);

        assert_eq!(Some((2, 13)), network.find_closest_pair());
        network.connect((2, 13));
        assert_eq!(2, network.circuits[&13]);

        assert_eq!(Some((17, 18)), network.find_closest_pair());
        network.connect((17, 18));
        assert_eq!(17, network.circuits[&18]);

        assert_eq!(Some((9, 12)), network.find_closest_pair());
        network.connect((9, 12));
        assert_eq!(9, network.circuits[&12]);

        assert_eq!(Some((11, 16)), network.find_closest_pair());
        network.connect((11, 16));
        assert_eq!(11, network.circuits[&16]);

        assert_eq!(Some((2, 8)), network.find_closest_pair());
        network.connect((2, 8));
        assert_eq!(2, network.circuits[&8]);
        assert_eq!(2, network.circuits[&13]);

        assert_eq!(Some((14, 19)), network.find_closest_pair());
        network.connect((14, 19));
        assert_eq!(14, network.circuits[&19]);
        assert_eq!(14, network.circuits[&0]);
        assert_eq!(14, network.circuits[&7]);

        assert_eq!(Some((2, 18)), network.find_closest_pair());
        network.connect((2, 18));
        assert_eq!(2, network.circuits[&18]);
        assert_eq!(2, network.circuits[&8]);
        assert_eq!(2, network.circuits[&13]);
    }

    #[test]
    fn test_part1() {
        let mut network = Network::load("sample.txt").unwrap();
        assert_eq!(40, network.part1(10));
    }

    #[test]
    fn test_part2() {
        let mut network = Network::load("sample.txt").unwrap();
        assert_eq!(25272, network.part2());
    }
}
