use crate::stats::statify::SafetyValve;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};


#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Assists(pub Option<u8>);

impl Display for Assists {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.unwrap_fmt("null"))
    }
}

// impl Serialize for Assists {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self.0 {
//             Some(u) => serializer.serialize_u8(u),
//             None => serializer.serialize_none(),
//         }
//     }
// }

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

