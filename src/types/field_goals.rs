use crate::stats::shooting::{Attempts, Makes};
use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldGoalAttempts(pub Option<u8>);

impl Attempts for FieldGoalAttempts {
    fn attempts(&self) -> Option<u8> {
        self.0
    }
}

impl Display for FieldGoalAttempts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt(" - "))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FieldGoalMakes(pub u8);

impl Makes for FieldGoalMakes {
    fn makes(&self) -> u8 {
        self.0
    }
}

impl Display for FieldGoalMakes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// `FieldGoalPercentage` is a wrapper of the `Option<f32>` struct. This allows for null values to
/// represent making 0/0 field goals. field goals have always been recorded so the
/// previous 2 fields are non-optional.s
#[derive(Clone, Debug, Deserialize)]
pub struct FieldGoalPercentage(pub Option<f32>);

impl Display for FieldGoalPercentage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(float) => write!(f, "{:.5}", float),
            None => write!(f, "null"),
        }
    }
}

impl Serialize for FieldGoalPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(f) => serializer.serialize_f64(f as f64),
            None => serializer.serialize_none(),
        }
    }
}
