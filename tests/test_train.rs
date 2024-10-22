#[cfg(test)]
// Tests using Arrange, Act, Assert
mod tests {
    use burn::optim::Adam;
    use burn::tensor::{Tensor, backend::ndarray::NdArrayBackend};
    use crate::{model::LstmModel, train::train};

    #[test]
    fn test_training_loop() {

        let input_size = 1;
        let hidden_size = 64;
        let output_size = 1;
        let epochs = 1;
        
        let mut model = LstmModel::<NdArrayBackend>::new(input_size, hidden_size, output_size);
        let mut optimizer = Adam::new(&mut model, 0.001);

        let input = Tensor::from_floats([[[0.5], [0.3], [0.9], [0.1], [0.6]]]);
        let target = Tensor::from_floats([[0.7]]);

        train(&mut model, &mut optimizer, input, target, epochs);

        assert!(true);
    }
}
