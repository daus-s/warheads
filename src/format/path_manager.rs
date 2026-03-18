use crate::constants::paths::data;

use crate::format::season::season_path;
use crate::format::stat_path_formatter::StatPathFormatter as SPF;

use crate::ml::model::Model;

use crate::stats::nba_kind::NBAStatKind;

use crate::types::{GameDate, SeasonId};

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

pub fn nba_edit_file() -> PathBuf {
    PathBuf::from(format!("{}/nba/edits.json", *DATA))
}

/// `nba_storage_path` returns the PathBuf to the location of the processed nba data for storage on
/// disk.
pub fn nba_storage_path(season_id: SeasonId) -> PathBuf {
    let s = format!(
        "{}/nba/volumes/{}_{}.vol",
        *DATA,
        season_id.year(),
        season_id.period().path_specifier()
    );

    PathBuf::from(s)
}

pub fn nba_checksum_file() -> PathBuf {
    PathBuf::from(format!("{}/nba/checksums.json", *DATA))
}

////////////////////////////////////////////////////////////////////////////////////////////
//// Model Paths ///////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////

pub fn records_path<M: Model>(model: &M) -> PathBuf {
    static DATA: Lazy<String> = Lazy::new(data);

    PathBuf::from(format!("{}/nba/{}/records.csv", *DATA, model.model_name()))
}

/// results_path generates the path to where the model accuracy is stored.
pub fn results_path<M: Model>(model: &M) -> PathBuf {
    static DATA: Lazy<String> = Lazy::new(data);

    PathBuf::from(format!("{}/nba/{}/results.json", *DATA, model.model_name()))
}

pub fn nba_prediction_file<M: Model>(model: &M, date: GameDate) -> PathBuf {
    let d = date.to_filename();

    let mut path = PathBuf::from(format!("{}/nba/{}/predictions", *DATA, model.model_name()));

    path.push(d);

    path
}
