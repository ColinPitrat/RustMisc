pub struct Model {
    pub screen_width : u32,
    pub screen_height : u32,
    pub cell_width : u32,
    pub steps_per_round : u32,

    pub plants_at_start : u32,
    pub plants_max_layering : u32,
    pub plants_max_fertility : u32,
    pub plants_max_spread : u32,

    pub animals_at_start : u32,
    pub animals_min_speed : u32,
    pub animals_max_speed : u32,
    pub animals_min_range : u32,
    pub animals_max_range : u32,
    pub animals_eat_to_mate : u32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            screen_width: 2000,
            screen_height: 1400,
            cell_width: 5,
            steps_per_round: 5,

            plants_at_start: 4000,
            plants_max_layering: 4,
            plants_max_fertility: 3,
            plants_max_spread: 100,

            animals_at_start: 800,
            animals_min_speed: 1,
            animals_max_speed: 4,
            animals_min_range: 1,
            animals_max_range: 4,
            animals_eat_to_mate: 3,
        }
    }

    pub fn grid_width(&self) -> u32 {
        self.screen_width/self.cell_width
    }

    pub fn grid_height(&self) -> u32 {
        self.screen_height/self.cell_width
    }
}
