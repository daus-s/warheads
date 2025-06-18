use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Deserialize)]
pub struct FantasyPoints(pub Option<f32>);

impl Display for FantasyPoints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl Serialize for FantasyPoints {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            Some(f) => serializer.serialize_f32(f),
            None => serializer.serialize_none(),
        }
    }
}