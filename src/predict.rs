use burn::tensor::Tensor;

pub fn predict<B: Backend>(
    model: &LstmModel<B>, 
    input: Tensor<B, 3>
) -> Tensor<B, 2> {
    model.forward(input)
}
