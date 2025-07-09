use crate::format::path_manager::nba_storage_path;
use crate::format::season::season_fmt;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::types::SeasonId;
use std::fs;

pub fn read_season(season_id: SeasonId) -> Result<Vec<TeamBoxScore>, String> {
    let dir = nba_storage_path(&season_id);

    let files = fs::read_dir(dir).map_err(|e| {
        format!(
            "❌ failed to read {} season directory: {e}",
            season_fmt(season_id.year())
        )
    })?;

    let mut games = Vec::new();

    for file in files {
        match file {
            Ok(entry) => {
                let s = fs::read_to_string(entry.path())
                    .map_err(|e| format!("❌ failed to read file {:?}: {e}", entry.file_name()))?;

                let game = serde_json::from_str::<TeamBoxScore>(&s).map_err(|e| {
                    format!("❌ couldn't parse json for {:?}: {e}", entry.file_name())
                })?;

                games.push(game);
            }
            Err(e) => {
                return Err(format!("❌ failed to get an entry from file: {e}"))
            }
        }
    }

    Ok(games)
}
