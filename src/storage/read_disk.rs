use crate::format::path_manager::nba_storage_path;

use crate::stats::game_obj::GameObject;

use crate::types::SeasonId;

use NBAReadError::*;

use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

use thiserror::Error;

// todo: if data is loaded in the file system load from the json files rather than the source data (ugly).
pub fn read_nba_season(season_id: SeasonId) -> Result<Vec<GameObject>, NBAReadError> {
    let dir = nba_storage_path(season_id);

    let games = read_directory(&dir)?;

    Ok(games)
}

fn read_directory(path: &PathBuf) -> Result<Vec<GameObject>, NBAReadError> {
    let files = fs::read_dir(path).map_err(|e| DirectoryError(e, path.clone()))?;

    let mut games = Vec::new();

    for file in files {
        match file {
            Ok(entry) => {
                let s =
                    fs::read_to_string(entry.path()).map_err(|e| FileReadError(e, path.clone()))?;

                let game = serde_json::from_str::<GameObject>(&s)
                    .map_err(|e| JSONParseError(e, path.clone()))?;

                games.push(game);
            }
            Err(e) => return Err(FileEntryError(e, path.clone())),
        }
    }

    Ok(games)
}

#[derive(Error, Debug)]
pub enum NBAReadError {
    DirectoryError(std::io::Error, PathBuf),
    FileReadError(std::io::Error, PathBuf),
    JSONParseError(serde_json::Error, PathBuf),
    FileEntryError(std::io::Error, PathBuf),
}

impl Display for NBAReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NBAReadError::DirectoryError(e, path) => {
                write!(
                    f,
                    "❗ {e}:\n{}\n❗ failed to read directory",
                    path.display()
                )
            }
            NBAReadError::FileReadError(e, path) => {
                write!(f, "❗  {e}:\n{}\n❗  failed to read file", path.display())
            }
            NBAReadError::JSONParseError(e, path) => {
                write!(f, "❗  {e}:\n{}\n❗  failed to parse json", path.display())
            }
            NBAReadError::FileEntryError(e, path) => {
                write!(f, "❗  {e}:\n{}\n❗  failed to get entry", path.display())
            }
        }
    }
}
