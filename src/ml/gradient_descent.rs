use crate::ml::vector::Vector;

pub trait GradientDescent {
    fn gradient_descent(&mut self, data: &[(Vector, u8)], learning_rate: f64);
}
