use crate::ml::vector::Vector;

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
}
