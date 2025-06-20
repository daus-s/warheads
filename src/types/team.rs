use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TeamId(pub u64);

#[derive(Debug, PartialEq, Eq)]
pub struct TeamIdError;

impl FromStr for TeamId {
    type Err = TeamIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p = s.parse::<u64>().map_err(|_| TeamIdError);

        match p {
            Ok(i) => Ok(TeamId(i)),
            Err(e) => Err(e),
        }
    }
}

impl From<u64> for TeamId {
    fn from(value: u64) -> Self {
        TeamId(value)
    }
}

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// `TeamAbbreviation` is a String wrapper that enforces the 3-character Limit, as well as the
/// existence & correctness of the abbreviation.
///

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct TeamAbbreviation(pub String);

#[derive(Debug, PartialEq, Eq)]
pub struct TeamAbbreviationError;


impl TeamAbbreviation {

    pub fn emphasize(&self) -> String {
        self.0.to_ascii_uppercase()
    }
    pub fn de_emphasize(&self) -> String {
        self.0.to_ascii_lowercase()
    }
}

impl FromStr for TeamAbbreviation {
    type Err = TeamAbbreviationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 3 {
            return Err(TeamAbbreviationError);
        };

        Ok(TeamAbbreviation(s.to_string()))
    }
}

impl Display for TeamAbbreviationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "❌ failed to parse TeamAbbreviation")
    }
}

impl Display for TeamAbbreviation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}


/// `TeamName` is a validated (todo) String wrapper to represent Team Names and provide the same
///  existence checks as the other types in this module

#[derive(Clone, Debug, Serialize)]
pub struct TeamName(pub String);

impl Display for TeamName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
