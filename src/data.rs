use std::fs::File;
use csv::Reader;
use ndarray::{Array2, Array};
use std::error::Error;

// Load the dataset from CSV
pub fn load_data(file_path: &str) -> Result<Array2<f32>, Box<dyn Error>> {
    let mut reader = Reader::from_reader(File::open(file_path)?);
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let close_price: f32 = record.get(4).unwrap().parse().unwrap();
        records.push(close_price);
    }

    let data = Array::from_shape_vec((records.len(), 1), records)?;

    Ok(data)
}

// Normalize the data
pub fn normalize(data: Array2<f32>) -> (Array2<f32>, f32, f32) {
    let min = data.min().unwrap();
    let max = data.max().unwrap();
    let normalized = (data - min) / (max - min);
    (normalized, min, max)
}

// Split the dataset into sequences for training
pub fn create_sequences(data: Array2<f32>, sequence_length: usize) -> Array2<f32> {
    let mut sequences = Vec::new();

    for i in 0..(data.len() - sequence_length) {
        let seq = data.slice(s![i..i + sequence_length, ..]).to_owned();
        sequences.push(seq.into_raw_vec());
    }
    
    Array::from_shape_vec((sequences.len(), sequence_length), sequences.concat()).unwrap()
}
