pub trait Measureable {
    fn into_measurement(&self) -> Measurement;
}

pub struct Measurement {
    actual: u8, //assert 0 or 1
    prob: f64,
}

impl Measurement {
    pub fn new(actual: u8, prob: f64) -> Self {
        assert!(actual == 0 || actual == 1);
        assert!(prob > 0.0 && prob < 1.0);

        Measurement { actual, prob }
    }

    pub fn log_loss(&self) -> f64 {
        -1f64
            * (self.actual as f64 * self.prob.ln()
                + (1f64 - self.actual as f64) * (1f64 - self.prob).ln())
    }

    pub fn classification_success(&self) -> bool {
        if self.actual == 1 && self.prob > 0.5 || self.actual == 0 && self.prob < 0.5 {
            true
        } else if self.prob == 0.5 {
            false //dont reward not making a prediction
        } else {
            false
        }
    }
}
