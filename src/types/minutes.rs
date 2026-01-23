use serde::{Deserialize, Serialize};

use std::fmt::{Display, Formatter};

use wincode::{SchemaRead, SchemaWrite};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, SchemaWrite, SchemaRead)]
pub struct Minutes(pub u8);

impl Display for Minutes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
