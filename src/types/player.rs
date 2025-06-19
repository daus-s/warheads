use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};


/// `PlayerName` is a  StringWrapper for player names. Included only as a String Wrapper,
/// no added functionality is provided.
#[derive(Clone, Debug, Deserialize)]
pub struct PlayerName(pub String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for PlayerName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

/// `PlayerId` is represented by an unsigned integer in the NBA dataset. It is represented as a
/// *required* field in all PlayerBoxScores.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct PlayerId(pub u64);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for PlayerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.0)
    }
}
