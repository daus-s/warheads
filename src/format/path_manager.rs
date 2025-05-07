use crate::format::season::season_directory;
use crate::format::stat_path_formatter::StatPathFormatter as SPF;
use crate::constants::paths::{data};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_type::SeasonPeriod;

/// this is the environment variable for the data path as per system
///
/// an example is:
///
///     /Users/daus/Documents/warheads/data
static DATA: Lazy<String> = Lazy::new(data);

/// **returns**
///
/// `data/nba/{season_file}/{NBAStatType}/{Era}/{}.json`
pub fn nba_data_path(szn: i32, kind: impl SPF, period: impl SPF) -> PathBuf {
    PathBuf::from(format!(
        "{}/nba/data/{}/{}/{}_{}",
        *DATA,
        kind.path_specifier(),
        period.path_specifier(),
        season_directory(szn),
        kind.ext()
    ))
}


pub fn nba_correction_dir(season: i32, kind: NBAStatKind, period: SeasonPeriod) -> String {
    format!(
        "{}/nba/corrections/{}/{}",
        *DATA,
        kind.path_specifier(),
        season_directory(season),
    )
}

pub fn nba_correction_file(season: i32, kind: NBAStatKind, period: SeasonPeriod, game_id: &str, player_id: u64) -> String {
    format!("{}/{}_{}.corr", nba_correction_dir(season, kind, period), game_id, player_id)
}
