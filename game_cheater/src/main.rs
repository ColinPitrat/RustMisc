mod options;

mod capybara_clicker;
mod chrome_dino;
mod idle_monster_slayer;
mod storm_the_house;

fn main() {
    let options : options::CommandLineOptions = argh::from_env();
    match options.game.as_str() {
        "capybara_clicker" => capybara_clicker::play_game(&options),
        "chrome_dino" => chrome_dino::play_game(&options),
        "idle_monster_slayer" => idle_monster_slayer::play_game(&options),
        "storm_the_house" => storm_the_house::play_game(&options),
        _ => println!("Unknown game '{}'", options.game),
    }
}
