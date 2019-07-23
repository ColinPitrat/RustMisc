#[cfg(test)] #[macro_use] extern crate assert_matches;

mod activation;
mod neuron;
mod mnist;

const TRAIN_LABELS : &str = "data/train-labels-idx1-ubyte";
const TRAIN_IMAGES : &str = "data/train-images-idx3-ubyte";
const TEST_LABELS : &str = "data/t10k-labels-idx1-ubyte";
const TEST_IMAGES : &str = "data/t10k-images-idx3-ubyte";

fn show_samples(labels: &str, images: &str, limit: u32) {
    let labels = mnist::read_labels(labels.to_string(), Some(limit)).unwrap();
    let images = mnist::read_images(images.to_string(), Some(limit)).unwrap();
    for (i, l) in labels.iter().enumerate() {
        println!("{}\nThis is a {}\n\n", images[i].to_string(), l);
    }
}

fn main() {
    show_samples(TRAIN_LABELS, TRAIN_IMAGES, 10);
    show_samples(TEST_LABELS, TEST_IMAGES, 10);
}
