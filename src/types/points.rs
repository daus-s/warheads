use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, SchemaWrite, SchemaRead)]
pub struct Points(pub u8);

impl Display for Points {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
