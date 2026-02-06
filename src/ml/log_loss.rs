use std::fmt::Display;

use crate::ml::measurement::Measurement;
use crate::stats::key::Key;

pub struct LogLossTracker<K: Key + Display> {
    measurements: Vec<(K, Measurement)>,
}

impl<K: Key + Display> LogLossTracker<K> {
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

    pub fn serialize(&self) -> String {
        let mut contents = "gameid,probability,outcome,logloss,frequency\n".to_owned();

        let mut i = 0;
        let mut log_loss_acc = 0.0;
        let mut correct_classifications = 0;

        for (k, m) in &self.measurements {
            log_loss_acc += m.log_loss();
            if m.classification_success() {
                correct_classifications += 1;
            }
            i += 1;

            contents.push_str(&format!(
                "{},{},{},{},{}\n",
                k,
                m.probability(),
                m.outcome(),
                log_loss_acc / i as f64,
                correct_classifications as f64 / i as f64
            ));
        }

        contents
    }
}

impl<K: Key + Display> Display for LogLossTracker<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ log_loss: {}, freq: {} }}",
            self.log_loss(),
            self.freq()
        )
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

        let serialized = tracker.serialize();
        let expected = r#"gameid,probability,outcome,logloss,frequency
0,0.5,1,0.6931471805599453,0
1,0.5,0,0.6931471805599453,0
"#;
        assert_eq!(serialized, expected);
    }
}
