use std::fmt::Display;

use serde::Serialize;

use crate::ml::measurement::{Measureable, Measurement};

pub struct LogLossTracker {
    log_loss: f64,
    freq: u64,
    length: u64,
}

impl LogLossTracker {
    pub fn log_loss(&self) -> f64 {
        let count = self.length;
        if count == 0 {
            f64::NAN
        } else {
            self.log_loss / self.length as f64
        }
    }

    /// return the frequency of the models performance based on whether its naive? classification
    /// would create the best prediction.
    pub fn freq(&self) -> f64 {
        self.freq as f64 / self.length as f64
    }

    pub fn new() -> Self {
        LogLossTracker {
            log_loss: 0f64,
            freq: 0,
            length: 0,
        }
    }

    pub fn add_measurement(&mut self, m: Measurement) {
        self.log_loss += m.log_loss();
        self.freq += m.classification_success() as u64;
        self.length += 1;
    }

    pub fn add_measurable(&mut self, event: &Box<dyn Measureable>) {
        self.add_measurement(event.into_measurement());
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl Display for LogLossTracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "log_loss: {}\n freq: {}\n len: {}",
            self.log_loss(),
            self.freq(),
            self.length
        )
    }
}

impl Serialize for LogLossTracker {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(3))?;

        map.serialize_entry("log_loss", &self.log_loss())?;
        map.serialize_entry("freq", &self.freq())?;
        map.serialize_entry("len", &self.length)?;

        map.end()
    }
}

#[cfg(test)]
mod serialize_log_loss {
    use super::*;

    #[test]
    fn test_serialize_log_loss() {
        let mut tracker = LogLossTracker::new();
        tracker.add_measurement(Measurement::new(1, 0.5));
        tracker.add_measurement(Measurement::new(0, 0.5));

        let serialized = serde_json::to_string(&tracker).unwrap();
        let expected = r#"{"log_loss":0.6931471805599453,"freq":0.0,"len":2}"#;
        assert_eq!(serialized, expected);
    }
}
