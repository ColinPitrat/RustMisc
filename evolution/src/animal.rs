extern crate rand;

use crate::grid::CellContent;
use crate::grid::Grid;
use crate::stats::StatsItem;
use rand::Rng;
use std::cmp;
use std::cell::Cell;
use std::rc::Rc;

const MIN_RANGE : u32 = 1;
const MAX_RANGE : u32 = 4;
const MIN_SPEED : u32 = 1;
const MAX_SPEED : u32 = 4;
const EAT_TO_MATE : u32 = 3;

pub struct Animal {
    x: Cell<u32>,
    y: Cell<u32>,
    mated: Cell<bool>, // Whether this animal reproduced already
    eaten: Cell<u32>, // How much the animal ate in this round
    range: u32, // How far the animal can see
    speed: u32, // How far the animal can go at each step
}

pub struct Animals {
    animals: Vec<Rc<Animal>>,
}

impl Animal {
    pub fn new(x: u32, y: u32) -> Animal {
        let range = rand::thread_rng().gen_range(MIN_RANGE, MAX_RANGE+1);
        let speed = rand::thread_rng().gen_range(MIN_SPEED, MAX_SPEED+1);
        Animal{
            x: Cell::new(x),
            y: Cell::new(y),
            mated: Cell::new(false),
            eaten: Cell::new(0),
            range, speed
        }
    }

    pub fn assert_animal(&self, grid: &Grid) {
        match grid.at(self.x.get(), self.y.get()) {
            CellContent::Animal(_) => {},
            CellContent::Plant(_) => {
                panic!("Cell {}, {} should contain an animal but contains a plant", self.x.get(), self.y.get());
            }
            CellContent::Empty => {
                panic!("Cell {}, {} should contain an animal but is empty", self.x.get(), self.y.get());
            }
        };
    }

    pub fn move_to(&self, grid: &mut Grid, x: u32, y: u32) {
        if x == self.x.get() && y == self.y.get() {
            return;
        }
        match grid.at(x, y) {
            CellContent::Animal(_) => {
                //println!("There's another animal at {}, {} - not moving.", x, y);
                return;
            },
            CellContent::Plant(plant) => {
                plant.remove();
                //println!("There's a plant at {}, {}", x, y);
            }
            CellContent::Empty => {
                //println!("Empty at {}, {}", x, y);
            }
        }
        //println!("Move from {}, {} to {}, {}.", self.x.get(), self.y.get(), x, y);
        self.assert_animal(grid);
        if let CellContent::Animal(my_rc) = grid.at(self.x.get(), self.y.get()) {
            let new_animal = CellContent::Animal(Rc::clone(my_rc));
            grid.set_content(x, y, new_animal);
        };
        grid.set_content(self.x.get(), self.y.get(), CellContent::Empty);
        self.x.set(x);
        self.y.set(y);
    }

    pub fn mix_with(&self, other: &Rc<Animal>, x: u32, y: u32) -> Animal {
        let r = rand::thread_rng().gen_range(0, 1024);
        let range = if r % 2 == 0 {
            self.range
        } else {
            other.range
        };
        let speed = if (r / 2) % 2 == 0 {
            self.speed
        } else {
            other.speed
        };
        // Created with mated and eaten so that it doesn't get killed.
        Animal{
            x: Cell::new(x),
            y: Cell::new(y),
            mated: Cell::new(true),
            eaten: Cell::new(EAT_TO_MATE),
            range, speed
        }
    }

    pub fn reproduce(&self, other: &Rc<Animal>, grid: &Grid) -> Vec<Animal> {
        self.mated.set(true);
        other.mated.set(true);
        let (mut new_x, mut new_y) = (-1, -1);
        for (x, y) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, 20) {
            // TODO: move this logic (duplicated in update) into the iterator
            if x < 0 || x >= grid.width() as i32 {
                continue;
            }
            if y < 0 || y >= grid.height() as i32 {
                continue;
            }
            match grid.at(x as u32, y as u32) {
                CellContent::Animal(_) => continue,
                CellContent::Plant(plant) => {
                    new_x = x;
                    new_y = y;
                    //println!("Remove plant at {}, {}", new_x, new_y);
                    plant.remove();
                    break;
                },
                CellContent::Empty => {
                    new_x = x;
                    new_y = y;
                    break;
                }
            };
        }
        let mut result = vec!();
        if new_x >= 0 && new_y >= 0 {
            //println!("New animal at {}, {}", new_x, new_y);
            result.push(self.mix_with(other, new_x as u32, new_y as u32));
        }
        result
    }

    pub fn update(&self, grid: &mut Grid) -> Vec<Rc<Animal>> {
        let (mut new_x, mut new_y) = (self.x.get(), self.y.get());
        let mut must_move = false;
        let mut new_animals = vec!();
        for (tx, ty) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, self.range as i32) {
            if tx < 0 || tx >= grid.width() as i32 {
                continue;
            }
            if ty < 0 || ty >= grid.height() as i32 {
                continue;
            }
            let (dx, dy) = (tx - self.x.get() as i32, ty - self.y.get() as i32);
            if dx == 0 && dy == 0 {
                continue;
            }
            // The animal cannot necessarily move as far in one round, this are the capped value in
            // this direction.
            let move_dx = if dx > 0 { cmp::min(self.speed as i32, dx) } else { cmp::max(-(self.speed as i32), dx) };
            let move_dy = if dy > 0 { cmp::min(self.speed as i32, dy) } else { cmp::max(-(self.speed as i32), dy) };
            let (tx, ty) = (tx as u32, ty as u32);
            if self.eaten.get() < EAT_TO_MATE {
                if let CellContent::Plant(_) = grid.at(tx, ty) {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will eat plant at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        self.eaten.set(self.eaten.get()+1);
                    } else {
                        //println!("    Moving toward plant at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                    }
                    must_move = true;
                    break;
                }
            } else if self.eaten.get() >= EAT_TO_MATE && !self.mated.get() {
                if let CellContent::Animal(other) = grid.at(tx, ty) {
                    if !other.mated.get() {
                        new_x = ((self.x.get() as i32) + move_dx) as u32;
                        new_y = ((self.y.get() as i32) + move_dy) as u32;
                        if new_x == tx && new_y == ty {
                            //println!("    Will mate with animal at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                            new_animals = self.reproduce(&other, grid);
                        } else {
                            //println!("    Moving toward animal at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                        }
                        must_move = true;
                        break;
                    }
                }
            }
            /*
            match grid.at(tx, ty) {
                CellContent::Plant(_) => if self.eaten.get() < EAT_TO_MATE {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will eat plant at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        self.eaten.set(self.eaten.get()+1);
                    } else {
                        //println!("    Moving toward plant at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                    }
                    must_move = true;
                    break;
                },
                CellContent::Animal(other) => if self.eaten.get() >= EAT_TO_MATE && !self.mated.get() && !other.mated.get() {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will mate with animal at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        new_animals = self.reproduce(&other, grid);
                    } else {
                        //println!("    Moving toward animal at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                    }
                    must_move = true;
                    break;
                },
                _ => continue,
            }
            */
        }
        // If no move so far and still hungry or looking for a mate, just move as much as possible in one random diagonal direction
        if !must_move && !self.mated.get() {
            must_move = true;
            let dir = rand::thread_rng().gen_range(0, 4);
            let (dx, dy) = match dir {
                0 => {
                    (self.speed as i32, self.speed as i32)
                },
                1 => {
                    (-(self.speed as i32), self.speed as i32)
                },
                2 => {
                    (self.speed as i32, -(self.speed as i32))
                },
                3 => {
                    (-(self.speed as i32), -(self.speed as i32))
                },
                _ => panic!("Unexpected direction !"),
            };
            new_x = (self.x.get() as i32 + dx) as u32;
            new_y = (self.y.get() as i32 + dy) as u32;
            new_x = cmp::min(cmp::max(0, new_x), grid.width()-1);
            new_y = cmp::min(cmp::max(0, new_y), grid.height()-1);
        }
        if must_move {
            self.move_to(grid, new_x, new_y);
        }
        let mut result = vec!();
        for animal in new_animals {
            let animal = Rc::new(animal);
            grid.set_content(animal.x.get(), animal.y.get(), CellContent::Animal(Rc::clone(&animal)));
            result.push(animal);
        }
        result
    }

    pub fn finish_round(&self) {
        self.mated.set(false);
        self.eaten.set(0);
    }

    pub fn survive(&self) -> bool {
        // self.eaten.get() > 0
        // self.eaten.get() == EAT_TO_MATE
        self.mated.get()
    }

}

impl Animals {
    pub fn new(grid: &mut Grid, nb_animals: u32) -> Animals {
        let mut animals = vec!();
        for _ in 0..nb_animals {
            let (x, y) = grid.get_empty_cell();
            let new_animal = Rc::new(Animal::new(x, y));
            grid.set_content(x, y, CellContent::Animal(Rc::clone(&new_animal)));
            animals.push(new_animal);
        }
        Animals{animals}
    }

    pub fn update(&mut self, grid: &mut Grid) {
        let mut to_add = vec!();
        for animal in self.animals.iter() {
            to_add.append(&mut animal.update(grid));
        }
        self.animals.append(&mut to_add);
    }

    pub fn finish_round(&mut self, grid: &mut Grid) {
        // Remove animals which died of hunger
        for animal in self.animals.iter() {
            if !animal.survive() {
                animal.assert_animal(grid);
                grid.set_content(animal.x.get(), animal.y.get(), CellContent::Empty);
            }
        }
        self.animals.retain(|animal| animal.survive());

        // Reset remaining animals for next round
        for animal in self.animals.iter() {
            animal.finish_round();
        }
    }

    pub fn size(&self) -> usize {
        self.animals.len()
    }

    /*pub fn consistency_checks(&self) {
        for (i1, animal1) in self.animals.iter().enumerate() {
            for (i2, animal2) in self.animals.iter().enumerate() {
                if i1 != i2 {
                    assert!(animal1.x != animal2.x || animal1.y != animal2.y);
                }
            }
        }
    }*/

    pub fn stats(&self, stats: &mut StatsItem) {
        stats.nb_animals = self.animals.len() as u32;
        for _ in 0..=MAX_RANGE {
            stats.nb_animals_per_range.push(0);
        }
        for _ in 0..=MAX_SPEED {
            stats.nb_animals_per_speed.push(0);
        }
        for a in self.animals.iter() {
            stats.nb_animals_per_range[a.range as usize] += 1;
            stats.nb_animals_per_speed[a.speed as usize] += 1;
        }
        assert!(stats.nb_animals_per_range.iter().sum::<u32>() == stats.nb_animals);
        assert!(stats.nb_animals_per_speed.iter().sum::<u32>() == stats.nb_animals);
    }
}

struct RangeIterator {
    x: i32,
    y: i32,
    range: i32,
    d: i32,
    dx: i32,
    dy: i32,
}

impl RangeIterator {
    fn new(x: i32, y: i32, range: i32) -> RangeIterator {
        RangeIterator{x, y, range, d: 0, dx: 0, dy: 0}
    }
}

impl Iterator for RangeIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.dx < -self.range && self.dy < -self.range {
            None
        } else {
            let r = Some((self.x+self.dx, self.y+self.dy));
            if self.dx == self.d && self.dy == self.d {
                self.d += 1;
                self.dx = -self.d;
                self.dy = -self.d;
            } else if (self.dx == -self.d || self.dx == self.d) && self.dy < self.d {
                self.dy += 1;
            } else if self.dy == -self.d {
                self.dy = self.d;
            } else {
                self.dy = -self.d;
                self.dx += 1;
            }
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_iterator() {
        let mut rg = RangeIterator::new(10, 10, 3);

        // Distance 0
        assert_eq!(Some((10, 10)), rg.next());

        // Distance 1
        assert_eq!(Some((9, 9)), rg.next());
        assert_eq!(Some((9, 10)), rg.next());
        assert_eq!(Some((9, 11)), rg.next());

        assert_eq!(Some((10, 9)), rg.next());
        assert_eq!(Some((10, 11)), rg.next());

        assert_eq!(Some((11, 9)), rg.next());
        assert_eq!(Some((11, 10)), rg.next());
        assert_eq!(Some((11, 11)), rg.next());

        // Distance 2
        assert_eq!(Some((8, 8)), rg.next());
        assert_eq!(Some((8, 9)), rg.next());
        assert_eq!(Some((8, 10)), rg.next());
        assert_eq!(Some((8, 11)), rg.next());
        assert_eq!(Some((8, 12)), rg.next());

        assert_eq!(Some((9, 8)), rg.next());
        assert_eq!(Some((9, 12)), rg.next());
        assert_eq!(Some((10, 8)), rg.next());
        assert_eq!(Some((10, 12)), rg.next());
        assert_eq!(Some((11, 8)), rg.next());
        assert_eq!(Some((11, 12)), rg.next());

        assert_eq!(Some((12, 8)), rg.next());
        assert_eq!(Some((12, 9)), rg.next());
        assert_eq!(Some((12, 10)), rg.next());
        assert_eq!(Some((12, 11)), rg.next());
        assert_eq!(Some((12, 12)), rg.next());

        // Distance 3
        assert_eq!(Some((7, 7)), rg.next());
        assert_eq!(Some((7, 8)), rg.next());
        assert_eq!(Some((7, 9)), rg.next());
        assert_eq!(Some((7, 10)), rg.next());
        assert_eq!(Some((7, 11)), rg.next());
        assert_eq!(Some((7, 12)), rg.next());
        assert_eq!(Some((7, 13)), rg.next());

        assert_eq!(Some((8, 7)), rg.next());
        assert_eq!(Some((8, 13)), rg.next());
        assert_eq!(Some((9, 7)), rg.next());
        assert_eq!(Some((9, 13)), rg.next());
        assert_eq!(Some((10, 7)), rg.next());
        assert_eq!(Some((10, 13)), rg.next());
        assert_eq!(Some((11, 7)), rg.next());
        assert_eq!(Some((11, 13)), rg.next());
        assert_eq!(Some((12, 7)), rg.next());
        assert_eq!(Some((12, 13)), rg.next());

        assert_eq!(Some((13, 7)), rg.next());
        assert_eq!(Some((13, 8)), rg.next());
        assert_eq!(Some((13, 9)), rg.next());
        assert_eq!(Some((13, 10)), rg.next());
        assert_eq!(Some((13, 11)), rg.next());
        assert_eq!(Some((13, 12)), rg.next());
        assert_eq!(Some((13, 13)), rg.next());

        // Finished
        assert_eq!(None, rg.next());
    }
}
