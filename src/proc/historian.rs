use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::sign::sign_nba;

use crate::dapi::season_manager::nba_lifespan;

use crate::format::path_manager::nba_checksum_path;

use crate::ml::elo_tracker::EloTracker;

use crate::proc::hunting::compare_and_fetch;
use crate::proc::store::save_nba_season;

use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;

pub async fn observe_nba() {
    let checksums = ChecksumMap::load().expect("💀 failed to load checksums");

    for year in nba_lifespan() {
        for era in minimum_spanning_era(year) {
            compare_and_fetch(era, NBAStatKind::Player, &checksums).await;
            compare_and_fetch(era, NBAStatKind::Team, &checksums).await;
        }
    }

    match sign_nba() {
        Ok(_) => println!(
            "✅ successfully signed nba data with checksums in {}",
            nba_checksum_path().display()
        ),
        Err(_) => eprintln!(
            "❌ failed to sign nba data with checksums in {}",
            nba_checksum_path().display()
        ),
    };
}

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    for szn in nba_lifespan() {
        save_nba_season(szn).await;
    }
}

pub fn rate_nba() {
    let mut tracker = EloTracker::new();

    match tracker.process_elo() {
        Ok(_) => println!("✅  Elo data processed successfully"),
        Err(_) => println!("❌  Error processing elo data"),
    }

    match tracker.save() {
        Ok(_) => println!("✅  Elo data saved successfully"),
        Err(e) => println!("❌  Error saving elo data: {}", e),
    };
}
