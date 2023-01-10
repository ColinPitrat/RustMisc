use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Sub;
use std::ops::SubAssign;
use std::fmt::{self,Display};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
struct Cost {
    ore: i64,
    clay: i64,
    obsidian: i64,
    geodes: i64,
}

impl Cost {
    fn new(ore: i64, clay: i64, obsidian: i64, geodes: i64) -> Cost {
        Cost {ore, clay, obsidian, geodes}
    }

    fn parse(line: &str) -> Cost {
        //println!("Parsing Cost {}", line);
        let costs = line.split(" costs ").collect::<Vec<_>>()[1].split(" and ").collect::<Vec<_>>();
        let mut result = Cost{..Default::default()};
        for cost in costs {
            let (p, c) = cost.split(' ').collect_tuple().unwrap();
            //println!(" Cost parts {} - {}", p, c);
            match c {
                "ore" => result.ore = p.parse::<i64>().unwrap(),
                "clay" => result.clay = p.parse::<i64>().unwrap(),
                "obsidian" => result.obsidian = p.parse::<i64>().unwrap(),
                _ => println!("Unsupported cost: {}", c),
            }
        }
        result
    }
}

impl Sub for Cost {
    type Output = Self; 

    fn sub(self, other: Self) -> Self::Output {
        Cost {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geodes: self.geodes - other.geodes,
        }
    }
}

impl SubAssign for Cost {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Cost) -> Option<Ordering> {
        match (self.ore.cmp(&other.ore), self.clay.cmp(&other.clay), self.obsidian.cmp(&other.obsidian), self.geodes.cmp(&other.geodes)) {
            (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal, Ordering::Greater | Ordering::Equal) => Some(Ordering::Greater),
            (Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal, Ordering::Less | Ordering::Equal) => Some(Ordering::Less),
            _ => None,
        }
    }
}

impl Display for Cost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut continuation = false;
        if self.ore != 0 {
            write!(f, "{} ore", self.ore)?;
            continuation = true;
        }
        if self.clay != 0 {
            if continuation {
                write!(f, " and ")?;
            }
            write!(f, "{} clay", self.clay)?;
            continuation = true;
        }
        if self.obsidian != 0 {
            if continuation {
                write!(f, " and ")?;
            }
            write!(f, "{} obsidian", self.obsidian)?;
        }
        if self.geodes != 0 {
            if continuation {
                write!(f, " and ")?;
            }
            write!(f, "{} geodes", self.geodes)?;
        }
        Ok(())
    }
}

#[derive(Clone,Debug,Default)]
struct Blueprint {
    id: usize,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

impl Blueprint {
    fn parse(line: &str) -> Blueprint {
        //println!("Parsing Blueprint {}", line);
        let (blueprint_id, rest) = line.split(':').collect_tuple().unwrap();
        let id = blueprint_id.split(' ').collect::<Vec<_>>()[1].parse::<usize>().unwrap();
        let mut lines = rest.split('.');
        let ore_robot = Cost::parse(lines.next().unwrap());
        let clay_robot = Cost::parse(lines.next().unwrap());
        let obsidian_robot = Cost::parse(lines.next().unwrap());
        let geode_robot = Cost::parse(lines.next().unwrap());
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        }
    }
}

impl Display for Blueprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blueprint {}\n", self.id)?;
        write!(f, "  Each ore robot costs {}\n", self.ore_robot)?;
        write!(f, "  Each clay robot costs {}\n", self.clay_robot)?;
        write!(f, "  Each obsidian robot costs {}\n", self.obsidian_robot)?;
        write!(f, "  Each geode robot costs {}\n", self.geode_robot)
    }
}

#[derive(Clone,Debug,Default)]
struct Factory {
    blueprints: Vec<Blueprint>,
    resources: Cost,
    ore_robots: i64,
    clay_robots: i64,
    obsidian_robots: i64,
    geode_robots: i64,
}

impl Factory {
    fn parse(lines: Lines<BufReader<File>>) -> Factory {
        let mut blueprints = vec!();
        for l in lines {
            blueprints.push(Blueprint::parse(l.unwrap().as_str()));
        }

        let resources = Cost::new(0, 0, 0, 0);

        Factory{
            blueprints,
            resources,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn create_ore_robot(&mut self, blueprint: usize) {
        self.ore_robots += 1;
        self.resources -= self.blueprints[blueprint].ore_robot;
    }

    fn create_clay_robot(&mut self, blueprint: usize) {
        self.clay_robots += 1;
        self.resources -= self.blueprints[blueprint].clay_robot;
    }

    fn create_obsidian_robot(&mut self, blueprint: usize) {
        self.obsidian_robots += 1;
        self.resources -= self.blueprints[blueprint].obsidian_robot;
    }

    fn create_geode_robot(&mut self, blueprint: usize) {
        self.geode_robots += 1;
        self.resources -= self.blueprints[blueprint].geode_robot;
    }

    fn wait(&mut self) {
    }

    fn collect_resources(&mut self) {
        self.resources.ore += self.ore_robots;
        self.resources.clay += self.clay_robots;
        self.resources.obsidian += self.obsidian_robots;
        self.resources.geodes += self.geode_robots;
        //println!("New resources: {}", self.resources);
    }

    fn hash_value(&self, minutes: i64) -> i64 {
        let mut result = minutes;
        result *= 100;
        result += self.resources.ore;
        result *= 100;
        result += self.resources.clay;
        result *= 100;
        result += self.resources.obsidian;
        result *= 100;
        result += self.resources.geodes;
        result *= 100;
        result += self.ore_robots;
        result *= 100;
        result += self.clay_robots;
        result *= 100;
        result += self.obsidian_robots;
        result *= 100;
        result += self.geode_robots;
        result
    }

    fn max_geodes_for_blueprint(&self, cache: &mut HashMap<i64, i64>, blueprint: usize, minutes: i64) -> i64 {
        //println!("\nMinute {}: {}", 24-minutes, self);
        if minutes == 0 {
            return self.resources.geodes;
        }
        let key = self.hash_value(minutes);
        if cache.contains_key(&key) {
            return cache[&key];
        }
        let mut result = 0;
        let mut options_considered = 0;
        if self.ore_robots > self.blueprints[blueprint].clay_robot.ore && self.ore_robots > self.blueprints[blueprint].obsidian_robot.ore && self.ore_robots > self.blueprints[blueprint].geode_robot.ore {
            // No need to build more ore robots as we have enough ore each round to build any robot
            // (and we can only build one per round).
            options_considered += 1;
        } else {
            if self.resources >= self.blueprints[blueprint].ore_robot {
                //println!("Minute {} - Building an ore robot.", 24-minutes);
                let mut new_factory = self.clone();
                new_factory.collect_resources();
                new_factory.create_ore_robot(blueprint);
                result = std::cmp::max(result, new_factory.max_geodes_for_blueprint(cache, blueprint, minutes - 1));
                options_considered += 1;
            }
        }
        if self.clay_robots > self.blueprints[blueprint].obsidian_robot.clay && self.clay_robots > self.blueprints[blueprint].geode_robot.clay {
            // No need to build more ore robots as we have enough ore each round to build any robot
            // (and we can only build one per round).
            options_considered += 1;
        } else {
            if self.resources >= self.blueprints[blueprint].clay_robot {
                //println!("Minute {} - Building an clay robot.", 24-minutes);
                let mut new_factory = self.clone();
                new_factory.collect_resources();
                new_factory.create_clay_robot(blueprint);
                result = std::cmp::max(result, new_factory.max_geodes_for_blueprint(cache, blueprint, minutes - 1));
                options_considered += 1;
            } else {
                // We won't be able to build a clay robot just by waiting, so do not wait just for it
                if self.ore_robots == 0 {
                    options_considered += 1;
                }
            }
        }
        if self.obsidian_robots > self.blueprints[blueprint].geode_robot.obsidian {
            // No need to build more ore robots as we have enough ore each round to build any robot
            // (and we can only build one per round).
            options_considered += 1;
        } else {
            if self.resources >= self.blueprints[blueprint].obsidian_robot {
                //println!("Minute {} - Building an obisidian robot.", 24-minutes);
                let mut new_factory = self.clone();
                new_factory.collect_resources();
                new_factory.create_obsidian_robot(blueprint);
                result = std::cmp::max(result, new_factory.max_geodes_for_blueprint(cache, blueprint, minutes - 1));
                options_considered += 1;
            } else {
                // We won't be able to build an obsidian robot just by waiting, so do not wait just for it
                if self.ore_robots == 0 || self.clay_robots == 0 {
                    options_considered += 1;
                }
            }
        }
        if self.resources >= self.blueprints[blueprint].geode_robot {
            //println!("Minute {} - Building an geode robot.", 24-minutes);
            let mut new_factory = self.clone();
            new_factory.collect_resources();
            new_factory.create_geode_robot(blueprint);
            result = std::cmp::max(result, new_factory.max_geodes_for_blueprint(cache, blueprint, minutes - 1));
            options_considered += 1;
        } else {
            // We won't be able to build an obsidian robot just by waiting, so do not wait just for it
            if self.ore_robots == 0 || self.clay_robots == 0 || self.obsidian_robots == 0{
                options_considered += 1;
            }
        }
        // Or do nothing - only if not all kind of robots can be built, otherwise this is wasteful
        if options_considered < 4 {
            //println!("Minute {} - Collecting resources", 24-minutes);
            let mut new_factory = self.clone();
            new_factory.wait();
            new_factory.collect_resources();
            result = std::cmp::max(result, new_factory.max_geodes_for_blueprint(cache, blueprint, minutes - 1));
        }
        cache.insert(key, result);
        result
    }

    fn part1(&self) -> i64 {
        let mut quality = 0;
        for (i, b) in self.blueprints.iter().enumerate() {
            let mut cache = HashMap::new();
            let geodes = self.max_geodes_for_blueprint(&mut cache, i, 24);
            println!("Blueprint {}: {} geodes", i+1, geodes);
            quality += (i+1) as i64 * geodes;
        }
        quality
    }

    fn part2(&self) -> i64 {
        let mut result = 1;
        let rounds = 32;
        let nb_blueprints = 3;
        for i in 0..nb_blueprints {
            let mut cache = HashMap::new();
            let geodes = self.max_geodes_for_blueprint(&mut cache, i, rounds);
            println!("Blueprint {}: {} geodes", i, geodes);
            result *= geodes;
        }
        result
    }
}

impl Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.blueprints.iter() {
            write!(f, "{}", b)?;
        }

        write!(f, "Robots: {} ore, {} clay, {} obsidian\n", self.ore_robots, self.clay_robots, self.obsidian_robots)?;
        write!(f, "Resources: {}", self.resources)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let filename = "sample.txt";
    let filename = "my_input.txt";

    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let factory = Factory::parse(lines);

    println!("{}", factory);

    println!("Part 1: Total quality: {}", factory.part1());
    println!("Part 2: Top 3 blueprints product: {}", factory.part2());

    Ok(())
}
