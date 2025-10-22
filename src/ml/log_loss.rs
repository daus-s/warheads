use std::fmt::Display;

use crate::ml::measurement::{Measureable, Measurement};

pub struct LogLossTracker {
    measurements: Vec<Measurement>,
}

impl LogLossTracker {
    pub fn log_loss(&self) -> f64 {
        let count = self.measurements.len();
        if count == 0 {
            0.0
        } else {
            self.measurements
                .iter()
                .fold(0.0, |acc, m| acc + m.log_loss())
                / count as f64
        }
    }

    /// return the frequency of the models performance based on whether its naive? classification
    /// would create the best prediction.
    pub fn freq(&self) -> f64 {
        let mut successes = 0;

        for measurement in &self.measurements {
            if measurement.classification_success() {
                successes += 1;
            }
        }
        successes as f64 / self.measurements.len() as f64
    }

    pub fn new() -> Self {
        LogLossTracker {
            measurements: Vec::new(),
        }
    }

    pub fn add_measurement(&mut self, m: Measurement) {
        self.measurements.push(m);
    }

    pub fn add_measurable(&mut self, event: &Box<dyn Measureable>) {
        self.add_measurement(event.into_measurement());
    }
}

impl Display for LogLossTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LogLossTracker {{ log_loss: {}, freq: {} }}",
            self.log_loss(),
            self.freq()
        )
    }
}
