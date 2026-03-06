use crate::format::path_manager::{nba_storage_file, nba_storage_path};

use crate::stats::game_obj::GameObject;

use std::fmt::Display;
use std::fs;
use thiserror::Error;

use SaveGameError::{CreateDirectoryError, FileWriteError, WincodeSerializationError};

pub fn save_nba_game(roster: &GameObject) -> Result<(), SaveGameError> {
    let season = roster.season();

    let contents = wincode::serialize(&roster).map_err(|e| WincodeSerializationError(e))?;

    let path = nba_storage_path(season);

    fs::create_dir_all(&path).map_err(|e| CreateDirectoryError(e))?;

    let (season, game) = roster.moment();

    let file = nba_storage_file(season, game);

    fs::write(&file, contents).map_err(|e| FileWriteError(e))?;

    Ok(())
}

#[derive(Debug, Error)]
pub enum SaveGameError {
    WincodeSerializationError(wincode::WriteError),
    CreateDirectoryError(std::io::Error),
    FileWriteError(std::io::Error),
}

impl Display for SaveGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveGameError::WincodeSerializationError(e) => {
                write!(f, "{}\n❌ failed to serialize TeamBoxScore as binary", e)
            }
            SaveGameError::CreateDirectoryError(e) => {
                write!(f, "{}\n❌ failed to create directory", e)
            }
            SaveGameError::FileWriteError(e) => {
                write!(f, "{}\n❌ failed to write file", e)
            }
        }
    }
}
