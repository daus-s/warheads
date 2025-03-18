use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, Value};
use serde_json::Value::Number;
use crate::se::SerdeEnum;

/// BoolInt is an int that is either 1 or 0
/// to represent a boolean value

pub struct BoolInt (u8);

impl BoolInt {
    // Constructor that validates the input
    pub fn new(value: u8) -> Self {
        if value != 0 && value != 1 {
            panic!("BoolInt can only be 0 or 1");
        }
        BoolInt(value)
    }
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl SerdeEnum for BoolInt {
    type Item = u64;

    fn enumerate() -> Vec<Self::Item> {
        vec![0, 1]
    }

    fn evaluate() -> Vec<Value> {
        vec![json!(0), json!(1)]
    }
}

/// MatchupString is a String wrapper that is
/// enforced by its `fn parse() -> Self`
/// when loaded from String (or str)
#[derive(Clone, Debug)]
pub struct MatchupString (pub String);

impl MatchupString {
    // todo: implement the functions from player and team box score
    // todo: for matchup string (from s3)

    pub fn str(&self) -> String {
        self.0.to_string()
    }
}

impl Serialize for MatchupString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for MatchupString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;

        MatchupString::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for MatchupString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [_home, "vs.", _away] |
            [_away,  "@" , _home]
            => Ok(MatchupString(s.to_string())),
            _ => Err(format!("{} is not  a valid matchup string.\nexpected either:\n  • [tm1, @, tm2]\n  • [tm1, vs, tm2]", s )),
        }
    }
}


///
/// GameResult is an enum that implements SerdeEnum
/// as well as functions for `to_str` and `from_str`
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
            "W" => Ok(GameResult::Win),
            "L" => Ok(GameResult::Loss),
            "D" => Ok(GameResult::Draw),
            _ => Err(
                format!("could not parse a game result from string: {}", s)
            )
        }
    }
}

impl SerdeEnum for GameResult {
    type Item = &'static str;

    fn enumerate() -> Vec<Self::Item> {
        vec!["W", "L", "D"]
    }

    fn evaluate() -> Vec<Value> {
        vec![json!("W"), json!("L"), json!("D")]

    }
}

impl GameResult {
    pub fn to_str(&self) -> &'static str {
        match self {
            GameResult::Win => "W",
            GameResult::Loss => "L",
            GameResult::Draw => "D",
        }
    }
}

