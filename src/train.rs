use burn::tensor::Tensor;
use burn::optim::Optimizer;
use burn::nn::Loss;

pub fn train<B: Backend>(
    model: &mut LstmModel<B>, 
    optimizer: &mut Adam<B>, 
    input: Tensor<B, 3>, 
    target: Tensor<B, 2>, 
    epochs: usize
) {
    for epoch in 0..epochs {
        let prediction = model.forward(input.clone());

        let loss = Loss::mse(prediction, target.clone());

        optimizer.zero_grad();
        loss.backward();
        optimizer.step();

        println!("Epoch: {}, Loss: {:?}", epoch, loss);
    }
}
