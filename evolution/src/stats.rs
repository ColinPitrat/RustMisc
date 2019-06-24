use crate::animal::Animals;
use crate::model::Model;
use crate::plant::Plants;
use crate::predator::Predators;

pub struct StatsItem {
    pub nb_plants: u32,
    pub nb_plants_per_layering: Vec<u32>,
    pub nb_plants_per_fertility: Vec<u32>,
    pub nb_plants_per_spread: Vec<u32>,

    pub nb_animals: u32,
    pub nb_animals_per_range: Vec<u32>,
    pub nb_animals_per_speed: Vec<u32>,

    pub nb_predators: u32,
    pub nb_predators_per_range: Vec<u32>,
    pub nb_predators_per_speed: Vec<u32>,
    pub nb_predators_per_power: Vec<u32>,
}

impl StatsItem {
    pub fn empty() -> StatsItem {
        StatsItem {
            nb_plants: 0,
            nb_plants_per_layering: vec!(),
            nb_plants_per_fertility: vec!(),
            nb_plants_per_spread: vec!(),
            nb_animals: 0,
            nb_animals_per_range: vec!(),
            nb_animals_per_speed: vec!(),
            nb_predators: 0,
            nb_predators_per_range: vec!(),
            nb_predators_per_speed: vec!(),
            nb_predators_per_power: vec!(),
        }
    }

    pub fn new(predators: &Predators, animals: &Animals, plants: &Plants, model: &Model) -> StatsItem {
        let mut stats = StatsItem::empty();
        plants.stats(&mut stats, model);
        animals.stats(&mut stats, model);
        predators.stats(&mut stats, model);
        stats
    }
}

pub struct Stats {
    pub stats: Vec<StatsItem>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats{
            stats: vec!()
        }
    }

    pub fn update(&mut self, predators: &Predators, animals: &Animals, plants: &Plants, model: &Model) {
        self.stats.push(StatsItem::new(predators, animals, plants, model));
    }
}

