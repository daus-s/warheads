use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct Steals(pub Option<u8>);

impl Display for Steals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Serialize for Steals {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(u) => serializer.serialize_u8(u),
            None => serializer.serialize_none(),
        }
    }
}
