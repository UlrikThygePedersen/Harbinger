use burn::tensor::backend::Backend;
use burn::tensor::{Tensor, module::Module};
use burn::nn::{self, Module as BurnModule, Param, Lstm, Linear, ReLU};
use burn::optim::{Adam, Optimizer};
use burn::nn::Loss;

#[derive(Module, Debug)]
pub struct LstmModel<B: Backend> {
    lstm: Lstm<B>,
    linear: Linear<B>,
    relu: ReLU,
}

impl<B: Backend> LstmModel<B> {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        Self {
            lstm: Lstm::new(input_size, hidden_size),
            linear: Linear::new(hidden_size, output_size),
            relu: ReLU::new(),
        }
    }

    pub fn forward(&self, input: Tensor<B, 3>) -> Tensor<B, 2> {
        let (output, _hidden_state) = self.lstm.forward(input);
        let output = self.linear.forward(output);
        self.relu.forward(output)
    }
}
