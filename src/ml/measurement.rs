use serde::Serialize;

pub trait Measureable {
    fn into_measurement(&self) -> Measurement;
}

#[derive(Debug, Clone, Serialize)]
pub struct Measurement {
    outcome: u8, //assert 0 or 1
    probability: f64,
}

impl Measurement {
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

        Measurement {
            outcome: actual,
            probability: prob,
        }
    }

    // L = = -(y⋅ln*(p))+(1-y⋅ln*(1-p))
    pub fn log_loss(&self) -> f64 {
        -1f64
            * (self.outcome as f64 * self.probability.ln()
                + (1f64 - self.outcome as f64) * (1f64 - self.probability).ln())
    }

    pub fn classification_success(&self) -> bool {
        if self.outcome == 1 && self.probability > 0.5
            || self.outcome == 0 && self.probability < 0.5
        {
            true
        } else {
            false
        }
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }

    pub fn outcome(&self) -> u8 {
        self.outcome
    }
}
