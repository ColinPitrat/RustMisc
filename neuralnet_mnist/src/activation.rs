#[cfg(test)] extern crate assert_approx_eq;

#[cfg(test)] use assert_approx_eq::assert_approx_eq;
use serde::{Serialize,Deserialize};

pub const RELU: ReLu = ReLu{alpha: 0.0, beta: 0.0, gamma: 1.0, t1: 0.0, t2: 0.0};
pub const RELU6: ReLu = ReLu{alpha: 0.0, beta: 1.0, gamma: 0.0, t1: 0.0, t2: 6.0};
pub const LEAKYRELU: ReLu = ReLu{alpha: 0.01, beta: 0.0, gamma: 1.0, t1: 0.0, t2: 0.0};
pub const SIGMOID: Sigmoid = Sigmoid{};
pub const TANH: TanH = TanH{};

#[typetag::serde]
pub trait ActivationFunction {
    fn value(&self, x: f64) -> f64;
    fn derivative(&self, x: f64) -> f64;
    fn name(&self) -> String;
}

#[derive(Serialize,Deserialize)]
pub struct ReLu {
    // Slope before t1
    pub alpha: f64,
    // Slope between t1 and t2
    pub beta: f64,
    // Slope after t2
    pub gamma: f64,

    pub t1: f64,
    pub t2: f64,
}

#[typetag::serde]
impl ActivationFunction for ReLu {
    fn value(&self, x: f64) -> f64 {
        if x > self.t2 {
            self.gamma*x
        } else if x > self.t1 {
            self.beta*x
        } else {
            self.alpha*x
        }
    }

    fn derivative(&self, x: f64) -> f64 {
        if x > self.t2 {
            self.gamma
        } else if x > self.t1 {
            self.beta
        } else {
            self.alpha
        }
    }

    fn name(&self) -> String {
        format!("ReLu({} <{}, {} <{}, {})", self.alpha, self.t1, self.beta, self.t2, self.gamma)
    }
}

#[derive(Serialize,Deserialize)]
pub struct Sigmoid;

#[typetag::serde]
impl ActivationFunction for Sigmoid {
    fn value(&self, x: f64) -> f64 {
        1.0/(1.0 + (-x).exp())
    }

    fn derivative(&self, x: f64) -> f64 {
        // exp overflows after 709
        if x < -709.0 {
            0.0
        } else {
            let e_x = (-x).exp();
            e_x/(1.0+2.0*e_x+e_x*e_x)
        }
    }

    fn name(&self) -> String {
        String::from("Sigmoid")
    }
}

#[derive(Serialize,Deserialize)]
pub struct TanH;

#[typetag::serde]
impl ActivationFunction for TanH {
    fn value(&self, x: f64) -> f64 {
        x.tanh()
    }

    fn derivative(&self, x: f64) -> f64 {
        let th = x.tanh();
        1.0-th*th
    }

    fn name(&self) -> String {
        String::from("TanH")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relu_value() {
        assert_eq!(0.0, RELU.value(-1.0));
        assert_eq!(0.0, RELU.value(-0.5));
        assert_eq!(0.0, RELU.value(0.0));
        assert_eq!(0.5, RELU.value(0.5));
        assert_eq!(1.0, RELU.value(1.0));
    }

    #[test]
    fn relu_derivative() {
        assert_eq!(0.0, RELU.derivative(-1.0));
        assert_eq!(0.0, RELU.derivative(-0.5));
        assert_eq!(0.0, RELU.derivative(0.0));
        assert_eq!(1.0, RELU.derivative(0.5));
        assert_eq!(1.0, RELU.derivative(1.0));
    }

    #[test]
    fn sigmoid_value() {
        assert_approx_eq!(0.26894, SIGMOID.value(-1.0), 1e-5f64);
        assert_approx_eq!(0.5, SIGMOID.value(0.0));
        assert_approx_eq!(0.73106, SIGMOID.value(1.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.value(-710.0), 1e-5f64);
        assert_approx_eq!(1.0, SIGMOID.value(710.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.value(-1420.0), 1e-5f64);
        assert_approx_eq!(1.0, SIGMOID.value(1420.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.value(-1234567890.0), 1e-5f64);
        assert_approx_eq!(1.0, SIGMOID.value(1234567890.0), 1e-5f64);
    }

    #[test]
    fn sigmoid_derivative() {
        assert_approx_eq!(0.19661, SIGMOID.derivative(-1.0), 1e-5f64);
        assert_approx_eq!(0.25, SIGMOID.derivative(0.0));
        assert_approx_eq!(0.19661, SIGMOID.derivative(1.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(-709.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(709.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(-710.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(710.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(-1234567890.0), 1e-5f64);
        assert_approx_eq!(0.0, SIGMOID.derivative(1234567890.0), 1e-5f64);
    }

    #[test]
    fn tanh_value() {
        assert_approx_eq!(-0.964027, TANH.value(-2.0));
        assert_approx_eq!(-0.761594, TANH.value(-1.0));
        assert_approx_eq!(-0.462117, TANH.value(-0.5));
        assert_approx_eq!(0.0, TANH.value(0.0));
        assert_approx_eq!(0.462117, TANH.value(0.5));
        assert_approx_eq!(0.761594, TANH.value(1.0));
        assert_approx_eq!(0.964027, TANH.value(2.0));
    }

    #[test]
    fn tanh_derivative() {
        assert_approx_eq!(0.070651, TANH.derivative(-2.0));
        assert_approx_eq!(0.419974, TANH.derivative(-1.0));
        assert_approx_eq!(0.786448, TANH.derivative(-0.5));
        assert_approx_eq!(1.0, TANH.derivative(0.0));
        assert_approx_eq!(0.786448, TANH.derivative(0.5));
        assert_approx_eq!(0.419974, TANH.derivative(1.0));
        assert_approx_eq!(0.070651, TANH.derivative(2.0));
    }
}
