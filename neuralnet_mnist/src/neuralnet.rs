extern crate rand;

use crate::neuron::Neuron;
use crate::activation::ActivationFunction;
use rand::prelude::*;
use serde::{Serialize,Deserialize};
use std::fs;
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct NeuralNet {
    layers: Vec<Vec<Neuron>>,
}

impl NeuralNet {
    pub fn new(inputs_size: usize, layers_sizes: Vec<usize>, activation: Box<dyn ActivationFunction>, average_gradient: bool) -> NeuralNet {
        let mut layers = vec!();
        let mut previous_size = inputs_size;
        let activation = Rc::new(activation);
        for size in layers_sizes {
            layers.push((0..size).map(|_| Neuron::new(previous_size, Rc::clone(&activation), average_gradient)).collect());
            previous_size = size;
        }
        NeuralNet{layers}
    }

    pub fn load(filename: &str) -> NeuralNet {
	serde_json::from_str(&fs::read_to_string(filename).expect(&format!("Unable to read file {:?}.", filename))).unwrap()
    }

    pub fn save(&self, filename: &str) {
	fs::write(&filename, serde_json::to_string_pretty(&self).unwrap()).expect(&format!("Unable to write file {:?}.", filename));
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("Network ({} layers):\n", self.layers.len());
        for (i, l) in self.layers.iter().enumerate() {
            result += &format!(" - layer {}: {} neurons\n", i, l.len());
            for (j, n) in l.iter().enumerate() {
                result += &format!("    - neuron {}: {}\n", j, n.to_string());
            }
        }
        result
    }

    pub fn evaluate(&mut self, input: Vec<f64>, for_training: bool) -> Vec<f64> {
        let mut result = input;
        for layer in self.layers.iter_mut() {
            result = layer.iter_mut().map(|neuron| neuron.output(result.clone(), for_training)).collect();
        }
        result
    }

    fn per_eval_backprop(&mut self, errors: Vec<f64>, learning_rate: f64) {
        let mut result = errors;
        for layer in self.layers.iter_mut().rev() {
            let mut new_result = vec!();
            for (r, neuron) in result.iter().zip(layer.iter_mut()) {
                let contrib = neuron.per_eval_backprop(*r, learning_rate);
                if new_result.is_empty() {
                    new_result = contrib;
                } else {
                    new_result = new_result.into_iter().zip(&contrib).map(|(nr, c)| nr+c).collect();
                }
            }
            let l = layer.len() as f64;
            result = new_result.into_iter().map(|nr| nr/l).collect();
        }
    }

    fn per_round_backprop(&mut self) {
        for layer in self.layers.iter_mut() {
            for neuron in layer.iter_mut() {
                neuron.per_round_backprop();
            }
        }
    }

    pub fn train(&mut self, training_rounds: u32, samples_per_round: u32, learning_rate: f64, examples: Vec<Vec<f64>>, output: Vec<Vec<f64>>) {
        let nb_examples = examples.len();
        let mut rng = rand::thread_rng();
        for i in 0..training_rounds {
            //println!("{}", self.to_string());
            let mut _error = 0.0;
            for _i in 0..samples_per_round {
                let k = rng.gen_range(0, nb_examples);
                //println!("  k = {} (0, {})", k, nb_examples);
                let result = self.evaluate(examples[k].clone(), true);
                //println!("  sin({}) = {} - estimated at {}", examples[k][0], output[k][0], result[0]);
                let errors : Vec<_> = result.into_iter().zip(output[k].iter()).map(|(r, e)| e - r).collect();
                self.per_eval_backprop(errors.clone(), learning_rate);
                _error += errors.into_iter().map(|x| x*x).sum::<f64>().sqrt();
            }
            /*if i%100 == 0 {
                println!("Round {}: error={}, learning_rate={}", i, _error, learning_rate);
            }*/
            self.per_round_backprop();
        }
    }

    // TODO: Deduplicate with train (build output from labels)
    pub fn train_class(&mut self, training_rounds: u32, samples_per_round: u32, learning_rate: f64, examples: Vec<Vec<f64>>, labels: Vec<usize>) {
        let nb_examples = examples.len();
        let nb_classes = self.layers.last().unwrap().len();
        let mut rng = rand::thread_rng();
        for i in 0..training_rounds {
            //println!("{}", self.to_string());
            let mut _error = 0.0;
            for _i in 0..samples_per_round {
                let k = rng.gen_range(0, nb_examples);
                let result = self.evaluate(examples[k].clone(), true);
                let mut expected = vec![0.0;nb_classes];
                expected[labels[k]] = 1.0;
                //println!("Result: {:?}", result);
                //println!("Expected: {:?}", expected);
                let errors : Vec<_> = result.into_iter().zip(expected.into_iter()).map(|(r, e)| e - r).collect();
                //println!("Errors: {:?}", errors);
                self.per_eval_backprop(errors.clone(), learning_rate);
                _error += errors.into_iter().map(|x| x*x).sum::<f64>().sqrt();
                //println!("Error: {}", _error);
            }
            println!("Round {}: error={}, learning_rate={}", i, _error, learning_rate);
            self.per_round_backprop();
        }
        //println!("{}", self.to_string());
    }

    pub fn predict(&mut self, example: Vec<f64>) -> usize {
        let scores = self.evaluate(example, false);
	scores.iter()
	    .enumerate()
	    .max_by(|(_, a), (_, b)| a.partial_cmp(b).expect("Predict got a NaN !"))
	    .expect("Predict got empty result from evaluate").0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use crate::activation::{ReLu,RELU,SIGMOID};

    #[test]
    fn multiple_layers_network_activation() {
        let mut nn = NeuralNet::new(2, vec![2, 2], Box::new(RELU), false);
        // 1 -- 0.5 --> [-0.1](A) -- 0.7 --> [-0.2](C)
        //     ^  |              ^  |
        //     |  0.7            |  0.8
        //   0.2    |          0.5    |
        //   |      v          |      v
        // 0 -- 0.6 --> [-0.1](B) -- 0.9 --> [-0.3](D)
        // (A) = 0.5 - 0.1 = 0.4
        // (B) = 0.7 - 0.1 = 0.6
        // (C) = 0.4x0.7 + 0.6x0.5 - 0.2 = 0.38
        // (D) = 0.9x0.6 + 0.8x0.4 - 0.3 = 0.56
        nn.layers[0][0].weights = vec![0.5, 0.2];
        nn.layers[0][0].bias = -0.1;
        nn.layers[0][1].weights = vec![0.7, 0.6];
        nn.layers[0][1].bias = -0.1;
        nn.layers[1][0].weights = vec![0.7, 0.5];
        nn.layers[1][0].bias = -0.2;
        nn.layers[1][1].weights = vec![0.8, 0.9];
        nn.layers[1][1].bias = -0.3;

        let neuron_a_output = nn.layers[0][0].output(vec![1.0, 0.0], false);
        assert_approx_eq!(0.4, neuron_a_output);
        let neuron_b_output = nn.layers[0][1].output(vec![1.0, 0.0], false);
        assert_approx_eq!(0.6, neuron_b_output);

        let output = nn.evaluate(vec![1.0, 0.0], false);

        assert_approx_eq!(0.38, output[0]);
        assert_approx_eq!(0.56, output[1]);
    }

    #[test]
    fn multiple_layers_network_backpropagation() {
        let mut nn = NeuralNet::new(2, vec![2, 2], Box::new(RELU), false);
        // 1 -- 0.5 --> [-0.1](A) -- 0.7 --> [-0.2](C)
        //     ^  |              ^  |
        //     |  0.7            |  0.8
        //   0.2    |          0.5    |
        //   |      v          |      v
        // 0 -- 0.6 --> [-0.1](B) -- 0.9 --> [-0.3](D)
        // (A) = 0.5 - 0.1 = 0.4
        // (B) = 0.7 - 0.1 = 0.6
        // (C) = 0.4x0.7 + 0.6x0.5 - 0.2 = 0.38
        // (D) = 0.9x0.6 + 0.8x0.4 - 0.3 = 0.56
        nn.layers[0][0].weights = vec![0.5, 0.2];
        nn.layers[0][0].bias = -0.1;
        nn.layers[0][1].weights = vec![0.7, 0.6];
        nn.layers[0][1].bias = -0.1;
        nn.layers[1][0].weights = vec![0.7, 0.5];
        nn.layers[1][0].bias = -0.2;
        nn.layers[1][1].weights = vec![0.8, 0.9];
        nn.layers[1][1].bias = -0.3;

        // Start with same evaluation as multiple_layers_network_activation
        let output = nn.evaluate(vec![1.0, 0.0], true);
        assert_approx_eq!(0.38, output[0]);
        assert_approx_eq!(0.56, output[1]);
        let expected = vec![0.0, 1.0];
        let error : Vec<_> = expected.into_iter().zip(output.into_iter()).map(|(e, o)| e-o).collect();
        // Verify errors values are as expected
        assert_approx_eq!(-0.38, error[0]);
        assert_approx_eq!(0.44, error[1]);

        nn.per_eval_backprop(error, 1.0);

        // Verify backpropagation of layer 1:
        // a(n-1)    w   expected   result   error     dE     db      dw   da(n-1)
        //   0.4   0.7          0     0.38   -0.38   0.76  -0.76  -0.304    -0.532
        //   0.6   0.5          -        -       -      -      -  -0.456     -0.38
        //   0.4   0.8          -        -       -      -      -   0.352     0.704
        //   0.6   0.9          1     0.56    0.44  -0.88   0.88   0.528     0.792
        assert_approx_eq!(-0.76, nn.layers[1][0].db);
        assert_approx_eq!(0.88, nn.layers[1][1].db);
        assert_approx_eq!(-0.304, nn.layers[1][0].dw[0]);
        assert_approx_eq!(-0.456, nn.layers[1][0].dw[1]);
        assert_approx_eq!(0.352, nn.layers[1][1].dw[0]);
        assert_approx_eq!(0.528, nn.layers[1][1].dw[1]);
        assert_approx_eq!(-0.532, nn.layers[1][0].da[0]);
        assert_approx_eq!(-0.38, nn.layers[1][0].da[1]);
        assert_approx_eq!(0.704, nn.layers[1][1].da[0]);
        assert_approx_eq!(0.792, nn.layers[1][1].da[1]);
        // Verify backpropagation of layer 2:
        // a(n-1)    w   expected   result   error      dE       db       dw   da(n-1)
        //     1   0.0          -      0.4   0.086  -0.172    0.172    0.172     0.086
        //     0   0.2          -        -       -       -        -        0    0.0344
        //     1   0.7          -        -       -       -        -    0.412    0.2884
        //     0   0.6          -      0.6   0.206  -0.412    0.412        0    0.2472
        assert_approx_eq!(0.172, nn.layers[0][0].db);
        assert_approx_eq!(0.412, nn.layers[0][1].db);
        assert_approx_eq!(0.172, nn.layers[0][0].dw[0]);
        assert_approx_eq!(0.0, nn.layers[0][0].dw[1]);
        assert_approx_eq!(0.412, nn.layers[0][1].dw[0]);
        assert_approx_eq!(0.0, nn.layers[0][1].dw[1]);
        assert_approx_eq!(0.086, nn.layers[0][0].da[0]);
        assert_approx_eq!(0.0344, nn.layers[0][0].da[1]);
        assert_approx_eq!(0.2884, nn.layers[0][1].da[0]);
        assert_approx_eq!(0.2472, nn.layers[0][1].da[1]);
    }

    #[test]
    fn backpropagate_error_on_network() {
        let mut nn = NeuralNet::new(2, vec![2, 2], Box::new(RELU), false);
        // 1 -- 0.5 --> [-0.1](A) -- 0.7 --> [-0.2](C)
        //     ^  |              ^  |
        //     |  0.7            |  0.8
        //   0.2    |          0.5    |
        //   |      v          |      v
        // 0 -- 0.6 --> [-0.1](B) -- 0.9 --> [-0.3](D)
        // (A) = 0.5 - 0.1 = 0.4
        // (B) = 0.7 - 0.1 = 0.6
        // (C) = 0.4x0.7 + 0.6x0.5 - 0.2 = 0.38
        // (D) = 0.9x0.6 + 0.8x0.4 - 0.3 = 0.56
        nn.layers[0][0].weights=vec![0.5, 0.2];
        nn.layers[0][0].bias=-0.1;
        nn.layers[0][1].weights=vec![0.7, 0.6];
        nn.layers[0][1].bias=-0.1;
        nn.layers[1][0].weights=vec![0.7, 0.5];
        nn.layers[1][0].bias=-0.2;
        nn.layers[1][1].weights=vec![0.8, 0.9];
        nn.layers[1][1].bias=-0.3;

        // Start with same evaluation as multiple_layers_network_activation
        let output = nn.evaluate(vec![1.0, 0.0], true);
        assert_approx_eq!(0.38, output[0]);
        assert_approx_eq!(0.56, output[1]);
        let expected = vec![0.0, 1.0];
        let error : Vec<_> = expected.into_iter().zip(output).map(|(e, r)| e-r).collect();
        // Verify errors values are as expected
        assert_approx_eq!(-0.38, error[0]);
        assert_approx_eq!(0.44, error[1]);

        nn.per_eval_backprop(error, 1.0);
        // Verify backpropagation of layer 1:
        // a(n-1)    w   expected   result   error     dE     db      dw   da(n-1)
        //   0.4   0.7          0     0.38   -0.38   0.76  -0.76  -0.304    -0.532
        //   0.6   0.5          -        -       -      -      -  -0.456     -0.38
        //   0.4   0.8          -        -       -      -      -   0.352     0.704
        //   0.6   0.9          1     0.56    0.44  -0.88   0.88   0.528     0.792
        assert_approx_eq!(-0.76, nn.layers[1][0].db);
        assert_approx_eq!(0.88, nn.layers[1][1].db);
        assert_approx_eq!(-0.304, nn.layers[1][0].dw[0]);
        assert_approx_eq!(-0.456, nn.layers[1][0].dw[1]);
        assert_approx_eq!(0.352, nn.layers[1][1].dw[0]);
        assert_approx_eq!(0.528, nn.layers[1][1].dw[1]);
        assert_approx_eq!(-0.532, nn.layers[1][0].da[0]);
        assert_approx_eq!(-0.38, nn.layers[1][0].da[1]);
        assert_approx_eq!(0.704, nn.layers[1][1].da[0]);
        assert_approx_eq!(0.792, nn.layers[1][1].da[1]);
        // Verify backpropagation of layer 2:
        // a(n-1)    w   expected   result   error      dE       db       dw   da(n-1)
        //     1   0.0          -      0.4   0.086  -0.172    0.172    0.172     0.086
        //     0   0.2          -        -       -       -        -        0    0.0344
        //     1   0.7          -        -       -       -        -    0.412    0.2884
        //     0   0.6          -      0.6   0.206  -0.412    0.412        0    0.2472
        assert_approx_eq!(0.172, nn.layers[0][0].db);
        assert_approx_eq!(0.412, nn.layers[0][1].db);
        assert_approx_eq!(0.172, nn.layers[0][0].dw[0]);
        assert_approx_eq!(0.0, nn.layers[0][0].dw[1]);
        assert_approx_eq!(0.412, nn.layers[0][1].dw[0]);
        assert_approx_eq!(0.0, nn.layers[0][1].dw[1]);
        assert_approx_eq!(0.086, nn.layers[0][0].da[0]);
        assert_approx_eq!(0.0344, nn.layers[0][0].da[1]);
        assert_approx_eq!(0.2884, nn.layers[0][1].da[0]);
        assert_approx_eq!(0.2472, nn.layers[0][1].da[1]);
    }

    #[test]
    fn train_on_simple_example() {
        // Simple example that only depends on the first parameter.
        // A linear separation would be enough but it has the benefit of training fast :-)
        // Created with Python script:
        //   import random
        //   print(random.random()*4, random.random()*5, 0) for i in range(0, 5)
        //   print(5+random.random()*4, random.random()*5, 1) for i in range(0, 5)
        let dataset = vec![
            (vec![2.7810836, 2.550537003],   0),
            (vec![1.465489372, 2.362125076], 0),
            (vec![3.396561688, 4.400293529], 0),
            (vec![1.38807019, 1.850220317],  0),
            (vec![3.06407232, 3.005305973],  0),
            (vec![7.627531214, 2.759262235], 1),
            (vec![5.332441248, 2.088626775], 1),
            (vec![6.922596716, 1.77106367],  1),
            (vec![8.675418651, -0.242068655],1),
            (vec![7.673756466, 3.508563011], 1),
        ];
        let examples = dataset.iter().map(|a| a.0.clone()).collect();
        let labels = dataset.iter().map(|a| a.1).collect();

        let mut nn = NeuralNet::new(2, vec![2, 2], Box::new(SIGMOID), false);
        // This empirically proves to be good parameters to train on this
        nn.train(10*20*20, 1, 0.2, examples, labels);

        /*
        println!("")
        println!(nn.evaluate([2, 5]))
        println!(nn.evaluate([2.5, 2.5]))
        println!(nn.evaluate([3, 0]))
        println!(nn.evaluate([6, 0]))
        println!(nn.evaluate([7.5, 2.5]))
        println!(nn.evaluate([9, 5]))
        println!("")
        */
        assert_eq!(0, nn.predict(vec![2.0, 5.0]));
        assert_eq!(0, nn.predict(vec![2.5, 2.5]));
        // This usually fails pretty badly on this one which is not necessarily surprising: there's no example close to it
        //assert_eq!(0, nn.predict(vec![3.0, 0.0]));
        assert_eq!(1, nn.predict(vec![6.0, 0.0]));
        assert_eq!(1, nn.predict(vec![7.5, 2.5]));
        assert_eq!(1, nn.predict(vec![9.0, 5.0]));
    }

    #[test]
    fn train_on_simple_example_relu() {
        // Same example as the previous one, trying to make it work with ReLu.
        // This always end up dying out (all neurons end up never firing).
        // I tried:
        //  - reducing the learning rate
        //  - normalizing input data
        //  - having more layers & neurons per layer ([10, 2], [10, 10, 2])
        //  - using leaky relu instead of relu
        let dataset = vec![
            (vec![2.7810836  -4.0, 2.550537003 -2.5], 0),
            (vec![1.465489372-4.0, 2.362125076 -2.5], 0),
            (vec![3.396561688-4.0, 4.400293529 -2.5], 0),
            (vec![1.38807019 -4.0, 1.850220317 -2.5], 0),
            (vec![3.06407232 -4.0, 3.005305973 -2.5], 0),
            (vec![7.627531214-4.0, 2.759262235 -2.5], 1),
            (vec![5.332441248-4.0, 2.088626775 -2.5], 1),
            (vec![6.922596716-4.0, 1.77106367  -2.5], 1),
            (vec![8.675418651-4.0, -0.242068655-2.5], 1),
            (vec![7.673756466-4.0, 3.508563011 -2.5], 1),
        ];
        let examples = dataset.iter().map(|a| a.0.clone()).collect();
        let labels = dataset.iter().map(|a| a.1).collect();

        let mut nn = NeuralNet::new(2, vec![10, 2], Box::new(ReLu{alpha: 0.01, beta: 1.0, gamma: 0.01, t1: 0.0, t2: 1.0}), false);
        // This empirically proves to be good parameters to train on this
        nn.train(10*20*20, 1, 0.2, examples, labels);

        /*
        println!("")
        println!(nn.evaluate([2, 5]))
        println!(nn.evaluate([2.5, 2.5]))
        println!(nn.evaluate([3, 0]))
        println!(nn.evaluate([6, 0]))
        println!(nn.evaluate([7.5, 2.5]))
        println!(nn.evaluate([9, 5]))
        println!("")
        */
        assert_eq!(0, nn.predict(vec![2.0, 5.0]));
        assert_eq!(0, nn.predict(vec![2.5, 2.5]));
        // This usually fails pretty badly on this one which is not necessarily surprising: there's no example close to it
        //assert_eq!(0, nn.predict(vec![3.0, 0.0]));
        assert_eq!(1, nn.predict(vec![6.0, 0.0]));
        assert_eq!(1, nn.predict(vec![7.5, 2.5]));
        assert_eq!(1, nn.predict(vec![9.0, 5.0]));
    }

    /*
     * This test is both too long and too flaky ...
    #[test]
    fn train_on_xor() {
        let dataset = vec![
            // Both negatives
            (vec![-0.5, -0.5], 1),
            (vec![-0.5, -0.3], 1),
            (vec![-0.5, -0.2], 1),
            (vec![-0.3, -0.5], 1),
            (vec![-0.3, -0.3], 1),
            (vec![-0.3, -0.2], 1),
            (vec![-0.2, -0.5], 1),
            (vec![-0.2, -0.3], 1),
            (vec![-0.2, -0.2], 1),
            // First negative, second positive
            (vec![-0.5, 0.5], 0),
            (vec![-0.5, 0.3], 0),
            (vec![-0.5, 0.2], 0),
            (vec![-0.3, 0.5], 0),
            (vec![-0.3, 0.3], 0),
            (vec![-0.3, 0.2], 0),
            (vec![-0.2, 0.5], 0),
            (vec![-0.2, 0.3], 0),
            (vec![-0.2, 0.2], 0),
            // Both positives
            (vec![0.5, 0.5], 1),
            (vec![0.5, 0.3], 1),
            (vec![0.5, 0.2], 1),
            (vec![0.3, 0.5], 1),
            (vec![0.3, 0.3], 1),
            (vec![0.3, 0.2], 1),
            (vec![0.2, 0.5], 1),
            (vec![0.2, 0.3], 1),
            (vec![0.2, 0.2], 1),
            // First positive, second negative
            (vec![0.5, -0.5], 0),
            (vec![0.5, -0.3], 0),
            (vec![0.5, -0.2], 0),
            (vec![0.3, -0.5], 0),
            (vec![0.3, -0.3], 0),
            (vec![0.3, -0.2], 0),
            (vec![0.2, -0.5], 0),
            (vec![0.2, -0.3], 0),
            (vec![0.2, -0.2], 0),
        ];
        let examples : Vec<_> = dataset.iter().map(|a| a.0.clone()).collect();
        let labels : Vec<_> = dataset.iter().map(|a| a.1).collect();

        let mut nn = NeuralNet::new(2, vec![4, 2], Box::new(SIGMOID), false);
        // This empirically proves to be good parameters to train on this
        nn.train(10*20*20, 10, 1.0, examples.clone(), labels.clone());

        /*
        println!("")
        println!(nn.evaluate([2, 5]))
        println!(nn.evaluate([2.5, 2.5]))
        println!(nn.evaluate([3, 0]))
        println!(nn.evaluate([6, 0]))
        println!(nn.evaluate([7.5, 2.5]))
        println!(nn.evaluate([9, 5]))
        println!("")
        */
        let mut good = 0;
        let mut total = 0;
        for (e, l) in examples.iter().zip(labels.iter()) {
            total += 1;
            if nn.predict(e.clone()) == *l {
                good += 1;
            }
        }

        // Tolerate 25% error
        assert!(good as f64/total as f64 >= 0.75);
    }
    */
    
    #[test]
    fn load_and_save() {
        let tmpdir = tempfile::tempdir().unwrap();
        let tmpfile = format!("{}/{}", tmpdir.path().to_str().unwrap(), "load_and_save");
        let mut nn = NeuralNet::new(2, vec![2, 2], Box::new(RELU), false);
        nn.layers[0][0].weights = vec![0.5, 0.2];
        nn.layers[0][0].bias = -0.1;
        nn.layers[0][1].weights = vec![0.7, 0.6];
        nn.layers[0][1].bias = -0.1;
        nn.layers[1][0].weights = vec![0.7, 0.5];
        nn.layers[1][0].bias = -0.2;
        nn.layers[1][1].weights = vec![0.8, 0.9];
        nn.layers[1][1].bias = -0.3;

        nn.save(&tmpfile);
        let nn2 = NeuralNet::load(&tmpfile);

        // Validate a few parameters from the NN
        assert_eq!(nn.layers.len(), nn2.layers.len());
        for (layer, layer2) in nn.layers.iter().zip(nn2.layers.iter()) {
            assert_eq!(layer.len(), layer2.len());
            for (neuron, neuron2) in layer.iter().zip(layer2.iter()) {
                assert_eq!(neuron.nb_inputs, neuron2.nb_inputs);
                assert_eq!(neuron.weights.len(), neuron2.weights.len());
                for (w, w2) in neuron.weights.iter().zip(neuron2.weights.iter()) {
                    assert_eq!(w, w2);
                }
                assert_eq!(neuron.bias, neuron2.bias);
                // Few arbitrary tests for activation function
                for v in &[-2.0, -0.42, 0.0, 0.25, 0.33, 1.0] {
                    assert_eq!(neuron.activation.value(*v), neuron2.activation.value(*v));
                    assert_eq!(neuron.activation.derivative(*v), neuron2.activation.derivative(*v));
                }
                assert_eq!(neuron.average_gradient, neuron2.average_gradient);
            }
        }
    }
}
