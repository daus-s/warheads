use crate::stats::se::SerdeEnum;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// `GameDate`is a `chrono::NaiveDate` wrapper that implements the necessary traits to work
/// interchangeably in the code base.
#[derive(Clone, Debug, Deserialize)]
pub struct GameDate(pub NaiveDate);

impl Display for GameDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for GameDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}


/// `GameId` is a number represented by a JSON String. It will sometimes be parsed and interpreted
/// as a numeric value.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct GameId(pub String);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for GameId {
    fn from(value: String) -> Self {
        GameId(value)
    }
}

impl Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

/// `GameResult` is an enum that represents the result of a game a Win, Loss or a Draw (NFL only.)
/// implements SerdeEnum as well as functions for `to_str` and `from_str`
///
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
            _ => Err(format!("❌ could not parse a game result from the string: {s}")),
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let str = match self {
            GameResult::Win => "W",
            GameResult::Loss => "L",
            GameResult::Draw => "D",
        };

        write!(f, "{}", str)
    }
}

