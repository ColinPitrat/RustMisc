#[cfg(test)] #[macro_use] extern crate assert_matches;
#[cfg(test)] extern crate assert_approx_eq;

mod activation;
mod neuralnet;
mod neuron;
mod mnist;

use crate::activation::SIGMOID;
use crate::mnist::Img;
use crate::neuralnet::NeuralNet;

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

fn main() {
    //show_samples(TRAIN_LABELS, TRAIN_IMAGES, 10);
    //show_samples(TEST_LABELS, TEST_IMAGES, 10);
    let (train_labels, train_examples) = load_both(TRAIN_LABELS, TRAIN_IMAGES);
    //let mut nn = NeuralNet::new(784, vec![20, 10], Box::new(SIGMOID), true);
    let mut nn = NeuralNet::load("model.json");
    nn.train(100000, 10, 0.2, train_examples, train_labels);

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
