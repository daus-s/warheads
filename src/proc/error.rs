use std::{fmt::Display, io};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadProcessError {
    IOError(io::Error),
    JSONParseError(serde_json::Error),
}

impl Display for ReadProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadProcessError::IOError(err) => write!(f, "❌ IO Error: {}", err),
            ReadProcessError::JSONParseError(err) => write!(f, "❌ JSON Parse Error: {}", err),
        }
    }
}
