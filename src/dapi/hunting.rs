use crate::dapi::archive::typed_domain_archive_pairs;
use crate::dapi::season_manager::nba_lifespan;
/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/
use crate::dapi::team_box_score::TeamBoxScore;

use crate::corrections::correction_loader::load_season_corrections;
use crate::corrections::corrector::Corrector;
use crate::dapi::gather;
use crate::dapi::gather::{player_games, team_games};
use crate::dapi::store::save_nba_season;
use crate::format::season::season_fmt;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::stats::season_period::minimum_spanning_era;

pub fn load_nba_season_from_file(year: i32) -> Vec<(Identity, TeamBoxScore)> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    for szn in nba_lifespan() {
        save_nba_season(szn).await;
    }
}

pub async fn observe_nba() {
    for year in nba_lifespan() {
        for era in minimum_spanning_era(year) {
            if let Err(msg) = gather::fetch_and_save_nba_stats(era, Player).await {
                eprintln!("{}", msg);
            }

            if let Err(msg) = gather::fetch_and_save_nba_stats(era, Team).await {
                eprintln!("{}", msg);
            }
        }
    }
}

pub fn revise_nba() {
    for szn in nba_lifespan() {
        let player_corrections = load_season_corrections(szn, Player);

        let mut player_archives = typed_domain_archive_pairs(szn, Player);

        if let Ok(corrections) = player_corrections {
            if let Err(msg) = corrections.apply(&mut player_archives) {
                println!(
                    "{msg}\n❌ failed to overwrite NBA player data for {}",
                    season_fmt(szn)
                );
            }
        } else if let Err(msg) = player_corrections {
            eprintln!(
                "{msg}\n❌ failed to load player corrections for the {} season:",
                season_fmt(szn)
            )
        }

        let team_corrections = load_season_corrections(szn, Team);

        let mut team_archives = typed_domain_archive_pairs(szn, Team);

        if let Ok(corrections) = team_corrections {
            if let Err(msg) = corrections.apply(&mut team_archives) {
                println!(
                    "{msg}\n❌ failed to overwrite NBA team data for {}",
                    season_fmt(szn)
                );
            }
        } else if let Err(msg) = team_corrections {
            eprintln!(
                "{msg}\n❌ failed to load team corrections for the {} season:",
                season_fmt(szn)
            )
        }
    }
}
