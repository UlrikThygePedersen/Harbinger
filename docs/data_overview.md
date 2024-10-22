# Dataset Overview: Massive Yahoo Finance Dataset

Harbinger uses the **Massive Yahoo Finance Dataset** for training and evaluating the time series forecasting model. This dataset includes historical stock data from multiple companies over a span of 5 years.

---

## Data Fields

The dataset contains the following fields:
- **Date**: The trading date.
- **Open**: Opening price of the stock on that day.
- **High**: Highest price reached during the trading day.
- **Low**: Lowest price during the trading day.
- **Close**: The closing price of the stock (used as the target for predictions).
- **Volume**: Number of shares traded during the day.

---

## Preprocessing

The dataset is preprocessed as follows:
1. **Normalization**: Stock prices are scaled between 0 and 1 for better training performance.
2. **Sequence Generation**: The data is split into sequences of 30 days (configurable) to provide context for the model’s predictions.

---

## Dataset Usage

- **Training**: The model uses the closing prices to train on sequences of past prices and predict the next closing price.
- **Testing**: After training, the model is evaluated on a separate portion of the dataset that wasn’t used during training.

---

## Source

The dataset is available in the `/assets` folder as `stock_details_5_years.csv`. Make sure the file is present before running the model.

