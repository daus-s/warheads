use crate::format::path_manager::nba_storage_path;
use crate::stats::id::Identifiable;
use crate::stats::team_box_score::TeamBoxScore;

pub fn save_nba_game(roster: TeamBoxScore) -> Result<(), String> {
    let identity = roster.identity();

    let filepath = nba_storage_path(identity);

    Ok(())
}

