<img src="assets/forecast.png" alt="Stock Prediction Image"/>

# Welcome to Harbinger 🔮🧙‍♂️✨

**Harbinger** is a high-performance time series forecasting model built using the [**Burn**](https://burn.dev/) deep learning framework in **Rust**. This project focuses on predicting stock prices using an LSTM (Long Short-Term Memory) model to capture trends and forecast future values based on historical stock data.

This project showcases how advanced AI models can be applied in real-world financial contexts, emphasizing speed, memory safety, and scalability through Rust. It also demonstrates my ability to work with cutting-edge tools and technologies while delivering accurate results for time-sensitive and resource-intensive tasks.

---

## Project Overview

Harbinger uses **time series forecasting** to predict stock prices based on historical data. The model is trained using daily stock prices from the Massive Yahoo Finance Dataset, leveraging an **LSTM-based architecture** to account for sequential dependencies in stock movements. This project demonstrates my expertise in applying deep learning methods for forecasting while showcasing the performance advantages of Rust.

The **goal** is to predict the closing stock prices for the next day (or future days) based on a rolling window of past prices, making this an excellent example of how deep learning can provide insights in financial forecasting.

---

## Key Features

### 1. **Time Series Forecasting Using LSTM**
   - **Scenario**: Predict the next day’s closing stock price using the past 30 days of stock data.
   - **Benefit**: Provides a real-world demonstration of how deep learning can be applied to financial data for trend prediction and decision-making.

### 2. **Scalable and Fast Training with Burn**
   - **Scenario**: Leverages the **Burn** deep learning framework in Rust to handle large volumes of data efficiently, while ensuring memory safety.
   - **Benefit**: Rust’s performance allows for faster training times and better resource management compared to traditional deep learning frameworks like TensorFlow or PyTorch.

### 3. **Normalized Data for Accurate Predictions**
   - **Scenario**: Data preprocessing includes normalization and lag feature creation to provide the model with the right inputs for capturing trends in stock prices.
   - **Benefit**: Ensures that the model learns from patterns without being affected by large fluctuations in the absolute values of stock prices.

### 4. **Prediction and Visualization**
   - **Scenario**: After training, the model generates predictions that can be visually compared to actual stock prices, highlighting the model’s accuracy.
   - **Benefit**: This allows for better interpretability of the model’s output, providing both numerical and graphical insights.

---

## Technical Architecture

Harbinger is built on the [**Burn framework**](https://github.com/tracel-ai/burn), a modular and performant deep learning library in Rust. The architecture leverages LSTM (Long Short-Term Memory) layers to capture temporal dependencies in stock prices and predict future values.

### Key Components:
1. **LSTM Model**: Processes the sequential stock data, capturing patterns and trends over time.
2. **Fully Connected (Dense) Layers**: Maps the LSTM output to a single prediction value (next day’s stock price).
3. **Loss Function**: Mean Squared Error (MSE) is used to measure the prediction accuracy and optimize the model.
4. **Optimizer**: Adam Optimizer ensures fast convergence during training by adjusting learning rates dynamically.

---

## Use Case Scenarios

### **Financial Forecasting**
   - **Scenario**: The model is designed to predict the future closing price of stocks based on historical trends.
   - **Benefit**: Helps financial analysts and traders make data-driven decisions about stock purchases and sales.

### **Portfolio Management**
   - **Scenario**: Harbinger can be integrated into a larger portfolio management system to forecast stock movements and adjust investment strategies dynamically.
   - **Benefit**: Maximizes portfolio returns by predicting potential stock price changes.

### **Risk Assessment**
   - **Scenario**: By forecasting stock prices, Harbinger can be used to assess the risk associated with volatile assets or market conditions.
   - **Benefit**: Provides early warnings for high-risk stock movements, allowing risk mitigation strategies to be put in place.

---

## Future Work

- **Multi-Step Forecasting**: Extend the model to predict stock prices for multiple future days rather than just the next day.
- **Feature Engineering**: Incorporate additional features such as trading volume, market indicators, or moving averages to improve prediction accuracy.
- **Real-Time Predictions**: Implement real-time stock prediction by integrating with live data feeds.

---

## Conclusion

Harbinger demonstrates the application of deep learning in financial forecasting, highlighting my skills in working with Rust, Burn, and time series data. This project showcases my ability to build scalable, efficient machine learning models that solve real-world problems.

By combining state-of-the-art deep learning techniques with the performance and safety of Rust, Harbinger stands out as a high-quality solution in the financial domain.
