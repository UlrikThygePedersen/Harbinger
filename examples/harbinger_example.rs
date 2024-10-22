use burn::optim::Adam;
use burn::tensor::{Tensor, backend::ndarray::NdArrayBackend};
use ndarray::Array2;
use harbinger::data::{load_data, normalize, create_sequences};
use harbinger::model::LstmModel;
use harbinger::train::train;
use harbinger::predict::predict;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Path to the dataset
    let file_path = "assets/stock_details_5_years.csv";

    // Step 1: Load and preprocess data
    let raw_data = load_data(file_path)?;  
    let (normalized_data, min, max) = normalize(raw_data);  // Normalize the data

    // Step 2: Create sequences for training (e.g., sequence length of 30)
    let sequence_length = 30;
    let sequences = create_sequences(normalized_data, sequence_length);

    // Prepare input and target tensors for training
    let input = Tensor::from_ndarray(sequences.slice(s![.., ..sequence_length]).to_owned());
    let target = Tensor::from_ndarray(sequences.slice(s![.., sequence_length..]).to_owned());

    // Step 3: Initialize the LSTM model and optimizer
    let input_size = 1; // 1 feature (stock price)
    let hidden_size = 64;
    let output_size = 1; // Predicting 1 future value
    let epochs = 100;

    let mut model = LstmModel::new(input_size, hidden_size, output_size);
    let mut optimizer = Adam::new(&mut model, 0.001);

    // Step 4: Train the model
    train(&mut model, &mut optimizer, input.clone(), target.clone(), epochs);

    // Step 5: Make predictions
    let prediction = predict(&model, input);
    println!("Prediction: {:?}", prediction);

    // Step 6: Optionally, denormalize the prediction to original scale
    let prediction_denorm = (prediction * (max - min)) + min;
    println!("Denormalized Prediction: {:?}", prediction_denorm);

    Ok(())
}
