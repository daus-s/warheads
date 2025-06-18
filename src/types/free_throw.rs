use crate::stats::shooting::{Attempts, Makes};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct FreeThrowAttempts(pub u8);

impl Display for FreeThrowAttempts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for FreeThrowAttempts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

impl Attempts for FreeThrowAttempts {
    fn attempts(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FreeThrowMakes(pub u8);

impl Display for FreeThrowMakes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for FreeThrowMakes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
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
            Some(float) => write!(f, "{:.6}", float),
            None => write!(f, "null")

        }

    }
}

impl Serialize for FreeThrowPercentage {
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