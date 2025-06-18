use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use crate::stats::shooting::{Attempts, Makes};

#[derive(Clone, Debug, Deserialize)]
pub struct FieldGoalAttempts(pub u8);

impl Attempts for FieldGoalAttempts {
    fn attempts(&self) -> u8 {
        self.0
    }
}

impl Display for FieldGoalAttempts {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for FieldGoalAttempts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FieldGoalMakes(pub u8);

impl Makes for FieldGoalMakes {
    fn makes(&self) -> u8 {
        self.0
    }
}

impl Display for FieldGoalMakes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for FieldGoalMakes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}

/// `FieldGoalPercentage` is a wrapper of the `Option<f32>` struct. This allows for null values to
/// represent making 0/0 field goals. field goals have always been recorded so the
/// previous 2 fields are non-optional.s
#[derive(Clone, Debug, Deserialize)]
pub struct FieldGoalPercentage(pub Option<f32>);

impl Display for FieldGoalPercentage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(float) => write!(f, "{:.6}", float),
            None => write!(f, "null")
        }
    }
}

impl Serialize for FieldGoalPercentage {
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