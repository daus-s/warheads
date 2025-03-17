use crate::season::season_file;
use crate::stat_path_formatter::StatPathFormatter as SPF;
use constants::{corrections, data};
use once_cell::sync::Lazy;

static DATA: Lazy<String> = Lazy::new(data);
static CORRECTIONS: Lazy<String> = Lazy::new(corrections);

/// **returns**
///
/// `data/nba/{season_file}/{NBAStatType}/{}.json`
pub fn data_path(szn: i32, stat: impl SPF) -> String {
    format!(
        "{}/nba/{}/{}_{}",
        *DATA,
        stat.epath(),
        season_file(szn),
        stat.ext()
    )
}

pub fn correction_path(season: i32, kind: impl SPF) -> String {
    format!("{}/{}/{}/", *CORRECTIONS, kind.epath(), season_file(season))
}

pub fn correction_file(gameid: &str, playerid: u64) -> String {
    format!("{}_{}.corr", gameid, playerid)
}
