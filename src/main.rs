mod model;
mod train;
mod predict;

use burn::tensor::backend::ndarray::NdArrayBackend;
use burn::optim::Adam;
use burn::nn::{Param, Lstm, Linear};
use ndarray::{Array, Array2};

fn main() {
    // Hyperparameters
    let input_size = 1;
    let hidden_size = 64;
    let output_size = 1;
    let epochs = 100;
    
    // Initialize model, optimizer
    let mut model = model::LstmModel::new(input_size, hidden_size, output_size);
    let mut optimizer = Adam::new(&mut model, 0.001);

    // Dummy data (will be replaced with actual stock price data)
    let input = Tensor::from_ndarray(Array::zeros((10, 30, 1)));  // Example shape: (batch, sequence, features)
    let target = Tensor::from_ndarray(Array::zeros((10, 1)));     // Example shape: (batch, output)

    // Train the model
    train::train(&mut model, &mut optimizer, input.clone(), target.clone(), epochs);

    // Make predictions
    let prediction = predict::predict(&model, input);
    println!("Prediction: {:?}", prediction);
}
