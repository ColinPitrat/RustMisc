use crate::animal::Animals;
use crate::plant::Plants;

pub struct StatsItem {
    pub nb_plants: u32,
    pub nb_plants_per_layering: Vec<u32>,
    pub nb_plants_per_fertility: Vec<u32>,
    pub nb_plants_per_spread: Vec<u32>,

    pub nb_animals: u32,
    pub nb_animals_per_range: Vec<u32>,
    pub nb_animals_per_speed: Vec<u32>,
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
        }
    }

    pub fn new(animals: &Animals, plants: &Plants) -> StatsItem {
        let mut stats = StatsItem::empty();
        plants.stats(&mut stats);
        animals.stats(&mut stats);
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

    pub fn update(&mut self, animals: &Animals, plants: &Plants) {
        self.stats.push(StatsItem::new(animals, plants));
    }
}

