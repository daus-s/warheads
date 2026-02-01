use std::fmt::Display;

use serde::Serialize;

use crate::ml::measurement::Measurement;
use crate::stats::key::Key;

pub struct LogLossTracker<K: Key> {
    measurements: Vec<(K, Measurement)>,
}

impl<K: Key> LogLossTracker<K> {
    pub fn log_loss(&self) -> f64 {
        let count = self.measurements.len();
        if count == 0 {
            0.0
        } else {
            self.measurements
                .iter()
                .fold(0.0, |acc, (_k, m)| acc + m.log_loss())
                / count as f64
        }
    }

    /// return the frequency of the models performance based on whether its naive? classification
    /// would create the best prediction.
    pub fn freq(&self) -> f64 {
        let mut successes = 0;

        for (_k, measurement) in &self.measurements {
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

    pub fn add_measurement(&mut self, k: K, m: Measurement) {
        self.measurements.push((k, m));
    }

    pub fn is_empty(&self) -> bool {
        self.measurements.is_empty()
    }
}

impl<K: Key> Display for LogLossTracker<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ log_loss: {}, freq: {} }}",
            self.log_loss(),
            self.freq()
        )
    }
}

impl<K: Key> Serialize for LogLossTracker<K> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = serde_json::json!({
            "log_loss": self.log_loss(),
            "freq": self.freq()
        });

        value.serialize(serializer)
    }
}

#[cfg(test)]
mod serialize_log_loss {
    use super::*;

    impl Key for u8 {}

    #[test]
    fn test_serialize_log_loss() {
        let mut tracker = LogLossTracker::new();
        tracker.add_measurement(0, Measurement::new(1, 0.5));
        tracker.add_measurement(1, Measurement::new(0, 0.5));

        let serialized = serde_json::to_string(&tracker).unwrap();
        let expected = r#"{"log_loss":0.6931471805599453,"freq":0.0}"#;
        assert_eq!(serialized, expected);
    }
}
