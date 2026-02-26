use thiserror::Error;

use crate::format::parse;

#[derive(Error, Debug)]
pub enum ReadProcessError {
    #[error("❌ {0}\n❌ IO Error")] //this style error message, cascading
    IOError(std::io::Error),
    #[error("❌ {0}\n❌ JSON Parse Error")]
    JSONParseError(serde_json::Error),
    #[error("❌ {0}\n❌ Object Structure Error")]
    ObjectStructureError(parse::ParseError),
    #[error("❌ could not write edits to edit.json")]
    SerializeEditError,
    #[error("❌ could not create a new edit object")]
    BuildEditError,
}
