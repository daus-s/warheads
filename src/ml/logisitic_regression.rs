use crate::ml::gradient_descent::GradientDescent;
use crate::ml::vector::Vector;

/// binary logistic regression model implementation
pub struct LogisticRegression {
    params: Vector,
    bias: f64,
}

impl LogisticRegression {
    pub fn new(params: Vector, bias: f64) -> Self {
        Self { params, bias }
    }

    pub fn predict(&self, x: &Vector) -> f64 {
        let z = x.dot(&self.params) + self.bias;

        1.0 / (1.0 + f64::exp(-z))
    }
}

impl GradientDescent for LogisticRegression {
    #[allow(non_snake_case)]
    fn gradient_descent(&mut self, data: &[(Vector, u8)], learning_rate: f64) {
        for (input, output) in data {
            let p = self.predict(input);

            let y = *output as f64;

            let diff = p - y;

            let dLdW = input * diff * learning_rate;
            let dLdB = diff * learning_rate;

            self.params -= dbg!(dLdW);
            self.bias -= dbg!(dLdB);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_origin() {
        let log = LogisticRegression::new(Vector::origin(1), 0.0);

        assert!(log.predict(&Vector::origin(1)) == 0.5)
    }

    #[test]
    fn test_predict_primes() {
        let log = LogisticRegression::new(Vector::from(vec![2f64, 3f64]), 0.0);

        assert!(log.predict(&Vector::from(vec![5f64, 7f64])) == 1f64 / (1f64 + f64::exp(-31f64)))
    }

    #[test]
    fn test_gradient_descent_step() {
        let mut log = LogisticRegression::new(Vector::origin(1), 0.0);
        let data = [(Vector::from(vec![1f64]), 1u8)];
        log.gradient_descent(&data, 0.1);

        assert_eq!(log.params, Vector::from(vec![0.05]));
        assert_eq!(log.bias, 0.05);
    }
}
