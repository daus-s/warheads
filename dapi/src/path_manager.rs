use once_cell::sync::Lazy;
use stats::format::season_file;
use stats::kind::NBAStatKind;
use crate::format::{epath, ext};
use crate::prefix::prefix;

static PREFIX: Lazy<String> = Lazy::new(prefix);
pub fn filepath(szn: i32, stat: NBAStatKind) -> String {
    format!("{}/nba/{}/{}_{}", *PREFIX, epath(stat), season_file(szn), ext(stat))
}