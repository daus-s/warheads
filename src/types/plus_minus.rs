use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct PlusMinus(pub Option<i16>);

impl Display for PlusMinus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Serialize for PlusMinus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(i) => serializer.serialize_i16(i),
            None => serializer.serialize_none(),
        }
    }
}
