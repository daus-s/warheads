use crate::constants::paths::data;
use crate::format::season::season_path;
use crate::format::stat_path_formatter::StatPathFormatter as SPF;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::types::{GameId, PlayerId, SeasonId, TeamId};
use once_cell::sync::Lazy;
use std::path::PathBuf;

/// this is the environment variable for the data path as per system
///
/// an example is:
///
///     let path = "/Users/daus/Documents/warheads/data";
///
/// is functionally equivalent to accessing *DATA when in scope
///

static DATA: Lazy<String> = Lazy::new(data);

/// `nba_data_path` returns the PathBuf to the raw data location of the nba stats for its relevant
/// domain.
///
/// **returns**
///
/// `data/nba/data/{team or players}/{year}/{period}_{team or player}.json`
pub fn nba_data_path(season: &SeasonId, kind: NBAStatKind) -> PathBuf {
    PathBuf::from(format!(
        "{}/nba/data/{}/{}/{}_{}",
        *DATA,
        kind.path_specifier(),
        season_path(season),
        season.period().path_specifier(),
        kind.ext()
    ))
}

pub fn nba_correction_dir(season: &SeasonId, kind: NBAStatKind) -> String {
    format!(
        "{}/nba/corrections/{}/{}/{}",
        *DATA,
        kind.path_specifier(),
        season_path(season),
        season.period().path_specifier(),
    )
}

pub fn nba_player_correction_file(
    season: &SeasonId,
    game_id: GameId,
    player_id: PlayerId,
) -> String {
    format!(
        "{}/{}_{}.corr",
        nba_correction_dir(season, Player),
        game_id,
        player_id
    )
}

pub fn nba_team_correction_file(season: &SeasonId, game_id: GameId, team_id: TeamId) -> String {
    format!(
        "{}/{}_{}.corr",
        nba_correction_dir(season, Team),
        game_id,
        team_id
    )
}


/// `nba_storage_path` returns the PathBuf to the location of the processed nba data for storage on
/// disk.
pub fn nba_storage_path(id: Identity) -> PathBuf {
    let s = format!("{}/nba/store/{}/{}/{}_{}",*DATA, season_path(&id.season_id), id.season_id.period(), id.game_id, id.team_abbr);

    PathBuf::from(s)
}