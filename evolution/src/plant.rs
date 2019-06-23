extern crate rand;

use crate::grid::CellContent;
use crate::grid::Grid;
use crate::model::Model;
use crate::stats::StatsItem;
use rand::Rng;
use std::cmp;
use std::cell::Cell;
use std::rc::Rc;

pub struct Plant {
    x: u32,
    y: u32,
    layering: u32, // Up to how many child will grow next to it
    fertility: u32, // The higher, the more likely it is to disperse seeds and the more it will disperse
    spread: u32, // Up to how far the seeds will disperse
    keep: Cell<bool>,
}

pub struct Plants {
    plants: Vec<Rc<Plant>>,
}

impl Plant {
    pub fn new(x: u32, y: u32, model: &Model) -> Plant {
        let layering = rand::thread_rng().gen_range(0, model.plants_max_layering+1);
        let fertility = rand::thread_rng().gen_range(0, model.plants_max_fertility+1);
        let spread = rand::thread_rng().gen_range(0, model.plants_max_spread+1);
        let keep = Cell::new(true);
        Plant{x, y, layering, fertility, spread, keep}
    }

    pub fn layer(&self, x: u32, y: u32) -> Plant {
        Plant{x, y, layering: self.layering, fertility: self.fertility, spread: self.spread, keep: Cell::new(true)}
    }

    pub fn remove(&self) {
        self.keep.set(false);
    }

    pub fn mix_with(&self, partner: &Plant, x: u32, y: u32) -> Plant {
        let r = rand::thread_rng().gen_range(0, 1024);
        let layering = if r % 2 == 0 {
            self.layering
        } else {
            partner.layering
        };
        let fertility = if (r / 2) % 2 == 0 {
            self.fertility
        } else {
            partner.fertility
        };
        let spread = if (r / 4) % 2 == 0 {
            self.spread
        } else {
            partner.spread
        };
        Plant{x, y, layering, fertility, spread, keep: Cell::new(true)}
    }
}

impl Plants {
    pub fn new(grid: &mut Grid, model: &Model) -> Plants {
        let mut plants = vec!();
        for _ in 0..model.plants_at_start {
            let (x, y) = grid.get_empty_cell();
            let new_plant = Rc::new(Plant::new(x, y, model));
            grid.set_content(x, y, CellContent::Plant(Rc::clone(&new_plant)));
            plants.push(new_plant);
        }
        Plants{plants}
    }

    pub fn layer(&mut self, grid: &mut Grid) {
        let neighbours : Vec<(i32, i32)> = vec![ (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1) ];
        let mut to_add = vec!();
        for plant in self.plants.iter() {
            let mut layers = 0;
            for n in neighbours.iter() {
                if layers >= plant.layering {
                    break;
                }
                let x = plant.x as i32 + n.0;
                let y = plant.y as i32 + n.1;
                if x < 0 || y < 0 || x >= grid.width() as i32 || y >= grid.height() as i32 {
                    continue;
                }
                let x = x as u32;
                let y = y as u32;
                if grid.empty(x, y) {
                    let new_plant = Rc::new(plant.layer(x, y));
                    grid.set_content(x, y, CellContent::Plant(Rc::clone(&new_plant)));
                    to_add.push(new_plant);
                    layers += 1;
                }
            }
        }
        self.plants.append(&mut to_add);
    }

    pub fn spread(&mut self, grid: &mut Grid, model: &Model) {
        let mut to_add = vec!();
        for plant in self.plants.iter() {
            // The plant is not able to reproduce, no matter what
            if plant.fertility == 0 || plant.spread == 0 {
                continue
            }
            let threshold = rand::thread_rng().gen_range(0, model.plants_max_fertility);
            // The plant is not fertil enough to reproduce this round
            if plant.fertility < threshold {
                continue
            }
            let nb_seeds = rand::thread_rng().gen_range(0, plant.fertility);
            for _ in 0..nb_seeds {
                let min_x = cmp::max(0, plant.x as i32 - plant.spread as i32) as u32;
                let min_y = cmp::max(0, plant.y as i32 - plant.spread as i32) as u32;
                let max_x = cmp::min(plant.x + plant.spread, grid.width());
                let max_y = cmp::min(plant.y + plant.spread, grid.height());
                let new_x = rand::thread_rng().gen_range(min_x, max_x);
                let new_y = rand::thread_rng().gen_range(min_y, max_y);
                // The place where the seed would land is already occupied
                if ! grid.empty(new_x, new_y) {
                    continue;
                }
                let partner_idx = rand::thread_rng().gen_range(0, self.plants.len());
                let partner = &self.plants[partner_idx];
                let new_plant = Rc::new(plant.mix_with(partner, new_x, new_y));
                grid.set_content(new_x, new_y, CellContent::Plant(Rc::clone(&new_plant)));
                to_add.push(new_plant);
            }
        }
        self.plants.append(&mut to_add);
    }

    pub fn reproduce(&mut self, grid: &mut Grid, model: &Model) {
        self.layer(grid);
        self.spread(grid, model);
    }

    pub fn size(&self) -> usize {
        self.plants.len()
    }

    //pub fn remove(&mut self, x: u32, y: u32) {
        //self.plants.retain(|plant| plant.x != x || plant.y != y);
        //let it = self.plants.iter().position(|p| p.x == x && p.y == y).unwrap();
        //self.plants.remove(it);
    //}

    pub fn cleanup(&mut self) {
        self.plants.retain(|plant| plant.keep.get());
    }

    // This method is for debugging only, no need to alert for dead code.
    #[allow(dead_code)]
    pub fn consistency_checks(&self) {
        for (i1, plant1) in self.plants.iter().enumerate() {
            for (i2, plant2) in self.plants.iter().enumerate() {
                if i1 != i2 {
                    assert!(plant1.x != plant2.x || plant1.y != plant2.y);
                }
            }
        }
    }

    pub fn stats(&self, stats: &mut StatsItem, model: &Model) {
        stats.nb_plants = self.plants.len() as u32;
        for _ in 0..=model.plants_max_layering {
            stats.nb_plants_per_layering.push(0);
        }
        for _ in 0..=model.plants_max_fertility {
            stats.nb_plants_per_fertility.push(0);
        }
        for _ in 0..=model.plants_max_spread {
            stats.nb_plants_per_spread.push(0);
        }
        for p in self.plants.iter() {
            stats.nb_plants_per_layering[p.layering as usize] += 1;
            stats.nb_plants_per_fertility[p.fertility as usize] += 1;
            stats.nb_plants_per_spread[p.spread as usize] += 1;
        }
        assert!(stats.nb_plants_per_layering.iter().sum::<u32>() == stats.nb_plants);
        assert!(stats.nb_plants_per_fertility.iter().sum::<u32>() == stats.nb_plants);
        assert!(stats.nb_plants_per_spread.iter().sum::<u32>() == stats.nb_plants);
    }
}
