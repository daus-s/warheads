use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Assists(pub Option<u8>);

impl Display for Assists {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

impl From<u8> for Assists {
    fn from(value: u8) -> Self {
        Assists(Some(value))
    }
}

impl From<Option<u8>> for Assists {
    fn from(value: Option<u8>) -> Self {
        Assists(value)
    }
}
