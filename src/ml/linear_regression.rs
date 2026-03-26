use crate::ml::vector::Vector;

pub struct LinearRegression {
    params: Vector,
    bias: f64,
}

impl LinearRegression {
    pub fn new(params: Vector, bias: f64) -> Self {
        Self { params, bias }
    }

    pub fn predict(&self, x: &Vector) -> f64 {
        self.params.dot(x) + self.bias
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_origin() {
        let log = LinearRegression::new(Vector::from(vec![1f64]), 0.5);

        assert!(log.predict(&Vector::from(vec![1f64])) == 1.5)
    }
}
