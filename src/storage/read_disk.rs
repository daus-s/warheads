use crate::format::path_manager::nba_storage_path;
use crate::stats::game_obj::GameObject;
use crate::types::SeasonId;
use std::fs;
use std::path::PathBuf;

// todo: if data is loaded in the file system load from the json files rather than the source data (ugly).
pub fn read_nba_season(season_id: SeasonId) -> Result<Vec<GameObject>, String> {
    let dir = nba_storage_path(season_id);

    let games = read_directory(&dir)?;

    Ok(games)
}

fn read_directory(path: &PathBuf) -> Result<Vec<GameObject>, String> {
    let files = fs::read_dir(path).map_err(|e| format!("❌ failed to read directory: {e}"))?;

    let mut games = Vec::new();

    for file in files {
        match file {
            Ok(entry) => {
                let s = fs::read_to_string(entry.path())
                    .map_err(|e| format!("❌ failed to read file {:?}: {e}", entry.file_name()))?;

                let game = serde_json::from_str::<GameObject>(&s).map_err(|e| {
                    format!("❌ couldn't parse json for {:?}: {e}", entry.file_name())
                })?;

                games.push(game);
            }
            Err(e) => return Err(format!("❌ failed to get an entry from file: {e}")),
        }
    }

    Ok(games)
}
