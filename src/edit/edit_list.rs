use serde::{Deserialize, Serialize};

use crate::edit::edit::Edit;

#[derive(Debug, Clone, Serialize)]
pub struct EditList {
    edits: Vec<Edit>,
}

impl EditList {
    pub fn new(edits: Vec<Edit>) -> Self {
        Self { edits }
    }

    pub fn edits(&self) -> &Vec<Edit> {
        &self.edits
    }

    pub fn into_edits(self) -> Vec<Edit> {
        self.edits
    }
}

impl<'de> Deserialize<'de> for EditList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let edits = Vec::<Edit>::deserialize(deserializer)?;
        Ok(Self { edits })
    }
}

impl Default for EditList {
    fn default() -> Self {
        Self { edits: Vec::new() }
    }
}
