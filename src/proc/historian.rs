use chrono::Datelike;

use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::sign::sign_nba;

use crate::dapi::currency::source_data_current;
use crate::dapi::season_manager::{get_current_era, nba_lifespan, nba_lifespan_period};

use crate::format::path_manager::nba_checksum_file;

use crate::ml::elo_tracker::EloTracker;

use crate::proc::gather::fetch_and_save_nba_stats;
use crate::proc::hunting::compare_and_fetch;

use crate::proc::store::store_nba_season;
use crate::stats::nba_kind::NBAStatKind;
use crate::storage::read_disk::read_entire_nba_season;

pub async fn observe_nba() {
    let checksums = ChecksumMap::load().expect("💀 failed to load checksums");

    let mut errors = 0;

    let eras = nba_lifespan_period();

    for era in &eras[0..eras.len() - 1] {
        errors += compare_and_fetch(*era, NBAStatKind::Player, &checksums).await;
        errors += compare_and_fetch(*era, NBAStatKind::Team, &checksums).await;
    }

    let current_era = get_current_era();

    if !source_data_current().await {
        let _ = fetch_and_save_nba_stats(current_era, NBAStatKind::Player).await;
        let _ = fetch_and_save_nba_stats(current_era, NBAStatKind::Team).await;

        errors += 1;
        errors += 1;
    }

    if errors > 0 {
        match sign_nba() {
            Ok(_) => println!(
                "✅ successfully signed nba data with checksums in {}",
                nba_checksum_file().display()
            ),
            Err(_) => eprintln!(
                "❌ failed to sign nba data with checksums in {}",
                nba_checksum_file().display()
            ),
        };
    }
}

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub fn chronicle_nba() {
    for szn in nba_lifespan() {
        if let Err(e) = read_entire_nba_season(szn) {
            store_nba_season(szn);
        }
    }

    let current_year = chrono::Utc::now().year();

    store_nba_season(current_year); //always update the current year's season
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
