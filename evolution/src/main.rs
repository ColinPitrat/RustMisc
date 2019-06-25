extern crate chrono;
extern crate clap;
extern crate num_derive;
extern crate num_traits;
extern crate sdl2;

#[cfg(test)]
#[macro_use]
extern crate matches;

mod animal;
mod dc;
mod graph;
mod grid;
mod model;
mod plant;
mod predator;
mod range_iterator;
mod stats;

use animal::Animals;
use chrono::Local;
use clap::{Arg,App};
use dc::DrawingContext;
use graph::Graph;
use grid::{CellContent,Grid};
use model::Model;
use plant::Plants;
use predator::Predators;
use stats::Stats;

use num_traits::FromPrimitive;
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
fn consistency_checks(predators: &Predators, animals: &Animals, plants: &Plants, grid: &Grid) {
    let mut nb_animals : u32 = 0;
    let mut nb_plants : u32 = 0;
    let mut nb_predators : u32 = 0;
    let mut nb_empty : u32 = 0;
    for x in 0..grid.width() {
        for y in 0..grid.height() {
            match grid.at(x, y) {
                CellContent::Plant(_) => nb_plants += 1,
                CellContent::Animal(_) => nb_animals += 1,
                CellContent::Predator(_) => nb_predators += 1,
                CellContent::Empty => nb_empty += 1,
            }
        }
    }
    plants.consistency_checks();
    animals.consistency_checks();
    predators.consistency_checks();
    println!("{} plants in the grid, {} plants in the list", nb_plants, plants.size() as u32);
    println!("{} animals in the grid, {} animals in the list", nb_animals, animals.size() as u32);
    println!("{} predators in the grid, {} predators in the list", nb_predators, predators.size() as u32);
    println!("{} cells in the grid, {} cells checked", grid.width()*grid.height(), nb_empty+nb_animals+nb_plants);
    assert!(animals.size() as u32 == nb_animals);
    assert!(plants.size() as u32 == nb_plants);
    assert!(predators.size() as u32 == nb_predators);
    assert!((grid.width()*grid.height()) == nb_empty + nb_animals + nb_plants + nb_predators);
}

// TODO: Bug to fix: The values provided are wrong if there's a min value different from 0
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
    header += ",predators";
    for r in 0..stats.stats[0].nb_predators_per_range.len() {
        header += &format!(",range={}", r);
    }
    for s in 0..stats.stats[0].nb_predators_per_speed.len() {
        header += &format!(",speed={}", s);
    }
    for s in 0..stats.stats[0].nb_predators_per_power.len() {
        header += &format!(",power={}", s);
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
        line += &format!(",{}", si.nb_predators);
        for r in si.nb_predators_per_range.iter() {
            line += &format!(",{}", r);
        }
        for s in si.nb_predators_per_speed.iter() {
            line += &format!(",{}", s);
        }
        for s in si.nb_predators_per_power.iter() {
            line += &format!(",{}", s);
        }
        line += "\n";
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn dump_graphs(dc: &mut DrawingContext, stats: &Stats, model: &Model, dir: String) {
    for graph_kind in GraphKind::all() {
        let mut graph = Graph::new(
                graph_title(&graph_kind),
                graph_legend(&model, &graph_kind),
                graph_data(&stats, &model, &graph_kind));
        graph.show(dc);
        dc.save_graph_png(Path::new(&format!("{}/{}.png", dir, graph_title(&graph_kind))));
        if let GraphKind::GlobalPopulations = graph_kind {
            graph.set_scale(vec![1.0, 5.0, 20.0]);
            graph.show(dc);
            dc.save_graph_png(Path::new(&format!("{}/{} scaled.png", dir, graph_title(&graph_kind))));
        }
    }
}

// TODO: AnimalsPower
#[derive(Copy, Clone, num_derive::FromPrimitive)]
enum GraphKind {
    GlobalPopulations,
    PlantsLayering,
    PlantsFertility,
    PlantsSpread,
    AnimalsSpeed,
    AnimalsRange,
    PredatorsSpeed,
    PredatorsRange,
    PredatorsPower,
}

impl GraphKind {
    fn next(&self) -> Self {
        match FromPrimitive::from_u8(*self as u8 + 1) {
            Some(k) => k,
            None => FromPrimitive::from_u8(0).unwrap(),
        }
    }

    fn all() -> Vec<GraphKind> {
        vec![
            GraphKind::GlobalPopulations,
            GraphKind::PlantsLayering,
            GraphKind::PlantsFertility,
            GraphKind::PlantsSpread,
            GraphKind::AnimalsSpeed,
            GraphKind::AnimalsRange,
            GraphKind::PredatorsSpeed,
            GraphKind::PredatorsRange,
            GraphKind::PredatorsPower,
        ]
    }
}

fn graph_title(kind: &GraphKind) -> String {
    String::from(match kind {
            GraphKind::GlobalPopulations => "Total populations",
            GraphKind::PlantsLayering => "Plants by layering",
            GraphKind::PlantsFertility => "Plants by fertility",
            GraphKind::PlantsSpread => "Plants by spread",
            GraphKind::AnimalsSpeed => "Animals by speed",
            GraphKind::AnimalsRange => "Animals by range",
            GraphKind::PredatorsSpeed => "Predators by speed",
            GraphKind::PredatorsRange => "Predators by range",
            GraphKind::PredatorsPower => "Predators by power",
            })
}

fn per_trait_graph_legend(min: u32, max: u32) -> Vec<String> {
    (min..=max).map(|i| i.to_string()).collect()
}

fn graph_legend(model: &Model, kind: &GraphKind) -> Vec<String> {
    match kind {
        GraphKind::GlobalPopulations => {
            vec!(String::from("Plants"), String::from("Animals"), String::from("Predators"))
        },
        GraphKind::PlantsLayering => {
            per_trait_graph_legend(model.plants_min_layering, model.plants_max_layering)
        },
        GraphKind::PlantsFertility => {
            per_trait_graph_legend(model.plants_min_fertility, model.plants_max_fertility)
        },
        GraphKind::PlantsSpread => {
            per_trait_graph_legend(model.plants_min_spread, model.plants_max_spread)
        },
        GraphKind::AnimalsSpeed => {
            per_trait_graph_legend(model.animals_min_speed, model.animals_max_speed)
        },
        GraphKind::AnimalsRange => {
            per_trait_graph_legend(model.animals_min_range, model.animals_max_range)
        },
        GraphKind::PredatorsSpeed => {
            per_trait_graph_legend(model.predators_min_range, model.predators_max_range)
        },
        GraphKind::PredatorsRange => {
            per_trait_graph_legend(model.predators_min_range, model.predators_max_range)
        },
        GraphKind::PredatorsPower => {
            per_trait_graph_legend(model.predators_min_power(), model.predators_max_power())
        },
    }
}

macro_rules! per_trait_graph_data {
    ($result: ident, $stats: ident, $min_val:expr, $max_val:expr, $item_list:ident) => {
            for _ in $min_val..=$max_val {
                $result.push(vec!());
            }
            for si in $stats.stats.iter() {
                for (i, l) in si.$item_list.iter().enumerate() {
                    $result[i].push(*l);
                }
            }
    };
}

fn graph_data(stats: &Stats, model: &Model, kind: &GraphKind) -> Vec<Vec<u32>> {
    let mut result = vec!();
    match kind {
        GraphKind::GlobalPopulations => {
            result.push(vec!());
            result.push(vec!());
            result.push(vec!());
            for si in stats.stats.iter() {
                result[0].push(si.nb_plants);
                result[1].push(si.nb_animals);
                result[2].push(si.nb_predators);
            }
        },
        GraphKind::PlantsLayering => {
            per_trait_graph_data!(result, stats, model.plants_min_layering, model.plants_max_layering, nb_plants_per_layering);
        },
        GraphKind::PlantsFertility => {
            per_trait_graph_data!(result, stats, model.plants_min_fertility, model.plants_max_fertility, nb_plants_per_fertility);
        },
        GraphKind::PlantsSpread => {
            per_trait_graph_data!(result, stats, model.plants_min_spread, model.plants_max_spread, nb_plants_per_spread);
        },
        GraphKind::AnimalsSpeed => {
            per_trait_graph_data!(result, stats, model.animals_min_speed, model.animals_max_speed, nb_animals_per_speed);
        },
        GraphKind::AnimalsRange => {
            per_trait_graph_data!(result, stats, model.animals_min_range, model.animals_max_range, nb_animals_per_range);
        },
        GraphKind::PredatorsSpeed => {
            per_trait_graph_data!(result, stats, model.predators_min_speed, model.predators_max_speed, nb_predators_per_speed);
        },
        GraphKind::PredatorsRange => {
            per_trait_graph_data!(result, stats, model.predators_min_range, model.predators_max_range, nb_predators_per_range);
        },
        GraphKind::PredatorsPower => {
            per_trait_graph_data!(result, stats, model.predators_min_power(), model.predators_max_power(), nb_predators_per_power);
        },
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

fn help_message() {
    println!("Commands available:");
    println!(" - G: Switch between graph and grid");
    println!(" - N: Switch to the next kind of graph");
    println!(" - S: On populations graph, apply a scale for better readability");
    println!(" - R: Reset the simulation");
    println!(" - P: Pause the simulation");
    println!(" - Space: When paused, advance one step");
    println!(" - Escape: Quit the simulation");
}

// TODO: Move the logic to move an animal/plant to the grid to have a single place that calls the
// removes
// TODO: Introduce mutations
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
        .arg(Arg::with_name("graph_every_n_round")
                .short("g")
                .long("graph_every_n_round")
                .value_name("NUMBER")
                .help("Dump graph every N generation (default: 100)")
                .takes_value(true)
                .validator(is_type::<u32>))
        .arg(Arg::with_name("screenshot_every_round")
                .short("s")
                .long("screenshot_every_round")
                .help("When provided, take a screenshot at every round."))
        .get_matches();
    let model = match matches.value_of("model") {
        None => Model::new(),
        Some(filename) => Model::load(Path::new(filename)),
    };
    let max_rounds = match matches.value_of("rounds") {
        None => std::u32::MAX,
        Some(num) => num.parse::<u32>().unwrap(),
    };
    let dump_graphs_every_n_round = match matches.value_of("graph_every_n_round") {
        None => 100,
        Some(num) => num.parse::<u32>().unwrap(),
    };
    let dump_screenshots = matches.is_present("screenshot_every_round");
    let mut pause = false;
    let mut scale = false;
    let mut show_graph = false;
    let mut dc = DrawingContext::new(model.screen_width, model.screen_height);
    // TODO: Move grid, plants, animals, predators, stats, step, run_name, result_dir in a "Run" object?
    let mut grid = Grid::new(model.grid_width(), model.grid_height(), model.cell_width);
    let mut plants = Plants::new(&mut grid, &model);
    let mut animals = Animals::new(&mut grid, &model);
    let mut predators = Predators::new(&mut grid, &model);
    let mut stats = Stats::new();
    let mut step = 0;
    let mut graph_kind = GraphKind::GlobalPopulations;

    let mut event_pump = dc.sdl_context.event_pump().unwrap();

    let run_name = Local::now().format("%Y-%m-%d_%H:%M:%S");
    let _ = fs::create_dir("results/");  // Can already exist
    let results_dir = format!("results/{}", run_name);
    fs::create_dir(&results_dir).unwrap();
    let result_path = |filename: &str| -> String {
        format!("{}/{}", results_dir, filename)
    };
    model.save(Path::new(&result_path("model.json")));
    fs::create_dir(Path::new(&result_path("graphs"))).unwrap();
    if dump_screenshots {
        fs::create_dir(Path::new(&result_path("screenshots"))).unwrap();
    }

    help_message();
    'game_loop: loop {
        grid.show(&mut dc);
        if show_graph {
            let mut graph = Graph::new(
                    graph_title(&graph_kind),
                    graph_legend(&model, &graph_kind),
                    graph_data(&stats, &model, &graph_kind));
            if scale {
                // TODO: Think of a better way of handling per-curve scaling
                if let GraphKind::GlobalPopulations = graph_kind {
                    graph.set_scale(vec![1.0, 5.0, 20.0]);
                }
            }
            graph.show(&mut dc);
            dc.blit_graph();
        } else {
            dc.blit_grid();
        }

        let mut do_one_step = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'game_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    do_one_step = true;
                },
                Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                    show_graph = !show_graph;
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    if show_graph {
                        graph_kind = graph_kind.next();
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    pause = !pause;
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new(model.grid_width(), model.grid_height(), model.cell_width);
                    plants = Plants::new(&mut grid, &model);
                    animals = Animals::new(&mut grid, &model);
                    predators = Predators::new(&mut grid, &model);
                    stats = Stats::new();
                    step = 0;
                    // TODO: Do a new result dir?
                },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    scale = !scale;
                },
                _ => {},
            }
        }

        if do_one_step || !pause {
            if step % model.steps_per_round == 0 {
                stats.update(&predators, &animals, &plants, &model);
            }
            step += 1;
            animals.update(&mut grid, &model);
            predators.update(&mut grid, &model);
            plants.cleanup();
            animals.cleanup();
            if step % model.steps_per_round == 0 {
                let round = step / model.steps_per_round;
                plants.reproduce(&mut grid, &model);
                animals.finish_round(&mut grid);
                predators.finish_round(&mut grid);
                if dump_screenshots {
                    // TODO: The following doesn't work. Try to reproduce in a minimal example and open an
                    // issue to sdl2 on github.
                    //dc.canvas.window().surface(&event_pump).unwrap().save_bmp(Path::new(&result_path(&format!("screenshots/{:06}.bmp", step)))).unwrap();
                    dc.save_grid_png(Path::new(&result_path(&format!("screenshots/{:06}.png", round))));
                }
                if round % dump_graphs_every_n_round == 0 {
                    let dirname = result_path(&format!("graphs/round{}", round));
                    fs::create_dir(Path::new(&dirname)).unwrap();
                    dump_graphs(&mut dc, &stats, &model, dirname);
                }
            }
            //consistency_checks(&predators, &animals, &plants, &grid);
        }

        dc.canvas.present();

        if step/model.steps_per_round >= max_rounds {
            break 'game_loop;
        }
    }

    dump_stats(&stats, Path::new(&result_path("stats.csv")));
    fs::create_dir(Path::new(&result_path("graphs/final"))).unwrap();
    dump_graphs(&mut dc, &stats, &model, result_path("graphs/final"));
}
