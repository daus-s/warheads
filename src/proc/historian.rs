use chrono::Datelike;

use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::sign::sign_nba;

use crate::dapi::currency::source_data_current;
use crate::dapi::season_manager::{get_current_era, nba_lifespan, nba_lifespan_period};

use crate::format::path_manager::nba_checksum_file;

use crate::ml::elo_tracker::{EloTracker, EloTrackerError};

use crate::proc::gather::fetch_and_save_nba_stats;
use crate::proc::hunting::compare_and_fetch;

use crate::proc::store::store_nba_season;
use crate::stats::nba_kind::NBAStatKind;
use crate::storage::read_disk::{read_entire_nba_season, read_nba_season};

pub async fn observe_nba() {
    match ChecksumMap::load() {
        Ok(checksums) => {
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
                    Err(_) => println!(
                        "❌ failed to sign nba data with checksums in {}",
                        nba_checksum_file().display()
                    ),
                };
            }
        }
        Err(_) => {
            for era in nba_lifespan_period() {
                let _ = fetch_and_save_nba_stats(era, NBAStatKind::Player).await;
                let _ = fetch_and_save_nba_stats(era, NBAStatKind::Team).await;
            }

            sign_nba().expect("💀 should be able to write checksums. ");
        }
    }
}

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub fn chronicle_nba() {
    for era in nba_lifespan_period() {
        if let Err(_) = read_nba_season(era) {
            store_nba_season(era);
        }
    }

    let current_year = get_current_era();

    store_nba_season(current_year); //always update the current year's season
}

pub async fn rate_nba() -> Result<EloTracker, EloTrackerError> {
    EloTracker::train()
}
