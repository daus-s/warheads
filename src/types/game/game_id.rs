use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter};

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
