use std::path::PathBuf;

use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::dapi::write::write_games;

use crate::format::path_manager::nba_data_path;
use crate::format::season::season_fmt;

use crate::proc::error::ReadProcessError;
use crate::proc::hunting;
use crate::proc::rip::read_and_process_nba_games;

use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_stat::NBABoxScore::{Player, Team};

use crate::types::SeasonId;

pub fn player_games(
    season: SeasonId,
    path: &PathBuf,
) -> Result<Vec<(Identity, PlayerBoxScore)>, ReadProcessError> {
    Ok(
        read_and_process_nba_games(season, NBAStatKind::Player, path)
            .map_err(|e| e)?
            .into_iter()
            .filter_map(|(id, stat)| match stat {
                Player(box_score) => Some((id, box_score)),
                _ => None,
            })
            .collect::<Vec<(Identity, PlayerBoxScore)>>(),
    )
}

pub fn team_games(
    season: SeasonId,
    path: &PathBuf,
    roster: Vec<(Identity, PlayerBoxScore)>,
) -> Result<Vec<(Identity, TeamBoxScore)>, ReadProcessError> {
    let mut games: Vec<(Identity, TeamBoxScore)> =
        read_and_process_nba_games(season, NBAStatKind::Team, path)?
            .into_iter()
            .filter_map(|(id, stat)| match stat {
                Team(t) => Some((id, t)),
                _ => None,
            })
            .collect();

    // uh oh
    for (p_id, player_box_score) in roster.into_iter() {
        for (t_id, team_box_score) in games.iter_mut() {
            if p_id.game() == t_id.game() {
                team_box_score.add_player_stats(player_box_score.clone());
            }
        }
    }

    Ok(games)
}

pub async fn fetch_and_save_nba_stats(season: SeasonId, stat: NBAStatKind) -> Result<(), String> {
    let file_path = nba_data_path(season, stat);

    let (year, _period) = season.destructure();

    match hunting::query_nba(season, stat).await {
        Ok(response_data) => match write_games(&file_path, &response_data) {
            Ok(_) => {
                println!(
                    "✅ successfully saved nba stats for {} season at file: {:?}",
                    season_fmt(season.year()),
                    &file_path
                );
                Ok(())
            }
            Err(e) => Err(format!(
                "❌ error saving nba stats for {} season at file {:?}: {}",
                season_fmt(season.year()),
                &file_path,
                e
            )),
        },
        Err(e) => Err(format!(
            "❌ failed to fetch {} stats for {} season: {:?}",
            year, stat, e
        )),
    }
}
