use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::write_checksum::checksum_pair;
use crate::dapi::hunting::BEGINNING;
use crate::dapi::parse::{destructure_dt, DT};
use crate::format::path_manager::nba_data_path;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;
use chrono::Local;

pub fn generate_checksums() -> ChecksumMap {
    let DT { year, month, day } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = BEGINNING; //first year of the nba in record is 1946-1947 szn

    let mut checksums: ChecksumMap = ChecksumMap::new();

    for szn in begin..year + seasonal_depression {
        for era in minimum_spanning_era(szn) {
            //team
            let team_path = nba_data_path(era, NBAStatKind::Team);

            let team_checksum = checksum_pair(&team_path);

            if let Ok(checksum) = team_checksum {
                checksums.insert(team_path, checksum);
            } else {
                eprintln!("❌ tried to verify team checksum for the {szn} {era} but couldn't read data file.")
            }

            // player
            let player_path = nba_data_path(era, NBAStatKind::Player);

            let player_checksum = checksum_pair(&player_path);

            if let Ok(checksum) = player_checksum {
                checksums.insert(player_path, checksum);
            } else {
                eprintln!("❌ tried to verify player checksum for the {szn} {era} but couldn't read data file.")
            }
        }
    }
    checksums
}
