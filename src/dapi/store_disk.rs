use crate::format::path_manager::nba_storage_path;

use crate::stats::game_obj::GameObject;

use crate::types::SeasonId;

use std::fmt::Display;
use std::fs;
use thiserror::Error;

pub fn save_nba_games(season_id: SeasonId, games: &[GameObject]) -> Result<(), SaveGameError> {
    use SaveGameError::{CreateDirectoryError, FileWriteError, WincodeSerializationError};

    let contents = wincode::serialize(&games).map_err(|e| WincodeSerializationError(e))?;

    let path = nba_storage_path(season_id);

    fs::create_dir_all(&path.parent().unwrap()).map_err(|e| CreateDirectoryError(e))?;

    let path = nba_storage_path(season_id);

    fs::write(&path, contents).map_err(|e| FileWriteError(e))?;

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
                write!(f, "❌ {}\n❌ failed to serialize TeamBoxScore as binary", e)
            }
            SaveGameError::CreateDirectoryError(e) => {
                write!(f, "❌ {}\n❌ failed to create directory", e)
            }
            SaveGameError::FileWriteError(e) => {
                write!(f, "❌ {}\n❌ failed to write file", e)
            }
        }
    }
}
