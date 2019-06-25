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
    pub animals_max_energy : u32,
    pub animals_energy_per_plant : u32,
    pub animals_power_per_speed : f64,
    pub animals_power_per_range : f64,

    pub predators_at_start : u32,
    pub predators_min_speed : u32,
    pub predators_max_speed : u32,
    pub predators_min_range : u32,
    pub predators_max_range : u32,
    pub predators_max_energy : u32,
    pub predators_energy_per_prey : u32,
    pub predators_power_per_speed : f64,
    pub predators_power_per_range : f64,
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
            animals_max_energy: 3,
            animals_energy_per_plant: 1,
            animals_power_per_speed: 0.1,
            animals_power_per_range: 0.1,

            predators_at_start: 200,
            predators_min_speed: 1,
            predators_max_speed: 10,
            predators_min_range: 1,
            predators_max_range: 10,
            predators_max_energy: 8,
            predators_energy_per_prey: 1,
            predators_power_per_speed: 0.2,
            predators_power_per_range: 0.2,
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

    pub fn animal_power(&self, range: u32, speed: u32) -> u32 {
        (self.animals_power_per_range*range as f64 + self.animals_power_per_speed*speed as f64 + 1.0) as u32
    }

    pub fn animals_min_power(&self) -> u32 {
        self.animal_power(self.animals_min_speed, self.animals_min_range)
    }

    pub fn animals_max_power(&self) -> u32 {
        self.animal_power(self.animals_max_speed, self.animals_max_range)
    }

    pub fn predator_power(&self, range: u32, speed: u32) -> u32 {
        (self.predators_power_per_range*range as f64 + self.predators_power_per_speed*speed as f64 + 1.0) as u32
    }

    pub fn predators_min_power(&self) -> u32 {
        self.predator_power(self.predators_min_speed, self.predators_min_range)
    }

    pub fn predators_max_power(&self) -> u32 {
        self.predator_power(self.predators_max_speed, self.predators_max_range)
    }
}
