#[cfg(test)] #[macro_use] extern crate assert_matches;
#[cfg(test)] extern crate assert_approx_eq;
#[macro_use] extern crate itertools;
extern crate sdl2;

mod activation;
mod dc;
mod graph;
mod graph3D;
mod neuralnet;
mod neuron;
mod mnist;

use crate::activation::{RELU,SIGMOID,TANH};
use crate::dc::DrawingContext;
use crate::graph::Graph;
use crate::graph3D::Graph3D;
use crate::mnist::Img;
use crate::neuralnet::NeuralNet;

use chrono::Local;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::{LSHIFTMOD,NOMOD,RSHIFTMOD};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const TRAIN_LABELS : &str = "data/train-labels-idx1-ubyte";
const TRAIN_IMAGES : &str = "data/train-images-idx3-ubyte";
const TEST_LABELS : &str = "data/t10k-labels-idx1-ubyte";
const TEST_IMAGES : &str = "data/t10k-images-idx3-ubyte";

#[allow(dead_code)]
fn show_samples(labels: &str, images: &str, limit: u32) {
    let labels = mnist::read_labels(labels.to_string(), Some(limit)).unwrap();
    let images = mnist::read_images(images.to_string(), Some(limit)).unwrap();
    for (i, l) in labels.iter().enumerate() {
        println!("{}\nThis is a {}\n\n", images[i].to_string(), l);
    }
}

fn load_both(labels_filename: &str, images_filename: &str) -> (Vec<usize>, Vec<Vec<f64>>) {
    let labels = mnist::read_labels(labels_filename.to_string(), None).unwrap();
    let labels = labels.into_iter().map(|l| l as usize).collect();
    let examples = mnist::read_images(images_filename.to_string(), None).unwrap();
    let examples = examples.into_iter().map(|i| i.pixels).collect();
    (labels, examples)
}

fn train_mnist() {
    //show_samples(TRAIN_LABELS, TRAIN_IMAGES, 10);
    //show_samples(TEST_LABELS, TEST_IMAGES, 10);
    let (train_labels, train_examples) = load_both(TRAIN_LABELS, TRAIN_IMAGES);
    let mut nn = NeuralNet::new(784, vec![100, 10], Box::new(RELU), true);
    //let mut nn = NeuralNet::load("model.json");
    nn.train_class(30, 20, 0.1, train_examples, train_labels);

    let (test_labels, test_examples) = load_both(TEST_LABELS, TEST_IMAGES);
    let (mut correct, mut total) = (0, 0);
    for (l, e) in test_labels.iter().zip(test_examples.iter()) {
        let p = nn.predict(e.clone());
        if p != *l {
            let i = Img::new(28, 28, e.clone());
            println!("Mispredicted {} instead of {} for {}", p, l, i.to_string());
        } else {
            correct += 1;
        }
        total += 1;
    }

    println!("Score: {} out of {} ({}%)", correct, total, 100.0*correct as f64/total as f64);
    nn.save("model.json");
}

fn target_function_1d(i: f64) -> f64 {
    //(i + i.sin())/10.0
    //(i/3.0).sin()
    i.sin()/(i.abs() + 1.0)
}

fn target_function_2d(i: f64, j: f64) -> f64 {
    //i.sin()*j.sin()
    //i+j
    i*j
    //i
}

fn dump_errors(errors: &Vec<f64>, path: &str) {
    let mut file = File::create(Path::new(path)).unwrap();
    file.write_all(b"epoch,error\n").unwrap();
    for (i, e) in errors.iter().enumerate() {
        let line = format!("{},{}\n", i, e);
        file.write_all(line.as_bytes()).unwrap();
    }
}

fn train_1d_function() {
    //let max_inp = 1000;
    let max_inp = 50;
    let min_inp = -max_inp;
    let normalize_inp = (max_inp - min_inp) as f64;
    let min_inp_tst = min_inp;
    let max_inp_tst = max_inp;
    //let tst_div = 100.0;
    let tst_div = 5.0;

    let mut nn = NeuralNet::new(1, vec![20, 20, 20, 20, 1], Box::new(TANH), true);

    let nb_epochs = 10000;
    let mut learning_rate = 0.01;
    let mut samples_per_round = 1;
    let mut rounds_per_epoch = 1000;

    let mut errors = vec!();
    let run_name = Local::now().format("1d_%Y-%m-%d_%H:%M:%S");
    let _ = fs::create_dir(Path::new("results")); // Can already exist
    let results_dir = format!("results/{}", run_name);
    fs::create_dir(&results_dir).unwrap();
    let graphs_dir = &format!("{}/graphs", results_dir);
    fs::create_dir(&graphs_dir).unwrap();
    let mut dc = DrawingContext::new(2000, 1200);
    'main_loop: for epoch in 0..nb_epochs {
        let mut event_pump = dc.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::L), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    learning_rate *= 1.1;
                    println!("learning_rate={}", learning_rate);
                },
                Event::KeyDown { keycode: Some(Keycode::L), keymod: NOMOD, .. } => {
                    learning_rate /= 1.1;
                    println!("learning_rate={}", learning_rate);
                },
                Event::KeyDown { keycode: Some(Keycode::S), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    samples_per_round *= 2;
                    println!("samples_per_round={}", samples_per_round);
                },
                Event::KeyDown { keycode: Some(Keycode::S), keymod: NOMOD, .. } => {
                    samples_per_round /= 2;
                    println!("samples_per_round={}", samples_per_round);
                    if samples_per_round == 0 {
                        samples_per_round = 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    rounds_per_epoch *= 2;
                    println!("rounds_per_epoch={}", rounds_per_epoch);
                },
                Event::KeyDown { keycode: Some(Keycode::R), keymod: NOMOD, .. } => {
                    rounds_per_epoch /= 2;
                    println!("rounds_per_epoch={}", rounds_per_epoch);
                    if rounds_per_epoch == 0 {
                        rounds_per_epoch = 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    nn.save("target_function_model.json");
                }
                _ => {},
            }
        }

        if epoch > 0 {
            // Train on an input shorter than the whole testing range first, then extend it
            /*
            let limit = i32::max(10*epoch, max_inp);
            let (min_inp_trn, max_inp_trn) = (-limit, limit);
            */
            /**/
            // Works pretty well, on sin(x)/(x+1). Even a more agressive with just 1st step and then
            // last works well too.
            let (min_inp_trn, max_inp_trn) = if epoch < nb_epochs / 20 {
                (min_inp/3, max_inp/3)
            } else if epoch < nb_epochs / 10 {
                (2*min_inp/3, 2*max_inp/3)
            } else {
                (min_inp, max_inp)
            };
            /**/
            /*
            let (min_inp_trn, max_inp_trn) = if epoch < nb_epochs / 20 {
                (0, max_inp/2)
            } else if epoch < nb_epochs / 10 {
                (0, max_inp)
            } else {
                (min_inp, max_inp+1)
            };
            */
            /*
            // Doesn't work well on sin(x)/(x+1). The end of the curve has variations that are too
            // small compare to the middle.
            let (min_inp_trn, max_inp_trn) = if epoch < nb_epochs / 20 {
                (max_inp/3, max_inp+1)
            } else if epoch < nb_epochs / 10 {
                (0, max_inp+1)
            } else {
                (min_inp, max_inp+1)
            };
            */
            /*
            let (min_inp_trn, max_inp_trn) = (min_inp, max_inp);
            */
            let trn_div = tst_div;
            let train_inputs : Vec<_> = (min_inp_trn..max_inp_trn).map(|x| vec![x as f64/normalize_inp]).collect();
            let train_outputs : Vec<_> = train_inputs.iter().map(|x| vec![target_function_1d(x[0]*normalize_inp/trn_div)]).collect();

            nn.train(rounds_per_epoch, samples_per_round, learning_rate, train_inputs.clone(), train_outputs.clone());
        }

        // Test domain is larger and more precise than train domain
        let test_inputs : Vec<_> = (min_inp_tst..max_inp_tst).map(|x| vec![x as f64/normalize_inp]).collect();
        let mut test_graph = vec!();
        let mut target_graph = vec!();
        //println!("test_inputs = {:?}", test_inputs);
        let mut nb = 0;
        let mut error = 0.0;
        for l in test_inputs.into_iter() {
            let p = nn.evaluate(l.clone(), false);
            let t = target_function_1d(l[0]*normalize_inp/tst_div);
            let e = p[0] - t;
            test_graph.push(p[0]);
            target_graph.push(t);
            error += e*e;
            nb += 1;
        }
        let rmse = (error / nb as f64).sqrt();
        errors.push(rmse);
        // Build and show graph
        {
            let title = String::from("Target function approximation");
            let legend = vec![
                String::from("Train"),
                String::from("Test")
            ];
            let data = vec![target_graph, test_graph];
            let additional_infos = vec![
                format!("epoch={}/{}", epoch, nb_epochs),
                format!("learning_rate={}", learning_rate),
                format!("samples_per_round={}", samples_per_round),
                format!("rounds_per_epoch={}", rounds_per_epoch),
                format!("error (RMS)={}", rmse),
            ];
            let graph = Graph::new(title, legend, data, additional_infos);
            graph.show(&mut dc);
            dc.save_graph_png(Path::new(&format!("{}/{:09}.png", graphs_dir, epoch)));
        }
        dc.blit_graph();
        dc.canvas.present();
        //println!("Epoch: {} - Error: {}", epoch, rmse);
    }
    dump_errors(&errors, &format!("{}/errors.csv", results_dir));
    nn.save(&format!("{}/model.json", results_dir));
}

fn train_2d_function() {
    //let max_inp = 1000;
    let max_inp = 5;
    let min_inp = -max_inp;
    let normalize_inp = (max_inp - min_inp) as f64;
    let min_inp_tst = min_inp;
    let max_inp_tst = max_inp;
    //let tst_div = 100.0;
    let tst_div = 5.0;

    let mut nn = NeuralNet::new(2, vec![40, 40, 40, 1], Box::new(TANH), true);

    let nb_epochs = 10000;
    let mut learning_rate = 0.01;
    let mut samples_per_round = 1;
    let mut rounds_per_epoch = 1000;

    let mut errors = vec!();
    let run_name = Local::now().format("2d_%Y-%m-%d_%H:%M:%S");
    let _ = fs::create_dir(Path::new("results")); // Can already exist
    let results_dir = format!("results/{}", run_name);
    fs::create_dir(&results_dir).unwrap();
    let graphs_dir = &format!("{}/graphs", results_dir);
    fs::create_dir(&graphs_dir).unwrap();
    let mut dc = DrawingContext::new(2000, 1200);
    'main_loop: for epoch in 0..nb_epochs {
        let mut event_pump = dc.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::L), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    learning_rate *= 1.1;
                    println!("learning_rate={}", learning_rate);
                },
                Event::KeyDown { keycode: Some(Keycode::L), keymod: NOMOD, .. } => {
                    learning_rate /= 1.1;
                    println!("learning_rate={}", learning_rate);
                },
                Event::KeyDown { keycode: Some(Keycode::S), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    samples_per_round *= 2;
                    println!("samples_per_round={}", samples_per_round);
                },
                Event::KeyDown { keycode: Some(Keycode::S), keymod: NOMOD, .. } => {
                    samples_per_round /= 2;
                    println!("samples_per_round={}", samples_per_round);
                    if samples_per_round == 0 {
                        samples_per_round = 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::R), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    rounds_per_epoch *= 2;
                    println!("rounds_per_epoch={}", rounds_per_epoch);
                },
                Event::KeyDown { keycode: Some(Keycode::R), keymod: NOMOD, .. } => {
                    rounds_per_epoch /= 2;
                    println!("rounds_per_epoch={}", rounds_per_epoch);
                    if rounds_per_epoch == 0 {
                        rounds_per_epoch = 1;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    nn.save("target_function_model.json");
                }
                _ => {},
            }
        }

        if epoch > 0 {
            // Train on an input shorter than the whole testing range first, then extend it
            /*
            let limit = i32::max(10*epoch, max_inp);
            let (min_inp_trn, max_inp_trn) = (-limit, limit);
            */
            /*
            let (min_inp_trn, max_inp_trn) = if epoch < nb_epochs / 20 {
                (min_inp/4, max_inp/4)
            } else if epoch < nb_epochs / 10 {
                (min_inp/3, max_inp/3)
            } else if epoch < nb_epochs / 5 {
                (2*min_inp/3, 2*max_inp/3)
            } else {
                (min_inp, max_inp)
            };
            */
            let (min_inp_trn, max_inp_trn) = (min_inp, max_inp);
            let trn_div = tst_div;
            let train_inputs : Vec<_> = iproduct!((min_inp_trn..max_inp_trn), (min_inp_trn..max_inp_trn)).map(|(x, y)| vec![x as f64/normalize_inp, y as f64/normalize_inp]).collect();
            let train_outputs : Vec<_> = train_inputs.iter().map(|x| vec![target_function_2d(x[0]*normalize_inp/trn_div, x[1]*normalize_inp/trn_div)]).collect();

            nn.train(rounds_per_epoch, samples_per_round, learning_rate, train_inputs.clone(), train_outputs.clone());
        }

        // Test domain is larger and more precise than train domain
        let test_inputs : Vec<_> = iproduct!((min_inp_tst..max_inp_tst), (min_inp_tst..max_inp_tst)).map(|(x, y)| vec![x as f64/normalize_inp, y as f64/normalize_inp]).collect();
        let mut test_graph = vec!();
        let mut target_graph = vec!();
        //println!("test_inputs = {:?}", test_inputs);
        let mut nb = 0;
        let mut error = 0.0;
        let mut prev_x = 0.42;
        let mut prev_y = 0.42;
        let mut i = 0;
        for l in test_inputs.into_iter() {
            let x = l[0]*normalize_inp/tst_div;
            let y = l[1]*normalize_inp/tst_div;
            let p = nn.evaluate(l.clone(), false);
            let t = target_function_2d(x, y);
            let e = p[0] - t;
            if x != prev_x {
                if test_graph.len() != 0 {
                    i += 1;
                }
                test_graph.push(vec!());
                target_graph.push(vec!());
            }
            test_graph[i].push((x, y, p[0]));
            target_graph[i].push((x, y, t));
            error += e*e;
            nb += 1;
            prev_x = x;
            prev_y = y;
        }
        let rmse = (error / nb as f64).sqrt();
        errors.push(rmse);
        // Build and show graph
        {
            let title = String::from("Target function approximation");
            let legend = vec![
                String::from("Train"),
                String::from("Test")
            ];
            let data = vec![target_graph, test_graph];
            let additional_infos = vec![
                format!("epoch={}/{}", epoch, nb_epochs),
                format!("learning_rate={}", learning_rate),
                format!("samples_per_round={}", samples_per_round),
                format!("rounds_per_epoch={}", rounds_per_epoch),
                format!("error (RMS)={}", rmse),
            ];
            let mut graph = Graph3D::new(title, legend, data, additional_infos);
            graph.show(&mut dc);
            dc.save_graph_png(Path::new(&format!("{}/{:09}.png", graphs_dir, epoch)));
        }
        dc.blit_graph();
        dc.canvas.present();
        //println!("Epoch: {} - Error: {}", epoch, rmse);
    }
    dump_errors(&errors, &format!("{}/errors.csv", results_dir));
    nn.save(&format!("{}/model.json", results_dir));
}

fn test_3d_graph() {
    let mut dc = DrawingContext::new(2000, 1200);
    let mut graph3D_res = 20;
    'main_loop: loop {
        let mut event_pump = dc.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'main_loop;
                },
                Event::KeyDown { keycode: Some(Keycode::P), keymod: NOMOD, .. } => {
                    graph3D_res /= 2;
                }
                Event::KeyDown { keycode: Some(Keycode::P), keymod: m, .. } if m == LSHIFTMOD || m == RSHIFTMOD => {
                    graph3D_res *= 2;
                }
                _ => {},
            }
        }

        let title = String::from("Graph of x*y");
        let legend = vec![
            String::from("x*y"),
            String::from("sin(x)*sin(y)"),
        ];
        let mut data = vec![vec![], vec![]];
        // Draw a graph for x in [min_x, max_x] with res_x point, so a point every dx (and same for
        // y).
        let res_x = graph3D_res;
        let res_y = graph3D_res;
        let min_x = -10.0;
        let max_x = 10.0;
        let min_y = 0.0;
        let max_y = 10.0;
        let dx = (max_x-min_x)/res_x as f64;
        let dy = (max_y-min_y)/res_y as f64;
        for i in 0..res_y {
            data[0].push(vec![]);
            data[1].push(vec![]);
            for j in 0..res_x {
                let x = min_x + j as f64*dx;
                let y = min_y + i as f64*dy;
                data[0][i].push((x, y, 0.0*x*y - 50.0));
                data[1][i].push((x, y, 20.0*x.sin()*y.sin() + 100.0));
            }
        }
        let x_scale = 100.0*dx;
        let y_scale = 10.0*dy;
        let z_scale = 10.0;
        let additional_infos = vec![];
        let mut graph = Graph3D::new(title, legend, data, additional_infos);
        graph.set_scale(x_scale, y_scale, z_scale);
        graph.show(&mut dc);
        dc.blit_graph();
        dc.canvas.present();
    }
}

fn main() {
    //train_mnist();
    //train_1d_function();
    //test_3d_graph();
    train_2d_function();
}
