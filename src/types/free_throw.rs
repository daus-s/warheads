use crate::stats::shooting::{Attempts, Makes};
use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};
use crate::types::{FieldGoalPercentage, ThreePointPercentage};

#[derive(Clone, Debug, Serialize)]
pub struct FreeThrowAttempts(pub u8);

impl Display for FreeThrowAttempts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Attempts for FreeThrowAttempts {
    fn attempts(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Debug, Serialize)]
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


#[derive(Clone, Debug)]
pub struct FreeThrowPercentage(pub Option<f32>);

impl Display for FreeThrowPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(float) => write!(f, "{:.5}", float),
            None => write!(f, "null")

        }

    }
}

impl Serialize for FreeThrowPercentage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self.0 {
            Some(f) => serializer.serialize_f64(f as f64),
            None => serializer.serialize_none(),
        }
    }
}