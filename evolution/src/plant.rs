extern crate rand;

use crate::grid::Grid;
use rand::Rng;
use sdl2::pixels::Color;
use std::cmp;

const MAX_LAYERING : u32 = 8;
const MAX_FERTILITY : u32 = 8;
const MAX_SPREAD : u32 = 20;

pub struct Plant {
    x: u32,
    y: u32,
    layering: u32, // Up to how many child will grow next to it
    fertility: u32, // The higher, the more likely it is to disperse seeds and the more it will disperse
    spread: u32, // Up to how far the seeds will disperse
}

pub struct Plants {
    plants: Vec<Plant>,
}

impl Plant {
    pub fn new(x: u32, y: u32) -> Plant {
        let layering = rand::thread_rng().gen_range(0, MAX_LAYERING);
        let fertility = rand::thread_rng().gen_range(0, MAX_FERTILITY);
        let spread = rand::thread_rng().gen_range(0, MAX_SPREAD);
        Plant{x, y, layering, fertility, spread}
    }

    pub fn to_grid(&self, grid: &mut Grid) {
        let green = Color::RGB(0, 255, 0);
        grid.set_color(self.x, self.y, green);
    }

    pub fn layer(&self, x: u32, y: u32) -> Plant {
        Plant{x, y, layering: self.layering, fertility: self.fertility, spread: self.spread}
    }

    pub fn mix_with(&self, partner: &Plant, x: u32, y: u32) -> Plant {
        let r = rand::thread_rng().gen_range(0, 1024);
        let layering = if r % 2 == 0 {
            self.layering
        } else {
            partner.layering
        };
        let fertility = if r % 2 == 0 {
            self.fertility
        } else {
            partner.fertility
        };
        let spread = if r % 2 == 0 {
            self.spread
        } else {
            partner.spread
        };
        Plant{x, y, layering, fertility, spread}
    }
}

impl Plants {
    pub fn new(grid: &mut Grid, nb_plants: u32) -> Plants {
        let mut plants = vec!();
        for _ in 0..nb_plants {
            let (mut x, mut y);
            loop {
                x = rand::thread_rng().gen_range(0, grid.width());
                y = rand::thread_rng().gen_range(0, grid.height());
                if grid.empty(x, y) {
                    break;
                }
            }
            let new_plant = Plant::new(x, y);
            new_plant.to_grid(grid);
            plants.push(new_plant);
        }
        Plants{plants}
    }

    pub fn to_grid(&self, grid: &mut Grid) {
        for plant in self.plants.iter() {
            plant.to_grid(grid);
        }
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
                    let new_plant = plant.layer(x, y);
                    new_plant.to_grid(grid);
                    to_add.push(new_plant);
                    layers += 1;
                }
            }
        }
        self.plants.append(&mut to_add);
    }

    pub fn spread(&mut self, grid: &mut Grid) {
        let mut to_add = vec!();
        for plant in self.plants.iter() {
            // The plant is not able to reproduce, no matter what
            if plant.fertility == 0 || plant.spread == 0 {
                continue
            }
            let threshold = rand::thread_rng().gen_range(0, MAX_FERTILITY);
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
                let new_plant = plant.mix_with(partner, new_x, new_y);
                new_plant.to_grid(grid);
                to_add.push(new_plant);
            }
        }
        self.plants.append(&mut to_add);
    }

    pub fn reproduce(&mut self, grid: &mut Grid) {
        self.layer(grid);
        self.spread(grid);
    }
}
