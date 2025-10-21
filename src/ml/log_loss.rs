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
