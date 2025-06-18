use crate::stats::shooting::{Attempts, Makes};
use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};


#[derive(Clone, Debug, Deserialize)]
pub struct ThreePointAttempts(pub Option<u8>);

impl Display for ThreePointAttempts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Serialize for ThreePointAttempts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(u) => serializer.serialize_u8(u),
            None => serializer.serialize_none(),
        }
    }
}

impl Attempts for ThreePointAttempts {
    fn attempts(&self) -> u8 {
        self.0.unwrap_or_default()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThreePointMakes(pub Option<u8>);

impl Display for ThreePointMakes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Makes for ThreePointMakes {
    fn makes(&self) -> u8 {
        self.0.unwrap_or_default()
    }
}

impl Serialize for ThreePointMakes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(u) => serializer.serialize_u8(u),
            None => serializer.serialize_none(),
        }
    }
}


#[derive(Clone, Debug, Deserialize)]
pub struct ThreePointPercentage(pub Option<f32>);

impl Display for ThreePointPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(float) => write!(f, "{:.6}", float),
            None => write!(f, "null")
        }
    }
}

impl Serialize for ThreePointPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(f) => serializer.serialize_f32(f),
            None => serializer.serialize_none(),
        }
    }
}
