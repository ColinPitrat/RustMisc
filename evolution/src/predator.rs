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

pub struct Predator {
    x: Cell<u32>,
    y: Cell<u32>,
    mated: Cell<bool>, // Whether this predator reproduced already
    energy: Cell<i32>, // How much energy the predator has left
    power: u32, // How much energy this predator consumes each round
    range: u32, // How far the predator can see
    speed: u32, // How far the predator can go at each step
}

pub struct Predators {
    predators: Vec<Rc<Predator>>,
}

impl Predator {
    pub fn new(x: u32, y: u32, model: &Model) -> Predator {
        let range = rand::thread_rng().gen_range(model.predators_min_range, model.predators_max_range+1);
        let speed = rand::thread_rng().gen_range(model.predators_min_speed, model.predators_max_speed+1);
        let power = model.predator_power(range, speed);
        Predator{
            x: Cell::new(x),
            y: Cell::new(y),
            mated: Cell::new(false),
            energy: Cell::new(model.predators_max_energy as i32),
            power, range, speed
        }
    }

    pub fn assert_predator(&self, grid: &Grid) {
        match grid.at(self.x.get(), self.y.get()) {
            CellContent::Predator(_) => {},
            CellContent::Animal(_) => {
                panic!("Cell {}, {} should contain a predator but contains an animal", self.x.get(), self.y.get());
            },
            CellContent::Plant(_) => {
                panic!("Cell {}, {} should contain a predator but contains a plant", self.x.get(), self.y.get());
            }
            CellContent::Empty => {
                panic!("Cell {}, {} should contain a predator but is empty", self.x.get(), self.y.get());
            }
        };
    }

    pub fn move_to(&self, grid: &mut Grid, x: u32, y: u32) {
        if x == self.x.get() && y == self.y.get() {
            return;
        }
        match grid.at(x, y) {
            CellContent::Predator(_) => {
                return;
            },
            CellContent::Animal(animal) => {
                animal.remove();
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
        self.assert_predator(grid);
        if let CellContent::Predator(my_rc) = grid.at(self.x.get(), self.y.get()) {
            let new_predator = CellContent::Predator(Rc::clone(my_rc));
            grid.set_content(x, y, new_predator);
        };
        grid.set_content(self.x.get(), self.y.get(), CellContent::Empty);
        self.x.set(x);
        self.y.set(y);
    }

    pub fn mix_with(&self, other: &Rc<Predator>, x: u32, y: u32, model: &Model) -> Predator {
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
        let power = model.predator_power(range, speed);
        Predator{
            x: Cell::new(x),
            y: Cell::new(y),
            mated: Cell::new(true),
            energy: Cell::new(model.predators_max_energy as i32),
            power, range, speed
        }
    }

    pub fn reproduce(&self, other: &Rc<Predator>, grid: &Grid, model: &Model) -> Vec<Predator> {
        self.mated.set(true);
        other.mated.set(true);
        let (mut new_x, mut new_y) = (-1, -1);
        for (x, y) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, 20, grid.width() as i32, grid.height() as i32) {
            match grid.at(x as u32, y as u32) {
                CellContent::Predator(_) => continue,
                CellContent::Animal(animal) => {
                    new_x = x;
                    new_y = y;
                    //println!("Remove animal at {}, {}", new_x, new_y);
                    animal.remove();
                    break;
                },
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
            //println!("New predator at {}, {}", new_x, new_y);
            result.push(self.mix_with(other, new_x as u32, new_y as u32, model));
        }
        result
    }

    pub fn update(&self, grid: &mut Grid, model: &Model) -> Vec<Rc<Predator>> {
        let (mut new_x, mut new_y) = (self.x.get(), self.y.get());
        let mut must_move = false;
        let mut new_predators = vec!();
        for (tx, ty) in RangeIterator::new(self.x.get() as i32, self.y.get() as i32, self.range as i32, grid.width() as i32, grid.height() as i32) {
            let (dx, dy) = (tx - self.x.get() as i32, ty - self.y.get() as i32);
            if dx == 0 && dy == 0 {
                continue;
            }
            // The predator cannot necessarily move as far in one round, this are the capped value in
            // this direction.
            let move_dx = if dx > 0 { cmp::min(self.speed as i32, dx) } else { cmp::max(-(self.speed as i32), dx) };
            let move_dy = if dy > 0 { cmp::min(self.speed as i32, dy) } else { cmp::max(-(self.speed as i32), dy) };
            let (tx, ty) = (tx as u32, ty as u32);
            if self.energy.get() < model.predators_max_energy as i32 {
                if let CellContent::Animal(_) = grid.at(tx, ty) {
                    new_x = ((self.x.get() as i32) + move_dx) as u32;
                    new_y = ((self.y.get() as i32) + move_dy) as u32;
                    if new_x == tx && new_y == ty {
                        //println!("    Will eat animal at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                        self.energy.set(self.energy.get()+model.predators_energy_per_prey as i32);
                    } else {
                        //println!("    Moving toward animal at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                    }
                    must_move = true;
                    break;
                }
            } else if !self.mated.get() {
                if let CellContent::Predator(other) = grid.at(tx, ty) {
                    if !other.mated.get() {
                        new_x = ((self.x.get() as i32) + move_dx) as u32;
                        new_y = ((self.y.get() as i32) + move_dy) as u32;
                        if new_x == tx && new_y == ty {
                            //println!("    Will mate with predator at {}, {} from {}, {}.", new_x, new_y, self.x.get(), self.y.get());
                            new_predators = self.reproduce(&other, grid, model);
                        } else {
                            //println!("    Moving toward predator at {}, {} from {}, {} through {}, {}.", tx, ty, self.x.get(), self.y.get(), new_x, new_y);
                        }
                        must_move = true;
                        break;
                    }
                }
            }
        }
        // If no move so far, just move as much as possible in one random diagonal direction
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
        for predator in new_predators {
            let predator = Rc::new(predator);
            grid.set_content(predator.x.get(), predator.y.get(), CellContent::Predator(Rc::clone(&predator)));
            result.push(predator);
        }
        result
    }

    pub fn finish_round(&self) {
        self.mated.set(false);
    }

    pub fn consume_energy(&self) {
        self.energy.set(self.energy.get()-self.power as i32);
    }

    pub fn survive(&self) -> bool {
        self.energy.get() > 0
    }
}

impl Predators {
    pub fn new(grid: &mut Grid, model: &Model) -> Predators {
        let mut predators = vec!();
        for _ in 0..model.predators_at_start {
            let (x, y) = grid.get_empty_cell();
            let new_predator = Rc::new(Predator::new(x, y, model));
            grid.set_content(x, y, CellContent::Predator(Rc::clone(&new_predator)));
            predators.push(new_predator);
        }
        Predators{predators}
    }

    pub fn update(&mut self, grid: &mut Grid, model: &Model) {
        let mut to_add = vec!();
        for predator in self.predators.iter() {
            to_add.append(&mut predator.update(grid, model));
        }
        self.predators.append(&mut to_add);
    }

    pub fn finish_round(&mut self, grid: &mut Grid) {
        // Remove predators which died of hunger
        for predator in self.predators.iter_mut() {
            predator.consume_energy();
            if !predator.survive() {
                predator.assert_predator(grid);
                grid.set_content(predator.x.get(), predator.y.get(), CellContent::Empty);
            }
        }
        self.predators.retain(|predator| predator.survive());

        // Reset remaining predators for next round
        for predator in self.predators.iter() {
            predator.finish_round();
        }
    }

    pub fn size(&self) -> usize {
        self.predators.len()
    }

    // This method is for debugging only, no need to alert for dead code.
    #[allow(dead_code)]
    pub fn consistency_checks(&self) {
        for (i1, predator1) in self.predators.iter().enumerate() {
            for (i2, predator2) in self.predators.iter().enumerate() {
                if i1 != i2 {
                    assert!(predator1.x != predator2.x || predator1.y != predator2.y);
                }
            }
        }
    }

    pub fn stats(&self, stats: &mut StatsItem, model: &Model) {
        stats.nb_predators = self.predators.len() as u32;
        for _ in model.predators_min_range..=model.predators_max_range {
            stats.nb_predators_per_range.push(0);
        }
        for _ in model.predators_min_speed..=model.predators_max_speed {
            stats.nb_predators_per_speed.push(0);
        }
        for _ in model.predators_min_power()..=model.predators_max_power() {
            stats.nb_predators_per_power.push(0);
        }
        for a in self.predators.iter() {
            stats.nb_predators_per_range[(a.range-model.predators_min_range) as usize] += 1;
            stats.nb_predators_per_speed[(a.speed-model.predators_min_speed) as usize] += 1;
            stats.nb_predators_per_power[(a.power-model.predators_min_power()) as usize] += 1;
        }
        assert!(stats.nb_predators_per_range.iter().sum::<u32>() == stats.nb_predators);
        assert!(stats.nb_predators_per_speed.iter().sum::<u32>() == stats.nb_predators);
        assert!(stats.nb_predators_per_power.iter().sum::<u32>() == stats.nb_predators);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mix_with() {
        let model = Model::new();
        let pred1 = Predator::new(0, 0, &model);
        let pred2 = Rc::new(Predator::new(1, 0, &model));

        let child = pred1.mix_with(&pred2, 0, 1, &model);

        assert!(child.energy.get() == model.predators_max_energy as i32);
        assert!(child.range == pred1.range || child.range == pred2.range);
        assert!(child.speed == pred1.speed || child.speed == pred2.speed);
    }

    #[test]
    fn finish_round() {
        let model = Model::new();
        let pred = Predator::new(0, 0, &model);
        pred.mated.set(true);

        pred.finish_round();

        assert_eq!(pred.mated.get(), false);
    }
}
