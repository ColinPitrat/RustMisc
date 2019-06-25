extern crate rand;

use crate::grid::CellContent;
use crate::grid::Grid;
use crate::range_iterator::RangeIterator;
use crate::model::Model;
use crate::stats::StatsItem;
use rand::Rng;
use std::cmp;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Animal {
    x: Cell<u32>,
    y: Cell<u32>,
    mated: Cell<bool>, // Whether this animal reproduced already
    eaten: Cell<u32>, // How much the animal ate in this round
    range: u32, // How far the animal can see
    speed: u32, // How far the animal can go at each step
    keep: Cell<bool>,
}

pub struct Animals {
    animals: Vec<Rc<Animal>>,
}

impl Animal {
    pub fn new(x: u32, y: u32, model: &Model) -> Animal {
        let range = rand::thread_rng().gen_range(model.animals_min_range, model.animals_max_range+1);
        let speed = rand::thread_rng().gen_range(model.animals_min_speed, model.animals_max_speed+1);
        Animal{
            x: Cell::new(x),
            y: Cell::new(y),
            mated: Cell::new(false),
            eaten: Cell::new(0),
            range, speed,
            keep: Cell::new(true),
        }
    }

    pub fn remove(&self) {
        self.keep.set(false);
    }

    pub fn assert_animal(&self, grid: &Grid) {
        match grid.at(self.x.get(), self.y.get()) {
            CellContent::Predator(_) => {
                panic!("Cell {}, {} should contain an animal but contains a predator", self.x.get(), self.y.get());
            },
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
            CellContent::Predator(_) => {
                //println!("There's a predator at {}, {} - not moving.", x, y);
                return;
            },
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

    pub fn mix_with(&self, other: &Rc<Animal>, x: u32, y: u32, model: &Model) -> Animal {
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
            eaten: Cell::new(model.animals_eat_to_mate),
            range, speed,
            keep: Cell::new(true),
        }
    }

    pub fn reproduce(&self, other: &Rc<Animal>, grid: &Grid, model: &Model) -> Vec<Animal> {
        self.mated.set(true);
        other.mated.set(true);
        let (mut new_x, mut new_y) = (-1, -1);
        for (x, y) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, 20, grid.width() as i32, grid.height() as i32) {
            match grid.at(x as u32, y as u32) {
                CellContent::Predator(_) => continue,
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
            result.push(self.mix_with(other, new_x as u32, new_y as u32, model));
        }
        result
    }

    // TODO: Animals should try to escape predators
    pub fn update(&self, grid: &mut Grid, model: &Model) -> Vec<Rc<Animal>> {
        let (mut new_x, mut new_y) = (self.x.get(), self.y.get());
        let mut must_move = false;
        let mut new_animals = vec!();
        for (tx, ty) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, self.range as i32, grid.width() as i32, grid.height() as i32) {
            let (dx, dy) = (tx - self.x.get() as i32, ty - self.y.get() as i32);
            if dx == 0 && dy == 0 {
                continue;
            }
            // The animal cannot necessarily move as far in one round, this are the capped value in
            // this direction.
            let move_dx = if dx > 0 { cmp::min(self.speed as i32, dx) } else { cmp::max(-(self.speed as i32), dx) };
            let move_dy = if dy > 0 { cmp::min(self.speed as i32, dy) } else { cmp::max(-(self.speed as i32), dy) };
            let (tx, ty) = (tx as u32, ty as u32);
            if self.eaten.get() < model.animals_eat_to_mate {
                if let CellContent::Plant(_) = grid.at(tx, ty) {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will eat plant at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        //println!("    Will eat plant at {}, {}.", new_x, new_y);
                        self.eaten.set(self.eaten.get()+1);
                    } else {
                        //println!("    Moving toward plant at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                    }
                    must_move = true;
                    break;
                }
            } else if !self.mated.get() {
                if let CellContent::Animal(other) = grid.at(tx, ty) {
                    if !other.mated.get() {
                        new_x = ((self.x.get() as i32) + move_dx) as u32;
                        new_y = ((self.y.get() as i32) + move_dy) as u32;
                        if new_x == tx && new_y == ty {
                            //println!("    Will mate with animal at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                            new_animals = self.reproduce(&other, grid, model);
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
                CellContent::Plant(_) => if self.eaten.get() < model.animals_eat_to_mate {
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
                CellContent::Animal(other) => if self.eaten.get() >= model.animals_eat_to_mate && !self.mated.get() && !other.mated.get() {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will mate with animal at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        new_animals = self.reproduce(&other, grid, model);
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
            let nx = self.x.get() as i32 + dx;
            let ny = self.y.get() as i32 + dy;
            new_x = cmp::min(cmp::max(0, nx) as u32, grid.width()-1);
            new_y = cmp::min(cmp::max(0, ny) as u32, grid.height()-1);
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
        // self.eaten.get() == model.animals_eat_to_mate
        self.mated.get()
    }

}

impl Animals {
    pub fn new(grid: &mut Grid, model: &Model) -> Animals {
        let mut animals = vec!();
        for _ in 0..model.animals_at_start {
            if let Some((x, y)) = grid.get_empty_cell() {
                let new_animal = Rc::new(Animal::new(x, y, model));
                grid.set_content(x, y, CellContent::Animal(Rc::clone(&new_animal)));
                animals.push(new_animal);
            } else {
                break;
            }
        }
        Animals{animals}
    }

    pub fn update(&mut self, grid: &mut Grid, model: &Model) {
        let mut to_add = vec!();
        for animal in self.animals.iter() {
            to_add.append(&mut animal.update(grid, model));
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

    pub fn cleanup(&mut self) {
        self.animals.retain(|animal| animal.keep.get());
    }

    // This method is for debugging only, no need to alert for dead code.
    #[allow(dead_code)]
    pub fn consistency_checks(&self) {
        for (i1, animal1) in self.animals.iter().enumerate() {
            for (i2, animal2) in self.animals.iter().enumerate() {
                if i1 != i2 {
                    assert!(animal1.x != animal2.x || animal1.y != animal2.y);
                }
            }
        }
    }

    pub fn stats(&self, stats: &mut StatsItem, model: &Model) {
        stats.nb_animals = self.animals.len() as u32;
        for _ in model.animals_min_range..=model.animals_max_range {
            stats.nb_animals_per_range.push(0);
        }
        for _ in model.animals_min_speed..=model.animals_max_speed {
            stats.nb_animals_per_speed.push(0);
        }
        for a in self.animals.iter() {
            stats.nb_animals_per_range[(a.range-model.animals_min_range) as usize] += 1;
            stats.nb_animals_per_speed[(a.speed-model.animals_min_speed) as usize] += 1;
        }
        assert!(stats.nb_animals_per_range.iter().sum::<u32>() == stats.nb_animals);
        assert!(stats.nb_animals_per_speed.iter().sum::<u32>() == stats.nb_animals);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plant::Plant;

    #[test]
    fn eat_plant() {
        let mut grid = Grid::new(3, 3, 1);
        let mut model = Model::new();
        model.animals_min_range = 1;
        model.animals_max_range = 1;
        model.animals_min_speed = 1;
        model.animals_max_speed = 1;
        model.animals_eat_to_mate = 1;
        let animal1 = Rc::new(Animal::new(0, 0, &model));
        grid.set_content(0, 0, CellContent::Animal(Rc::clone(&animal1)));
        let animal2 = Rc::new(Animal::new(0, 2, &model));
        grid.set_content(0, 2, CellContent::Animal(Rc::clone(&animal2)));
        let plant = Rc::new(Plant::new(0, 1, &model));
        grid.set_content(0, 1, CellContent::Plant(Rc::clone(&plant)));

        assert_matches!(grid.at(0, 0), CellContent::Animal(_));
        assert_matches!(grid.at(0, 1), CellContent::Plant(_));
        assert_matches!(grid.at(0, 2), CellContent::Animal(_));
        animal1.update(&mut grid, &model);
        assert_matches!(grid.at(0, 0), CellContent::Empty);
        assert_matches!(grid.at(0, 1), CellContent::Animal(_));
        assert_matches!(grid.at(0, 2), CellContent::Animal(_));
        animal2.update(&mut grid, &model);
        assert_matches!(grid.at(0, 0), CellContent::Empty);
        assert_matches!(grid.at(0, 1), CellContent::Animal(_));
        // Note: at this point, animal2 could have moved

        // The first animal ate the plant, the second didn't
        assert_eq!(animal1.eaten.get(), 1);
        assert_eq!(animal2.eaten.get(), 0);
    }
}
