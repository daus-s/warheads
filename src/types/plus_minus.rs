use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct PlusMinus(pub Option<i16>);

impl Display for PlusMinus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}
