use crate::stats::statify::SafetyValve;
use serde::Serialize;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct Blocks(pub Option<u8>);

impl Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}
