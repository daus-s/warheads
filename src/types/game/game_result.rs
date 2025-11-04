use crate::stats::serde_enum::SerdeEnum;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// `GameResult` is an enum that represents the result of a game a Win, Loss or a Draw (NFL only.)
/// implements SerdeEnum as well as functions for `to_str` and `from_str`
///
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Loss,
    Draw,
}

impl FromStr for GameResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "W" | "w" => Ok(GameResult::Win),
            "L" | "l" => Ok(GameResult::Loss),
            "D" | "d" | "T" | "t" => Ok(GameResult::Draw),
            _ => Err(format!(
                "❌ could not parse a game result from the string: {s}"
            )),
        }
    }
}

impl SerdeEnum for GameResult {
    type Item = &'static str;

    fn items() -> Vec<Self::Item> {
        vec!["W", "L", "D"]
    }

    fn values() -> Vec<Value> {
        vec![json!("W"), json!("L"), json!("D")]
    }

    fn evaluate(&self) -> Value {
        match self {
            GameResult::Win => json!("W"),
            GameResult::Loss => json!("L"),
            GameResult::Draw => json!("D"),
        }
    }
}

impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::Win => {
                write!(f, "W")
            }
            GameResult::Loss => {
                write!(f, "L")
            }
            GameResult::Draw => {
                write!(f, "D")
            }
        }
    }
}

impl Serialize for GameResult {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for GameResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        s.parse::<GameResult>().map_err(de::Error::custom)
    }
}
