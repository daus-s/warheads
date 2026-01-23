use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, SchemaRead, SchemaWrite)]
pub struct PlusMinus(pub Option<i16>);

impl Display for PlusMinus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}
