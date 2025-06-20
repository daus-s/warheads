use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Serialize)]
pub struct Points(pub u8);

impl Display for Points {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
