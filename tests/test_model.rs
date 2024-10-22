#[cfg(test)]
// Tests using Arrange, Act, Assert
mod tests {
    use burn::tensor::{Tensor, backend::ndarray::NdArrayBackend};
    use crate::model::LstmModel;
    
    #[test]
    fn test_lstm_forward_pass() {
        let input_size = 1;
        let hidden_size = 64;
        let output_size = 1;
        let model = LstmModel::<NdArrayBackend>::new(input_size, hidden_size, output_size);

        let input_data = Tensor::from_floats([[0.5], [0.3], [0.9], [0.1], [0.6], [0.2]]);
        let input = input_data.reshape([1, 6, 1]);

        let output = model.forward(input);

        assert_eq!(output.dims(), [1, 1]);
    }
}
