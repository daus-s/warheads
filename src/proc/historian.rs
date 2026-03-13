use crate::checksum::checksum_map::ChecksumMap;
use crate::checksum::sign::sign_nba;

use crate::dapi::season_manager::{get_current_era, nba_lifespan_period};

use crate::edit::edit_list::EditList;
use crate::edit::edit_loader::load_edit_list;
use crate::format::path_manager::nba_checksum_file;

use crate::ml::elo_tracker::EloTracker;
use crate::ml::model::Model;

use crate::proc::gather::fetch_and_save_nba_stats;
use crate::proc::hunting::compare_and_fetch;
use crate::proc::query::nba_annotation_file;
use crate::proc::store::inscribe;

use crate::stats::chronology::Chronology;
use crate::stats::nba_kind::NBAStatKind;

use crate::dapi::read_disk::read_nba_season;

pub async fn observe_nba() {
    match ChecksumMap::load() {
        Ok(checksums) => {
            let eras = nba_lifespan_period();
            for era in &eras[0..eras.len() - 1] {
                compare_and_fetch(*era, NBAStatKind::Player, &checksums).await;
                compare_and_fetch(*era, NBAStatKind::Team, &checksums).await;
            }
            let current_era = get_current_era();
            match fetch_and_save_nba_stats(current_era, NBAStatKind::Player).await {
                Ok(_) => {
                    println!("✅ successfully fetched and saved nba player stats for {current_era}")
                }
                Err(e) => {
                    println!("❌ failed to fetch and save nba player stats for {current_era}\n{e}")
                }
            }
            match fetch_and_save_nba_stats(current_era, NBAStatKind::Team).await {
                Ok(_) => {
                    println!("✅ successfully fetched and saved nba team stats for {current_era}")
                }
                Err(e) => {
                    println!("❌ failed to fetch and save nba team stats for {current_era}\n{e}")
                }
            }
            match sign_nba() {
                Ok(_) => println!(
                    "✅ successfully signed nba data with checksums in {}",
                    nba_checksum_file().display()
                ),
                Err(_) => println!(
                    "❌ failed to sign nba data with checksums in {}",
                    nba_checksum_file().display()
                ),
            }
        }
        Err(_) => {
            for era in nba_lifespan_period() {
                match fetch_and_save_nba_stats(era, NBAStatKind::Player).await {
                    Ok(_) => {
                        println!("✅ successfully fetched and saved nba player stats for {era}")
                    }
                    Err(e) => {
                        println!("❌ failed to fetch and save nba player stats for {era}\n{e}")
                    }
                }
                match fetch_and_save_nba_stats(era, NBAStatKind::Team).await {
                    Ok(_) => println!("✅ successfully fetched and saved nba team stats for {era}"),
                    Err(e) => println!("❌ failed to fetch and save nba team stats for {era}\n{e}"),
                }
            }
            match sign_nba() {
                Ok(_) => println!(
                    "✅ successfully signed nba data with checksums in {}",
                    nba_checksum_file().display()
                ),
                Err(_) => println!(
                    "❌ failed to sign nba data with checksums in {}",
                    nba_checksum_file().display()
                ),
            }
        }
    }
}

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub fn chronicle_nba() {
    for season in nba_lifespan_period() {
        if read_nba_season(season).is_err() || season.is_current_era() {
            match inscribe(season) {
                Ok(_) => println!("✅ successfully chronicled {}", season),
                Err(e) => println!("{e}\n❌ failed to chronicle {}", season),
            }
        }
    }
}

pub fn rate_nba(elo_tracker: &mut EloTracker) {
    let mut chronology = Chronology::new();

    for era in nba_lifespan_period() {
        if let Ok(_) = chronology.load_era(era) {
            let games = chronology
                .games()
                .as_ref()
                .expect("💀 load chronology but failed to access games. ")
                .iter()
                .map(|game| {
                    let mut card = game.card();

                    card.add_home_roster(
                        chronology.get_expected_roster(card.home().team_id(), card.game_id()),
                    );

                    card.add_away_roster(
                        chronology.get_expected_roster(card.away().team_id(), card.game_id()),
                    );

                    (card, game.clone())
                })
                .collect::<Vec<_>>();

            elo_tracker.train(&games);
        }
    }

    if let Err(e) = elo_tracker.save() {
        println!("{}\n❌ failed to serialize elo tracker.", e);
    };
}

pub(crate) async fn annotate_nba() {
    match nba_annotation_file().await {
        Ok(json) => {
            let mut previous = load_edit_list().unwrap_or_default();

            let new = serde_json::from_str::<EditList>(&json.to_string()).unwrap_or_default();

            previous.merge(new);

            if let Err(_) = previous.write_to_file() {
                println!("❌ failed to write edit list to file.");
            }
        }
        Err(e) => {
            println!("{e}\n❌ failed to fetch nba annotation file.");
        }
    }
}
