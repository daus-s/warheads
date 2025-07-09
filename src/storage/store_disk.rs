use crate::format::path_manager::{nba_storage_file, nba_storage_path};
use crate::stats::box_score::BoxScore;
use crate::stats::id::Identifiable;
use crate::dapi::team_box_score::TeamBoxScore;
use std::fs;

pub fn save_nba_game(roster: &TeamBoxScore) -> Result<(), String> {
    let season = roster.season();

    let path = nba_storage_path(&season);

    let contents = serde_json::to_string_pretty(&roster)
        .map_err(|e| format!("❌ failed to convert TeamBoxScore to JSON: {e}"))?;

    fs::create_dir_all(&path)
        .map_err(|e| format!("❌ failed to create the path to the file {:?}: {e}", path))?;

    let identity = roster.identity();

    let file = nba_storage_file(&identity);

    fs::write(&file, contents)
        .map_err(|e| format!("❌ failed to write to the file {:?}: {e}", file))?;

    Ok(())
}
