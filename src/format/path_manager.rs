use crate::constants::paths::data;
use crate::format::season::season_path;
use crate::format::stat_path_formatter::StatPathFormatter as SPF;
use crate::ml::model::Model;
use crate::stats::identity::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::types::{GameDate, GameId, PlayerId, SeasonId, TeamId};
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

/// `nba_source_path` returns the PathBuf to the location of the raw nba data for its relevant domain.
///
/// **returns**
///
/// `data/nba/source/{team or players}/{year}/{period}_{team or player}.json`
pub fn nba_source_path(season: SeasonId, kind: NBAStatKind) -> PathBuf {
    PathBuf::from(format!(
        "{}/nba/source/{}/{}/{}_{}",
        *DATA,
        kind.path_specifier(),
        season_path(season),
        season.period().path_specifier(),
        kind.ext()
    ))
}

pub fn universal_nba_source_path(season: SeasonId, kind: NBAStatKind) -> PathBuf {
    PathBuf::from(format!(
        "nba/source/{}/{}/{}_{}",
        kind.path_specifier(),
        season_path(season),
        season.period().path_specifier(),
        kind.ext()
    ))
}

pub fn nba_correction_dir(season: SeasonId, kind: NBAStatKind) -> String {
    format!(
        "{}/nba/corrections/{}/{}/{}",
        *DATA,
        kind.path_specifier(),
        season_path(season),
        season.period().path_specifier(),
    )
}

pub fn nba_player_correction_file(
    season: SeasonId,
    game_id: GameId,
    player_id: PlayerId,
) -> PathBuf {
    PathBuf::from(format!(
        "{}/{}_{}.corr",
        nba_correction_dir(season, Player),
        game_id,
        player_id
    ))
}

pub fn nba_team_correction_file(season: SeasonId, game_id: GameId, team_id: TeamId) -> PathBuf {
    PathBuf::from(format!(
        "{}/{}_{}.corr",
        nba_correction_dir(season, Team),
        game_id,
        team_id
    ))
}

pub fn correction_path_from_identity(identity: &Identity) -> PathBuf {
    match identity.player_id {
        Some(player_id) => {
            nba_player_correction_file(identity.season_id, identity.game_id, player_id)
        }
        None => nba_team_correction_file(identity.season_id, identity.game_id, identity.team_id),
    }
}

/// `nba_storage_path` returns the PathBuf to the location of the processed nba data for storage on
/// disk.
pub fn nba_storage_path(season_id: SeasonId) -> PathBuf {
    let (_year, period) = season_id.destructure();

    let s = format!("{}/nba/store/{}/{}/", *DATA, season_path(season_id), period);

    PathBuf::from(s)
}

pub fn nba_storage_file(season_id: SeasonId, game_id: GameId) -> PathBuf {
    let s = format!("{}", game_id); // 24600001 -> 0024600001

    let mut path = nba_storage_path(season_id);

    path.push(s);

    path
}

pub fn nba_checksum_file() -> PathBuf {
    PathBuf::from(format!("{}/nba/checksum/checksums.json", *DATA))
}

////////////////////////////////////////////////////////////////////////////////////////////
//// Model Paths ///////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

pub fn records_path<M: Model>(model: &M) -> PathBuf {
    static DATA: Lazy<String> = Lazy::new(data);

    PathBuf::from(format!(
        "{}/nba/{}/records/records.csv",
        *DATA,
        model.model_name()
    ))
}

/// results_path generates the path to where the model accuracy is stored.
pub fn results_path<M: Model>(model: &M) -> PathBuf {
    static DATA: Lazy<String> = Lazy::new(data);

    PathBuf::from(format!(
        "{}/nba/{}/results/results",
        *DATA,
        model.model_name()
    ))
}

pub fn nba_prediction_file<M: Model>(model: &M, date: GameDate) -> PathBuf {
    let d = date.to_filename();

    let mut path = PathBuf::from(format!("{}/nba/{}/predictions", *DATA, model.model_name()));

    path.push(d);

    path
}
