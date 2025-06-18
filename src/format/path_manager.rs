use crate::constants::paths::data;
use crate::format::season::season_directory;
use crate::format::stat_path_formatter::StatPathFormatter as SPF;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_type::SeasonPeriod;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use crate::types::{GameId, PlayerId, SeasonId};

/// this is the environment variable for the data path as per system
///
/// an example is:
///
///     /Users/daus/Documents/warheads/data
static DATA: Lazy<String> = Lazy::new(data);

/// **returns**
///
/// `data/nba/{season_file}/{NBAStatType}/{Era}/{}.json`
pub fn nba_data_path(season: SeasonId, kind: NBAStatKind) -> PathBuf {
    let (year, period) = season.destructure();

    PathBuf::from(format!(
        "{}/nba/data/{}/{}/{}_{}",
        *DATA,
        kind.path_specifier(),
        period.path_specifier(),
        season_directory(season),
        kind.ext()
    ))
}

pub fn nba_correction_dir(season: SeasonId, kind: NBAStatKind) -> String {
    format!(
        "{}/nba/corrections/{}/{}/{}",
        *DATA,
        kind.path_specifier(),
        season.period().path_specifier(),
        season_directory(season),
    )
}

pub fn nba_correction_file(
    season: SeasonId,
    kind: NBAStatKind,
    game_id: GameId,
    player_id: PlayerId,
) -> String {
    format!(
        "{}/{}_{}.corr",
        nba_correction_dir(season, kind),
        game_id,
        player_id
    )
}
