use crate::stats::serde_enum::SerdeEnum;
use chrono::{Datelike, NaiveDate};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// `GameDate`is a `chrono::NaiveDate` wrapper that implements the necessary traits to work
/// interchangeably in the code base.
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct GameDate(pub NaiveDate);

impl GameDate {
    pub(crate) fn today() -> Self {
        let naive_date = chrono::Utc::now().date_naive();

        GameDate(naive_date)
    }

    /// destructure GameDate object into (y, m, d) tuple. year is an integer,
    /// years before the big JC was born are negative
    ///
    /// returns month and day as unsigned. both are 1-indexed. (month [1..12]) (day [1..31])
    pub fn destructure(&self) -> (i32, u32, u32) {
        let (year, month, day) = (
            self.0.year_ce().1 as i32 * (if self.0.year_ce().0 { 1i32 } else { -1i32 }),
            self.0.month0() + 1,
            self.0.day0() + 1,
        );

        (year, month, day)
    }
}

impl From<NaiveDate> for GameDate {
    fn from(date: NaiveDate) -> Self {
        GameDate(date)
    }
}

impl FromStr for GameDate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").map_err(|e| e.to_string())?;

        Ok(GameDate(date))
    }
}

impl<'de> Deserialize<'de> for GameDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let date = NaiveDate::parse_from_str(&*s, "%Y-%m-%d").map_err(de::Error::custom)?;

        Ok(GameDate(date))
    }
}

impl From<&str> for GameDate {
    fn from(s: &str) -> Self {
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            GameDate(date)
        } else if let Ok(date) = NaiveDate::parse_from_str(s, "%m/%d/%Y") {
            GameDate(date)
        } else if let Ok(date) = NaiveDate::parse_from_str(s, "%Y_%m_%d") {
            GameDate(date)
        } else {
            GameDate(NaiveDate::MIN)
        }
    }
}

impl Debug for GameDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted_date = self.0.format("%Y-%m-%d").to_string();

        write!(f, "{}", formatted_date)
    }
}

impl Display for GameDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let formatted_date = self.0.format("%m/%d/%Y").to_string();

        write!(f, "{}", formatted_date)
    }
}

impl Serialize for GameDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let format = format!(
            "{}-{:02}-{:02}",
            self.0.year(),
            self.0.month(),
            self.0.day()
        );

        serializer.serialize_str(&format)
    }
}

/// ## GameId
///
/// ### Ordering & Properties
/// `GameId` is the unique ordered id for the game. In a given season era
/// (`SeasonId`) the gameids are ordered.
///
/// However, gameids are not ordered across seasons as any game id from 2025
/// or other early 21st century years (see `(2000..=min(present, 2046))`)
/// comes before the earliest nba seasons of the 1940s.
///
/// However, this leaves us with the fact that GameId and SeasonId can form a basis
/// for ordering games across seasons.
///
/// ### Serialization & Deserialization
/// `GameId` is a number represented in the NBA data by a JSON String, but we will use it as an int.
///
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct GameId(pub u64);

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>10}", self.0)
    }
}

impl From<String> for GameId {
    fn from(value: String) -> Self {
        GameId(value.parse().expect("💀 couldn't parse game id"))
    }
}

impl From<&str> for GameId {
    fn from(value: &str) -> Self {
        GameId::from(value.to_string())
    }
}

impl<'de> Deserialize<'de> for GameId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        let game_number = s.parse::<u64>().map_err(de::Error::custom)?;

        Ok(GameId(game_number))
    }
}

impl Serialize for GameId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:0>10}", self.0))
    }
}

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
