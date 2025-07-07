use crate::stats::shooting::{Attempts, Makes};
use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreeThrowAttempts(pub Option<u8>);

impl Display for FreeThrowAttempts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Attempts for FreeThrowAttempts {
    fn attempts(&self) -> Option<u8> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FreeThrowMakes(pub u8);

impl Display for FreeThrowMakes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Makes for FreeThrowMakes {
    fn makes(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FreeThrowPercentage(pub Option<f32>);

impl Display for FreeThrowPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(float) => write!(f, "{:.5}", float),
            None => write!(f, "null"),
        }
    }
}

impl Serialize for FreeThrowPercentage {
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
