use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::read_checksum::read_checksum;

use crate::dapi::team_box_score::TeamBoxScore;

use crate::edit::edit_list::EditList;
use crate::edit::edit_loader::{load_edit_list, save_edit_list};

use crate::format::path_manager::{nba_source_path, universal_nba_source_path};

use crate::proc::error::ReadProcessError;
use crate::proc::gather;
use crate::proc::gather::{load_player_games_from_source, load_team_games_from_source};

use crate::stats::identity::Identity;
use crate::stats::nba_kind::NBAStatKind;

use crate::types::SeasonId;

/*
    GOOD WILL HUNTING
    this shit is NOT easy for me f*ck
*/

//change to result
pub fn load_season_from_source(
    era: SeasonId,
) -> Result<Vec<(Identity, TeamBoxScore)>, ReadProcessError> {
    let mut edit_list: EditList = load_edit_list().unwrap_or_default();

    let mut team_games_vec = Vec::new();

    let player_games_of_period = load_player_games_from_source(era, &mut edit_list)?;

    let team_games_of_period =
        load_team_games_from_source(era, player_games_of_period, &mut edit_list)?;

    save_edit_list(&edit_list).map_err(|_| ReadProcessError::SerializeEditError)?;

    team_games_vec.extend(team_games_of_period);

    Ok(team_games_vec)
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
        if let Err(msg) = gather::fetch_and_save_nba_stats(season_id, kind).await {
            println!("{}", msg);
        } else {
            println!("✅ successfully wrote {kind} data to file for the {season_id}");
        }
    } else if let Ok(checksum) = read_checksum(&source_path) {
        let expected_checksum = checksums.get(&checksum_path);

        if expected_checksum.is_none() || checksum != *expected_checksum.unwrap()
        //this might fail on new records
        {
            if let Err(msg) = gather::fetch_and_save_nba_stats(season_id, kind).await {
                println!("{}", msg);
            } else {
                println!("✅ successfully wrote {kind} data to file for the {season_id}");
            }
        } else {
            println!("✅ bypassing fetching {kind} data for the {season_id}, checksums match. ");
        }
    }
}
