use serde::{Serialize,Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub screen_width : u32,
    pub screen_height : u32,
    pub cell_width : u32,
    pub steps_per_round : u32,

    pub plants_at_start : u32,
    pub plants_spontaneous_per_round : u32,
    pub plants_min_layering : u32,
    pub plants_max_layering : u32,
    pub plants_min_fertility : u32,
    pub plants_max_fertility : u32,
    pub plants_min_spread : u32,
    pub plants_max_spread : u32,

    pub animals_at_start : u32,
    pub animals_min_speed : u32,
    pub animals_max_speed : u32,
    pub animals_min_range : u32,
    pub animals_max_range : u32,
    pub animals_eat_to_mate : u32,

    pub predators_at_start : u32,
    pub predators_min_speed : u32,
    pub predators_max_speed : u32,
    pub predators_min_range : u32,
    pub predators_max_range : u32,
    pub predators_power_factor : u32,
    pub predators_max_energy : u32,
    pub predators_energy_per_prey : u32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            screen_width: 2000,
            screen_height: 1400,
            cell_width: 5,
            steps_per_round: 5,

            plants_at_start: 4000,
            plants_spontaneous_per_round: 0,
            plants_min_layering: 0,
            plants_max_layering: 4,
            plants_min_fertility: 0,
            plants_max_fertility: 3,
            plants_min_spread: 0,
            plants_max_spread: 100,

            animals_at_start: 1600,
            animals_min_speed: 1,
            animals_max_speed: 5,
            animals_min_range: 1,
            animals_max_range: 5,
            animals_eat_to_mate: 3,

            predators_at_start: 200,
            predators_min_speed: 1,
            predators_max_speed: 10,
            predators_min_range: 1,
            predators_max_range: 10,
            predators_power_factor: 3,
            predators_max_energy: 8,
            predators_energy_per_prey: 1,
        }
    }

    pub fn load(path: &Path) -> Model {
        serde_json::from_str(&fs::read_to_string(path).expect(&format!("Unable to read file {:?}.", path))).unwrap()
    }

    pub fn save(&self, path: &Path) {
        fs::write(path, serde_json::to_string_pretty(&self).unwrap()).expect(&format!("Unable to write file {:?}.", path));
    }

    pub fn grid_width(&self) -> u32 {
        self.screen_width/self.cell_width
    }

    pub fn grid_height(&self) -> u32 {
        self.screen_height/self.cell_width
    }

    pub fn predator_power(&self, range: u32, speed: u32) -> u32 {
        range+speed/self.predators_power_factor + 1
    }

    pub fn predators_min_power(&self) -> u32 {
        self.predator_power(self.predators_min_speed, self.predators_min_range)
    }

    pub fn predators_max_power(&self) -> u32 {
        self.predator_power(self.predators_max_speed, self.predators_max_range)
    }
}
