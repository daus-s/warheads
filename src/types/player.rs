use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use wincode::{SchemaRead, SchemaWrite};

use crate::stats::key::Key;

/// `PlayerName` is a  StringWrapper for player names. Included only as a String Wrapper,
/// no added functionality is provided.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, SchemaWrite, SchemaRead)]
pub struct PlayerName(pub String);

impl Display for PlayerName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// `PlayerId` is represented by an unsigned integer in the NBA dataset. It is represented as a
/// *required* field in all PlayerBoxScores.
#[derive(
    Clone,
    Copy,
    Debug,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    SchemaWrite,
    SchemaRead,
)]
pub struct PlayerId(pub u64);

impl Display for PlayerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Key for PlayerId {}
