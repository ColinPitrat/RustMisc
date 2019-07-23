use crate::neuron::Neuron;
use crate::activation::ActivationFunction;

pub struct NeuralNet<'a> {
    layers: Vec<Vec<Neuron<'a>>>,
}

impl<'a> NeuralNet<'a> {
    fn new(inputs_size: usize, layers_sizes: Vec<usize>, activation: &ActivationFunction, average_gradient: bool) -> NeuralNet {
        let mut layers = vec!();
        let mut previous_size = inputs_size;
        for size in layers_sizes {
            layers.push((0..size).map(|_| Neuron::new(previous_size, activation, average_gradient)).collect());
            previous_size = size;
        }
        NeuralNet{layers}
    }

    fn evaluate(&mut self, input: Vec<f64>, for_training: bool) -> Vec<f64> {
        let mut result = input;
        for layer in self.layers.iter_mut() {
            result = layer.iter_mut().map(|neuron| neuron.output(result.clone(), for_training)).collect();
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use crate::activation::RELU;

    #[test]
    fn multiple_layers_network_activation() {
        let mut nn = NeuralNet::new(2, vec![2, 2], &RELU, false);
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
}
