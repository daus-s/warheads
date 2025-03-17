use std::str::FromStr;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub enum Visiting {
    Home,
    Away,
}

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

impl GameResult {
    pub fn to_str(&self) -> &'static str {
        match self {
            GameResult::Win => "W",
            GameResult::Loss => "L",
            GameResult::Draw => "D",
        }
    }
}

#[derive(Clone, Debug)]
pub struct MatchupString (pub String);

impl MatchupString {
    //todo: implement the functions from player and team box score for matchup string

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
            [home, "vs.", away] |
            [away, "@", home] => Ok(MatchupString(s.to_string())) ,
            _ => Err(format!("{} is not  a valid matchup string.\nexpected either:\n  • [tm1, @, tm2]\n  • [tm1, vs, tm2]", s )),
        }
    }
}