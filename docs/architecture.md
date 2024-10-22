# Harbinger Architecture

## Overview

Harbinger is a time series forecasting model built using the **Burn** framework in Rust. It leverages Long Short-Term Memory (LSTM) networks to model sequential dependencies in stock data and make future predictions. This document provides a breakdown of the architecture, highlighting each major module and its role in the system.

---

## Main Components

### 1. **LSTM Model (`model.rs`)**
   - **Purpose**: The LSTM model is the core of Harbinger’s predictive capabilities. It processes sequences of stock data and generates future stock price predictions.
   - **Key Functions**:
     - `forward()`: This function performs the forward pass through the LSTM layers, followed by fully connected layers.
     - `new()`: Initializes the LSTM layers and dense layers with given input/output sizes.
   - **Inputs**:
     - Sequences of stock prices (e.g., the past 30 days of closing prices).
   - **Outputs**:
     - The predicted stock price for the next time step.

### 2. **Data Preprocessing (`data.rs`)**
   - **Purpose**: Handles data loading, normalization, and the creation of training sequences from the raw stock data. 
   - **Key Functions**:
     - `load_data()`: Loads stock data from a CSV file and extracts relevant columns.
     - `normalize()`: Scales stock prices to a range between 0 and 1 for efficient model training.
     - `create_sequences()`: Generates sequences of past prices to feed into the model.
   - **Inputs**:
     - Raw stock data in CSV format.
   - **Outputs**:
     - Preprocessed and normalized data sequences ready for training.

### 3. **Training (`train.rs`)**
   - **Purpose**: Manages the training loop, calculating loss and updating model parameters.
   - **Key Functions**:
     - `train()`: Runs the training loop, performs forward and backward passes, and updates model weights.
   - **Inputs**:
     - Model, optimizer, input sequences, and target values (actual stock prices).
   - **Outputs**:
     - A trained model capable of making accurate predictions on unseen data.

### 4. **Prediction (`predict.rs`)**
   - **Purpose**: Handles model inference on unseen data.
   - **Key Functions**:
     - `predict()`: Takes input sequences and generates predicted stock prices.
   - **Inputs**:
     - Preprocessed sequences of stock data.
   - **Outputs**:
     - Predicted future stock prices.

---

## Data Flow

1. **Data Loading**: The stock data is loaded from a CSV file.
2. **Preprocessing**: The data is normalized and transformed into sequences.
3. **Model Training**: The LSTM model is trained on these sequences, minimizing the loss.
4. **Prediction**: Once trained, the model can make predictions on new sequences of stock data.
5. **Results**: Predictions can be compared with actual stock prices to evaluate accuracy.

---

## Model Summary

- **Input Size**: 1 (for stock price).
- **Hidden Size**: 64 (can be tuned).
- **Output Size**: 1 (for predicting the next price).
- **Loss Function**: Mean Squared Error (MSE).
- **Optimizer**: Adam optimizer for fast convergence.

---

## Conclusion

This architecture balances simplicity with effectiveness, providing an efficient solution for time series forecasting using Rust and Burn. The modularity ensures that components can be easily swapped or improved as needed.

