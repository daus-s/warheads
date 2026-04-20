use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::read_checksum::read_checksum;

use crate::dapi::write::write_serializable_with_directory;

use crate::format::path_manager::{nba_source_path, universal_nba_source_path};

use crate::proc::query;

use crate::stats::nba_kind::NBAStatKind;

use crate::types::SeasonId;

/*
    GOOD WILL HUNTING
    this shit is NOT easy for me f*ck
*/

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

/// Compare the checksums of a NBA data source file and if it matches the expected checksum we can bypass refetching from
/// [nba.com/stats](https://www.nba.com/stats). Otherwise we proceed fetching the data and saving the data to our source directory.
pub(crate) async fn compare_and_fetch(
    season_id: SeasonId,
    kind: NBAStatKind,
    checksums: &ChecksumMap,
) {
    let source_path = nba_source_path(season_id, kind);
    let checksum_path = universal_nba_source_path(season_id, kind);

    //why would you expect this if ur looking whether something is initialized correctly??? dummy
    //
    if let Err(_) = read_checksum(&source_path) {
        if let Err(msg) = fetch_and_save_nba_stats(season_id, kind).await {
            println!("{}", msg);
        } else {
            println!("✅ successfully wrote {kind} data to file for the {season_id}");
        }
    } else if let Ok(checksum) = read_checksum(&source_path) {
        let expected_checksum = checksums.get(&checksum_path);

        if expected_checksum.is_none() || checksum != *expected_checksum.unwrap()
        //this might fail on new records
        {
            if let Err(msg) = fetch_and_save_nba_stats(season_id, kind).await {
                println!("{}", msg);
            } else {
                println!("✅ successfully wrote {kind} data to file for the {season_id}");
            }
        } else {
            println!("✅ bypassing fetching {kind} data for the {season_id}, checksums match. ");
        }
    }
}
