use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::read_checksum::read_checksum;

use crate::dapi::team_box_score::TeamBoxScore;

use crate::format::path_manager::{nba_source_path, universal_nba_source_path};

use crate::proc::gather;
use crate::proc::gather::{player_games, team_games};

use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;

use crate::types::SeasonId;

/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

pub fn load_nba_season_from_source(year: i32) -> Vec<(Identity, TeamBoxScore)> {
    let mut team_games_vec = Vec::new();

    for period in minimum_spanning_era(year) {
        let player_games_of_period = player_games(period).unwrap_or_else(|e| {
            panic!(
                "{e}\n\
                💀 failed to load and parse player games as JSON.\n\
                run `cargo test checksum::assert_checksums`"
            );
        });

        let team_games_of_period = team_games(period, player_games_of_period).unwrap_or_else(|e| {
            panic!(
                "{e}\n\
                    💀 failed to load and parse team games as JSON.\n\
                    run `cargo test checksum::assert_checksums`"
            );
        });

        team_games_vec.extend(team_games_of_period);
    }

    team_games_vec
}

/// Compare the checksums of a NBA data source file and if it matches the expected checksum we can bypass refetching from
/// [nba.com/stats](https://www.nba.com/stats). Otherwise we proceed fetching the data and saving the data to our source directory.
pub(crate) async fn compare_and_fetch(
    season_id: SeasonId,
    kind: NBAStatKind,
    checksums: &ChecksumMap,
) -> u8 {
    let source_path = nba_source_path(season_id, kind);
    let checksum_path = universal_nba_source_path(season_id, kind);

    let checksum = read_checksum(&source_path).expect(
        "💀 failed to read source data file. check that the program was initialized correctly",
    );
    let expected_checksum = checksums.get(&checksum_path);

    if !source_path.exists()
        || expected_checksum.is_none()
        || checksum != *expected_checksum.unwrap()
    //this might fail on new records
    {
        if let Err(msg) = gather::fetch_and_save_nba_stats(season_id, kind).await {
            println!("{}", msg);
            return 1;
        } else {
            println!("✅ successfully wrote {kind} data to file for the {season_id}");
            return 1;
        }
    } else {
        println!("✅ bypassing fetching {kind} data for the {season_id}, checksums match. ");
        return 0;
    }
}
