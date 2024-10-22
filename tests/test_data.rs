#[cfg(test)]
// Tests using Arrange, Act, Assert
mod tests {
    use crate::data;

    #[test]
    fn test_data_loading() {
        let file_path = "assets/sample_stock_data.csv";
        
        let result = data::load_data(file_path);


        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(data.len() > 0);
    }

    #[test]
    fn test_data_normalization() {
        let data = ndarray::array![[1.0], [2.0], [3.0], [4.0], [5.0]];

        let (normalized_data, min, max) = data::normalize(data);

        assert!(min < max);
        assert_eq!(normalized_data[(0, 0)], 0.0);
        assert_eq!(normalized_data[(4, 0)], 1.0); 
    }
}
