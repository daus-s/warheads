use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rebounds(pub Option<u8>);

impl Display for Rebounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OffensiveRebounds(pub Option<u8>);

impl Display for OffensiveRebounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DefensiveRebounds(pub Option<u8>);

impl Display for DefensiveRebounds {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}
