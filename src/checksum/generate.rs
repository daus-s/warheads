use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::read_checksum::read_checksum;

use crate::dapi::season_manager::nba_lifespan;
use crate::format::path_manager::{nba_source_path, universal_nba_source_path};

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;

pub fn generate_checksums() -> ChecksumMap {
    let mut checksums: ChecksumMap = ChecksumMap::new();

    for szn in nba_lifespan() {
        for era in minimum_spanning_era(szn) {
            //team
            let team_path = nba_source_path(era, NBAStatKind::Team);

            let team_display_path = universal_nba_source_path(era, NBAStatKind::Team);

            let team_checksum = read_checksum(&team_path);

            if let Ok(checksum) = team_checksum {
                checksums.insert(team_display_path, checksum);
            } else {
                eprintln!("❌ tried to verify team checksum for the {szn} {era} but couldn't read data file.")
            }

            // player
            let player_path = nba_source_path(era, NBAStatKind::Player);

            let player_display_path = universal_nba_source_path(era, NBAStatKind::Player);

            let player_checksum = read_checksum(&player_path);

            if let Ok(checksum) = player_checksum {
                checksums.insert(player_display_path, checksum);
            } else {
                eprintln!("❌ tried to verify player checksum for the {szn} {era} but couldn't read data file.")
            }
        }
    }
    checksums
}
