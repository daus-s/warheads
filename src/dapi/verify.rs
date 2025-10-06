use std::collections::HashMap;

use chrono::Local;

use crate::checksum::write_checksum::checksum_pair;
use crate::dapi::hunting::BEGINNING;
use crate::dapi::parse::{destructure_dt, DT};
use crate::stats::domain::Domain;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;

pub fn verify_nba() {
    let DT { year, month, day } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = BEGINNING; //first year of the nba in record is 1946-1947 szn

    let mut checksums: HashMap<Domain, u32> = HashMap::new();

    for szn in begin..year + seasonal_depression {
        for era in minimum_spanning_era(szn) {
            //team
            let team_domain: Domain = (era, NBAStatKind::Team);

            let team_checksum = checksum_pair(team_domain);

            if let Ok(checksum) = team_checksum {
                checksums.insert(team_domain, checksum);
            } else {
                eprintln!("❌ tried to verify team checksum for era {era} and season {szn} but couldn't read data file.")
            }

            // player
            let player_domain: Domain = (era, NBAStatKind::Player);

            let player_checksum = checksum_pair(player_domain);

            if let Ok(checksum) = player_checksum {
                checksums.insert(player_domain, checksum);
            } else {
                eprintln!("❌ tried to verify player checksum for era {era} and season {szn} but couldn't read data file.")
            }
        }
    }
}
