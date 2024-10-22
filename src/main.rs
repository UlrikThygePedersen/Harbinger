mod model;
mod train;
mod predict;
mod data;

use burn::tensor::backend::ndarray::NdArrayBackend;
use burn::optim::Adam;
use burn::tensor::Tensor;
use ndarray::{Array2};

// Main function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load and preprocess data
    let file_path = "assets/stock_details_5_years.csv"; // Path to your dataset
    let raw_data = data::load_data(file_path)?;  // Load stock price data
    let (normalized_data, min, max) = data::normalize(raw_data);  // Normalize the data

    // Create sequences for training (e.g., sequence length of 30)
    let sequence_length = 30;
    let sequences = data::create_sequences(normalized_data, sequence_length);
    
    // Prepare data for LSTM input (reshape to match input format)
    let input = Tensor::from_ndarray(sequences.slice(s![.., ..sequence_length]).to_owned());
    let target = Tensor::from_ndarray(sequences.slice(s![.., sequence_length..]).to_owned());

    // Initialize model and optimizer
    let input_size = 1; // 1 feature (stock price)
    let hidden_size = 64;
    let output_size = 1; // Predicting 1 future value
    let epochs = 100;

    let mut model = model::LstmModel::new(input_size, hidden_size, output_size);
    let mut optimizer = Adam::new(&mut model, 0.001);

    // Train the model
    train::train(&mut model, &mut optimizer, input.clone(), target.clone(), epochs);

    // Make predictions (for simplicity, predicting on the same data here)
    let prediction = predict::predict(&model, input);
    println!("Prediction: {:?}", prediction);

    Ok(())
}
