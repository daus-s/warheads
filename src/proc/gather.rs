use std::fs::File;
use std::io::Read;

use crate::dapi::player_box_score::PlayerBoxScore;
use crate::dapi::team_box_score::TeamBoxScore;

use crate::edit::edit_list::{self, EditList};
use crate::format::parse::parse_season;
use crate::format::path_manager::nba_source_path;

use crate::proc::error::ReadProcessError;
use crate::proc::query;
use crate::proc::rip::process_nba_games;

use crate::stats::identity::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_stat::NBABoxScore::{Player, Team};

use crate::storage::write::write_serializable_with_directory;

use crate::types::SeasonId;

pub fn load_player_games_from_source(
    season_id: SeasonId,
    edit_list: &EditList,
) -> Result<Vec<(Identity, PlayerBoxScore)>, ReadProcessError> {
    let player_source_path = nba_source_path(season_id, NBAStatKind::Player);

    let mut file = File::open(player_source_path).map_err(|e| ReadProcessError::IOError(e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| ReadProcessError::IOError(e))?;

    let json = serde_json::from_str(&contents).map_err(|e| ReadProcessError::JSONParseError(e))?;

    let (rows, headers) =
        parse_season(json).map_err(|e| ReadProcessError::ObjectStructureError(e))?;

    Ok(
        process_nba_games(season_id, NBAStatKind::Player, headers, rows, edit_list)?
            .into_iter()
            .filter_map(|(id, stat)| match stat {
                Player(box_score) => Some((id, box_score)),
                _ => None,
            })
            .collect::<Vec<(Identity, PlayerBoxScore)>>(),
    )
}

pub fn load_team_games_from_source(
    season_id: SeasonId,
    player_games: Vec<(Identity, PlayerBoxScore)>,
    edit_list: &EditList,
) -> Result<Vec<(Identity, TeamBoxScore)>, ReadProcessError> {
    let team_source_path = nba_source_path(season_id, NBAStatKind::Team);

    let mut file = File::open(team_source_path).map_err(|e| ReadProcessError::IOError(e))?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .map_err(|e| ReadProcessError::IOError(e))?;

    let json = serde_json::from_str(&contents).map_err(|e| ReadProcessError::JSONParseError(e))?;

    let (rows, headers) =
        parse_season(json).map_err(|e| ReadProcessError::ObjectStructureError(e))?;

    let mut games: Vec<(Identity, TeamBoxScore)> =
        process_nba_games(season_id, NBAStatKind::Team, headers, rows, edit_list)?
            .into_iter()
            .filter_map(|(id, stat)| match stat {
                Team(t) => Some((id, t)),
                _ => None,
            })
            .collect();

    for (p_id, player_box_score) in player_games.into_iter() {
        for (t_id, team_box_score) in games.iter_mut() {
            if p_id.game() == t_id.game() {
                team_box_score.add_player_stats(player_box_score.clone());
            }
        }
    }

    Ok(games)
}

pub async fn fetch_and_save_nba_stats(season: SeasonId, stat: NBAStatKind) -> Result<(), String> {
    let file_path = nba_source_path(season, stat);

    let (year, _period) = season.destructure();

    match query::nba_history_json(season, stat).await {
        Ok(response_data) => match write_serializable_with_directory(&file_path, &response_data) {
            Ok(_) => {
                println!(
                    "✅ successfully saved nba stats for {} season at file: {:?}",
                    season, &file_path
                );
                Ok(())
            }
            Err(e) => Err(format!(
                "❌ error saving nba stats for {} season at file {:?}: {}",
                season, &file_path, e
            )),
        },
        Err(e) => Err(format!(
            "❌ failed to fetch {} stats for {} season: {:?}",
            year, stat, e
        )),
    }
}
