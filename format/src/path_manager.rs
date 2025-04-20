use crate::season::season_file;
use crate::stat_path_formatter::StatPathFormatter as SPF;
use constants::{corrections, data};
use once_cell::sync::Lazy;
use std::path::PathBuf;

static DATA: Lazy<String> = Lazy::new(data);
static CORRECTIONS: Lazy<String> = Lazy::new(corrections);

/// **returns**
///
/// `data/nba/{season_file}/{NBAStatType}/{Era}/{}.json`
pub fn data_path(szn: i32, stat: impl SPF, period: impl SPF) -> PathBuf {
    PathBuf::from(format!(
        "{}/nba/{}/{}/{}_{}",
        *DATA,
        stat.path_specifier(),
        period.path_specifier(),
        season_file(szn),
        stat.ext() //pg or tg
    ))
}

pub fn correction_path(season: i32, kind: impl SPF) -> String {
    format!(
        "{}/{}/{}/",
        *CORRECTIONS,
        kind.path_specifier(),
        season_file(season)
    )
}

pub fn correction_file(game_id: &str, player_id: u64) -> String {
    format!("{}_{}.corr", game_id, player_id)
}
