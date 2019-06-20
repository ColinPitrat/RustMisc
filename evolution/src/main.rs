extern crate chrono;
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
use dc::DrawingContext;
use graph::Graph;
use grid::{CellContent,Grid};
use model::Model;
use plant::Plants;
use stats::Stats;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::fs;
use std::path::Path;

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
    //plants.consistency_checks();
    //animals.consistency_checks();
    println!("{} plants in the grid, {} plants in the list", nb_plants, plants.size() as u32);
    println!("{} animals in the grid, {} animals in the list", nb_animals, animals.size() as u32);
    println!("{} cells in the grid, {} cells checked", grid.width()*grid.height(), nb_empty+nb_animals+nb_plants);
    assert!(animals.size() as u32 == nb_animals);
    assert!(plants.size() as u32 == nb_plants);
    assert!((grid.width()*grid.height()) == nb_empty + nb_animals + nb_plants);
}

/*fn summary(animals: &Animals, plants: &Plants) {
    println!("{} plants - {} animals", plants.size(), animals.size());
}*/

fn dump_stats(stats: &Stats) {
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
    println!("{}", header);
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
        println!("{}", &line);
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

// TODO: Load model from file (passed as argument)
// TODO: Support stopping after N generations
// TODO: Dump curves and stats in a subdirectory for each run. Screenshot at each generation too
// generate animated GIF?
// TODO: Loop on various models and compare results
// TODO: Add predators
fn main() {
    let model = Model::new();
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
    if dump_screenshots {
        let _ = fs::create_dir("results/");  // Can already exist
        fs::create_dir(format!("results/{}", run_name)).unwrap();
        fs::create_dir(format!("results/{}/screenshots", run_name)).unwrap();
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
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        dump_stats(&stats);
                        break 'game_loop
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        step += 1;
                        animals.update(&mut grid, &model);
                        plants.cleanup();
                        if step % model.steps_per_round == 0 {
                            plants.reproduce(&mut grid, &model);
                            animals.finish_round(&mut grid);
                        }
                        //summary(&animals, &plants);
                    },
                    Event::KeyDown { keycode: Some(Keycode::G), .. } => {
                        show_graph = !show_graph;
                    }
                Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                    grid = Grid::new(model.grid_width(), model.grid_height(), model.cell_width);
                    plants = Plants::new(&mut grid, &model);
                    animals = Animals::new(&mut grid, &model);
                    consistency_checks(&animals, &plants, &grid);
                },
                    _ => {}
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
                    //dc.canvas.window().surface(&event_pump).unwrap().save_bmp(Path::new(&format!("results/{}/screenshots/{:06}.bmp", run_name, step))).unwrap();
                    dc.save_grid_png(Path::new(&format!("results/{}/screenshots/{:06}.png", run_name, step/model.steps_per_round)));
                }
            }
            //summary(&animals, &plants);
        }

        dc.canvas.present();
    }
}
