use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// `PlayerName` is a  StringWrapper for player names. Included only as a String Wrapper,
/// no added functionality is provided.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerName(pub String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// `PlayerId` is represented by an unsigned integer in the NBA dataset. It is represented as a
/// *required* field in all PlayerBoxScores.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayerId(pub u64);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
