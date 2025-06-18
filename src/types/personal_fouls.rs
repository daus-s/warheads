use serde::{Deserialize, Serialize, Serializer};
use std::fmt::{Display, Formatter};


#[derive(Clone, Debug, Deserialize)]
pub struct PersonalFouls(pub u8);

impl Display for PersonalFouls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for PersonalFouls {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.0)
    }
}