extern crate chrono;
extern crate clap;
extern crate sdl2;

mod animal;
mod dc;
mod graph;
mod grid;
mod model;
mod plant;
mod stats;

use animal::Animals;
use chrono::Local;
use clap::{Arg,App};
use dc::DrawingContext;
use graph::Graph;
use grid::{CellContent,Grid};
use model::Model;
use plant::Plants;
use stats::Stats;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;
use std::string::ToString;
use std::path::Path;

// This method is for debugging only, no need to alert for dead code.
#[allow(dead_code)]
fn consistency_checks(animals: &Animals, plants: &Plants, grid: &Grid) {
    let mut nb_animals : u32 = 0;
    let mut nb_plants : u32 = 0;
    let mut nb_empty : u32 = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            match grid.at(x, y) {
                CellContent::Plant(_) => nb_plants += 1,
                CellContent::Animal(_) => nb_animals += 1,
                CellContent::Empty => nb_empty += 1,
            }
        }
    }
    plants.consistency_checks();
    animals.consistency_checks();
    println!("{} plants in the grid, {} plants in the list", nb_plants, plants.size() as u32);
    println!("{} animals in the grid, {} animals in the list", nb_animals, animals.size() as u32);
    println!("{} cells in the grid, {} cells checked", grid.width()*grid.height(), nb_empty+nb_animals+nb_plants);
    assert!(animals.size() as u32 == nb_animals);
    assert!(plants.size() as u32 == nb_plants);
    assert!((grid.width()*grid.height()) == nb_empty + nb_animals + nb_plants);
}

fn dump_stats(stats: &Stats, path: &Path) {
    let mut file = File::create(path).unwrap();
    let mut header = String::from("plants");
    for l in 0..stats.stats[0].nb_plants_per_layering.len() {
        header += &format!(",layering={}", l);
    }
    for f in 0..stats.stats[0].nb_plants_per_fertility.len() {
        header += &format!(",fertility={}", f);
    }
    for s in 0..stats.stats[0].nb_plants_per_spread.len() {
        header += &format!(",spread={}", s);
    }
    header += ",animals";
    for r in 0..stats.stats[0].nb_animals_per_range.len() {
        header += &format!(",range={}", r);
    }
    for s in 0..stats.stats[0].nb_animals_per_speed.len() {
        header += &format!(",speed={}", s);
    }
    header += "\n";
    file.write_all(header.as_bytes()).unwrap();
    for si in stats.stats.iter() {
        let mut line = format!("{}", si.nb_plants);
        for l in si.nb_plants_per_layering.iter() {
            line += &format!(",{}", l);
        }
        for f in si.nb_plants_per_fertility.iter() {
            line += &format!(",{}", f);
        }
        for s in si.nb_plants_per_spread.iter() {
            line += &format!(",{}", s);
        }
        line += &format!(",{}", si.nb_animals);
        for r in si.nb_animals_per_range.iter() {
            line += &format!(",{}", r);
        }
        for s in si.nb_animals_per_speed.iter() {
            line += &format!(",{}", s);
        }
        line += "\n";
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn graph_data(stats: &Stats) -> Vec<Vec<u32>> {
    let mut result = vec![vec!(), vec!()];
    for si in stats.stats.iter() {
        result[0].push(si.nb_plants);
        result[1].push(si.nb_animals);
    }
    result
}

fn is_type<T: FromStr>(val: String) -> Result<(), String>
where <T as std::str::FromStr>::Err : std::string::ToString
{
    match val.parse::<T>() {
        Ok(_) => Ok(()),
        Err(m) => Err(m.to_string()),
    }
}

// TODO: Curves for population per trait.
// TODO: Dump curves in result subdirectory for each run.
// TODO: Loop on various models and compare results
// TODO: Add predators
fn main() {
    let matches = App::new("Evolution")
        .version("0.1")
        .author("Colin Pitrat")
        .about("Simulates evolution.")
        .arg(Arg::with_name("model")
                .short("m")
                .long("model")
                .value_name("FILE")
                .help("Defines which model to use.")
                .takes_value(true))
        .arg(Arg::with_name("rounds")
                .short("r")
                .long("rounds")
                .value_name("NUMBER")
                .help("If provided, stop automatically after this number of rounds.")
                .takes_value(true)
                .validator(is_type::<u32>))
        .get_matches();
    let model = match matches.value_of("model") {
        None => Model::new(),
        Some(filename) => Model::load(Path::new(filename)),
    };
    let max_rounds = match matches.value_of("rounds") {
        None => std::u32::MAX,
        // TODO: The message from expect is not great, better handling would be nice.
        // Unfortunately, support of types in clap is not great ...
        Some(num) => num.parse::<u32>().unwrap(),
    };
    let mut show_graph = false;
    let dump_screenshots = false;
    let mut dc = DrawingContext::new(model.screen_width, model.screen_height);
    let mut grid = Grid::new(model.grid_width(), model.grid_height(), model.cell_width);
    let mut plants = Plants::new(&mut grid, &model);
    let mut animals = Animals::new(&mut grid, &model);
    let mut stats = Stats::new();
    let mut graph = Graph::new(graph_data(&stats));

    let mut event_pump = dc.sdl_context.event_pump().unwrap();

    let run_name = Local::now().format("%Y-%m-%d_%H:%M:%S");
    let _ = fs::create_dir("results/");  // Can already exist
    let results_dir = format!("results/{}", run_name);
    fs::create_dir(&results_dir).unwrap();
    let result_path = |filename: &str| -> String {
        format!("{}/{}", results_dir, filename)
    };
    model.save(Path::new(&result_path("model.json")));
    if dump_screenshots {
        fs::create_dir(Path::new(&result_path("screenshots"))).unwrap();
    }
    let mut step = 0;
    'game_loop: loop {
        grid.show(&mut dc);
        if show_graph {
            graph.set_data(graph_data(&stats));
            graph.show(&mut dc);
        } else {
            dc.blit_grid();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    step += 1;
                    animals.update(&mut grid, &model);
                    plants.cleanup();
                    if step % model.steps_per_round == 0 {
                        plants.reproduce(&mut grid, &model);
                        animals.finish_round(&mut grid);
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    show_graph = !show_graph;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new(model.grid_width(), model.grid_height(), model.cell_width);
                    plants = Plants::new(&mut grid, &model);
                    animals = Animals::new(&mut grid, &model);
                },
                _ => {},
            }
        }

        {
            if step % model.steps_per_round == 0 {
                stats.update(&animals, &plants, &model);
            }
            step += 1;
            animals.update(&mut grid, &model);
            plants.cleanup();
            if step % model.steps_per_round == 0 {
                plants.reproduce(&mut grid, &model);
                animals.finish_round(&mut grid);
                if dump_screenshots {
                    // TODO: The following doesn't work. Try to reproduce in a minimal example and open an
                    // issue to sdl2 on github.
                    //dc.canvas.window().surface(&event_pump).unwrap().save_bmp(Path::new(&result_path(&format!("screenshots/{:06}.bmp", step)))).unwrap();
                    dc.save_grid_png(Path::new(&result_path(&format!("screenshots/{:06}.png", step/model.steps_per_round))));
                }
            }
        }

        dc.canvas.present();

        if step/model.steps_per_round >= max_rounds {
            break 'game_loop;
        }
    }

    dump_stats(&stats, Path::new(&result_path("stats.csv")));
}
