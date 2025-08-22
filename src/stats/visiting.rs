use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Visiting {
    Home,
    Away,
}
