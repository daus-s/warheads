use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use wincode::{SchemaRead, SchemaWrite};

use std::fmt::{Debug, Display, Formatter};

use crate::stats::season_period::SeasonPeriod;

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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, SchemaRead, SchemaWrite)]
pub struct GameId(pub u64);

impl GameId {
    fn basis(&self) -> (i32, SeasonPeriod, u32) {
        //ex:0022501229

        let period = SeasonPeriod::from_offset(((self.0 / 10_000_000) * 10_000) as i32);
        let truncated = self.0 % 10_000_000;
        let mut year = (truncated / 100_000) as i64;

        if year >= 46 {
            year += 1900;
        } else {
            year += 2000;
        }

        let game_number = (truncated % 10000) as u32;
        (year as i32, period, game_number)
    }
}

impl Display for GameId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0>10}", self.0)
    }
}

impl From<u64> for GameId {
    fn from(value: u64) -> Self {
        assert!(value < 9999999999, "GameId format is 10 characters");

        GameId(value)
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

impl Ord for GameId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for GameId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.basis().partial_cmp(&other.basis())
    }
}

#[test]
fn test_basis() {
    assert_eq!(
        GameId(0022501229).basis(),
        (2025, SeasonPeriod::RegularSeason, 1229)
    );

    assert_eq!(
        GameId(0045900321).basis(),
        (1959, SeasonPeriod::PostSeason, 321)
    );
}
