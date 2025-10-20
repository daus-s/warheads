use EloWriterError::*;

use std::fmt::Display;
use std::fs::{self, File};
use std::path::Path;

use csv::Writer;

use crate::ml::elo::Elo;

pub struct EloWriter {
    file: Writer<File>,
}

impl EloWriter {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, EloWriterError> {
        let filename = path.as_ref();

        if !filename.is_file() {
            fs::create_dir_all(filename.parent().unwrap_or(Path::new("/")))
                .map_err(|e| DirectoryCreationError(e))?;
        }

        let writer = Writer::from_path(path).map_err(|e| WriterError(e))?;

        Ok(EloWriter { file: writer })
    }

    pub fn serialize_elo(&mut self, elo: &Elo) -> Result<(), EloWriterError> {
        //todo: implement this to have each row have a same

        self.file.serialize(elo).map_err(|e| SerializerError(e))?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum EloWriterError {
    DirectoryCreationError(std::io::Error),
    WriterError(csv::Error),
    SerializerError(csv::Error),
}

impl Display for EloWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EloWriterError::DirectoryCreationError(error) => {
                write!(f, "{error}\n❌ failed to create directory for EloWriter.")
            }
            EloWriterError::WriterError(error) => {
                write!(
                    f,
                    "{error}\n❌ failed to open a csv::Writer for EloWriter. "
                )
            }
            EloWriterError::SerializerError(error) => {
                write!(f, "{error}\n❌ failed to serialize data for EloWriter. ")
            }
        }
    }
}
