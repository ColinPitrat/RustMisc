extern crate rand;
extern crate assert_approx_eq;

use assert_approx_eq::assert_approx_eq;
use crate::activation::{ActivationFunction, RELU};
use rand::prelude::*;

pub struct Neuron<'a> {
    // Neuron properties
    pub nb_inputs: usize,
    pub weights: Vec<f64>,
    pub bias: f64,
    pub activation: &'a ActivationFunction,

    // Option: whether to average gradient on the batch or sum it
    pub average_gradient: bool,

    // Used for backpropagation of last value
    pub last_value: f64,
    pub last_input: Vec<f64>,

    // Back propagation accumulators
    pub dw: Vec<f64>,
    pub da: Vec<f64>,
    pub db: f64,
    pub nb_evals: usize,
}

impl<'a> Neuron<'a> {
    pub fn new(nb_inputs: usize, activation: &ActivationFunction, average_gradient: bool) -> Neuron {
        let mut rng = rand::thread_rng();
        let weights: Vec<f64> = (0..nb_inputs).map(|_| {
            rng.gen::<f64>()*2.0 - 1.0 // A number between -1.0 and 1.0
        }).collect();
        let bias = rng.gen::<f64>()*2.0 - 1.0;

        // Back propagation members are all initialized empty/0
        let last_input = vec!();
        let last_value = 0.0;
        let dw = vec!();
        let da = vec!();
        let db = 0.0;
        let nb_evals = 0;
        Neuron{
            nb_inputs, weights, bias, activation, average_gradient,
            last_input, last_value,
            dw, da, db, nb_evals,
        }
    }

    pub fn output(&mut self, previous_layer_values: Vec<f64>, for_training: bool) -> f64 {
        let mut result = self.bias;
        for i in 0..self.nb_inputs {
            result += self.weights[i]*previous_layer_values[i]
        }
        if for_training {
            // Save some info for per-eval backpropagation
            self.last_value = result;
            self.last_input = previous_layer_values;
        }
        return self.activation.value(result)
    }

    pub fn prepare_backprop(&mut self) {
        self.dw = vec![0.0; self.nb_inputs];
        self.da = vec![0.0; self.nb_inputs];
        self.db = 0.0;
        self.nb_evals = 0;
    }

    pub fn per_eval_backprop(&mut self, error: f64, learning_rate: f64) -> Vec<f64> {
        self.nb_evals += 1;
        let delta = 2.0*error * self.activation.derivative(self.last_value);
        self.db += delta * learning_rate;
        for i in 0..self.nb_inputs {
            self.dw[i] += delta*self.last_input[i]*learning_rate;
            self.da[i] += delta*self.weights[i];
        }
        self.da.clone()
    }

    pub fn per_round_backprop(&mut self) {
        let mut denominator = 1.0;
        if self.average_gradient {
            denominator = self.nb_evals as f64;
        }
        self.bias += self.db / denominator;
        for i in 0..self.nb_inputs {
            self.weights[i] += self.dw[i] / denominator;
        }
        self.prepare_backprop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_input_neuron_activation() {
        let mut n = Neuron::new(1, &RELU, false);
        n.bias = -0.5;
        n.weights = vec![0.7];

        // 1.0*0.7 - 0.5 = 0.2
        assert_approx_eq!(0.2, n.output(vec![1.0], false))
    }

    #[test]
    fn multiple_inputs_neuron_activation() {
        let mut n = Neuron::new(3, &RELU, false);
        n.bias = -0.5;
        n.weights = vec![0.7, 0.5, 0.3];

        // 0.7*0.7 + 0.5*0.5 + 0.3*0.3 - 0.5 = 0.49 + 0.25 + 0.09 - 0.5 = 0.83 - 0.5 = 0.33
        assert_approx_eq!(0.33, n.output(vec![0.7, 0.5, 0.3], false))
    }

    #[test]
    fn backpropagate_error_on_single_neuron() {
        let mut n = Neuron::new(1, &RELU, false);
        n.bias = -0.5;
        n.weights = vec![0.7];
        assert_approx_eq!(0.2, n.output(vec![1.0], true));
        n.prepare_backprop();

        // Assume expected output was 1 -> provide error of 0.8
        let da = n.per_eval_backprop(0.8, 1.0);

        assert_approx_eq!(1.6, n.db);
        assert_approx_eq!(1.6, n.dw[0]);
        assert_approx_eq!(1.12, da[0]);

        n.per_round_backprop();

        assert_approx_eq!(1.1, n.bias);
        assert_approx_eq!(2.3, n.weights[0]);
        // Accumulators are reset by per_round_backprop
        assert_approx_eq!(0.0, n.db);
        assert_approx_eq!(0.0, n.dw[0]);
        assert_approx_eq!(0.0, n.da[0]);
    }
}
