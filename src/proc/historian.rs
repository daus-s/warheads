use std::time::Instant;

use chrono::Datelike;

use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::sign::sign_nba;

use crate::dapi::currency::source_data_current;
use crate::dapi::season_manager::{get_current_era, nba_lifespan, nba_lifespan_period};

use crate::format::path_manager::nba_checksum_file;

use crate::ml::elo_tracker::EloTracker;

use crate::ml::model::Model;
use crate::proc::gather::fetch_and_save_nba_stats;
use crate::proc::hunting::compare_and_fetch;

use crate::proc::store::store_nba_season;
use crate::stats::chronology::Chronology;
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
        if let Err(_) = read_entire_nba_season(szn) {
            store_nba_season(szn);
        }
    }

    let current_year = chrono::Utc::now().year();

    store_nba_season(current_year); //always update the current year's season
}

pub fn rate_nba(elo_tracker: &mut EloTracker) {
    let mut chronology = Chronology::new();

    for era in nba_lifespan_period() {
        let era_start = Instant::now();

        println!("rating games for {}", era);

        let read_start = Instant::now();

        println!("loading chrono took {}ms", read_start.elapsed().as_millis());
        //waht is happening on this line and why so slow

        if let Ok(_) = chronology.load_year(era) {
            let rate_start = Instant::now();

            //
            let games = chronology
                .games()
                .as_ref()
                .expect("💀 load chronology but failed to access games. ")
                .iter()
                .map(|game| {
                    let mut card = game.card();

                    card.add_home_ratings(
                        chronology.get_expected_roster(card.home().team_id(), card.game_id()),
                    );

                    card.add_away_ratings(
                        chronology.get_expected_roster(card.away().team_id(), card.game_id()),
                    );

                    (card, game.clone())
                }) //fuck it
                .collect::<Vec<_>>();

            elo_tracker.train(&games);

            println!(
                "calculating ratings took {}ms",
                rate_start.elapsed().as_millis()
            );
        }

        println!("total time for {era}={}ms", era_start.elapsed().as_millis());
    }
}
