use std::fmt::Debug;

#[derive(Debug)]
pub struct Observation {
    actual: u8, //assert 0 or 1
    prob: f64,
}

impl Observation {
    pub fn new(actual: u8, prob: f64) -> Self {
        assert!(
            actual == 0 || actual == 1,
            "💀 outcome must be either 0 or 1. {}",
            actual
        );
        assert!(
            prob > 0.0 && prob < 1.0,
            "💀 probability must be between 0 and 1: {}",
            prob
        );

        Observation { actual, prob }
    }

    pub fn actual(&self) -> u8 {
        self.actual
    }

    pub fn prob(&self) -> f64 {
        self.prob
    }
}
