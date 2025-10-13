use crate::corrections::correction::Correction;
use crate::corrections::{correction_loader::load_season_corrections, corrector::Corrector};

use crate::dapi::team_box_score::TeamBoxScore;

use crate::format::parse::{destructure_dt, DT};
use crate::format::{path_manager::nba_data_path, season::season_fmt};

use crate::proc::hunting::BEGINNING;

use crate::stats::domain::Domain;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_period::minimum_spanning_era;

use chrono::Local;

use std::collections::HashMap;
use std::path::PathBuf;

pub fn revise_nba() {
    let DT { year, month, day } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = BEGINNING; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        let eras = minimum_spanning_era(szn);

        let player_corrections = load_season_corrections(szn, NBAStatKind::Player);

        let mut player_archives = eras
            .iter()
            .map(|x| (x, NBAStatKind::Player))
            .map(|(&s, k)| ((s, k), nba_data_path(s, k)))
            .collect::<HashMap<Domain, PathBuf>>();

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

        let team_corrections = load_season_corrections(szn, NBAStatKind::Team);

        let mut team_archives = eras
            .iter()
            .map(|x| (x, NBAStatKind::Team))
            .map(|(&s, k)| ((s, k), nba_data_path(s, k)))
            .collect::<HashMap<Domain, PathBuf>>();

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

pub fn revise_nba_season(year: i32, games: &mut Vec<(Identity, TeamBoxScore)>) -> Result<(), ()> {
    let mut player_corrections = load_season_corrections(year, NBAStatKind::Player)
        .map_err(|msg| {
            eprintln!(
                "{msg}\n❌ failed to load player corrections for the {} season:",
                season_fmt(year)
            );
            ()
        })?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    let mut team_corrections = load_season_corrections(year, NBAStatKind::Team)
        .map_err(|msg| {
            eprintln!(
                "{msg}\n❌ failed to load team corrections for the {} season:",
                season_fmt(year)
            );
            ()
        })?
        .into_iter()
        .map(|correction| (correction.identity(), correction))
        .collect::<HashMap<Identity, Correction>>();

    for (identity, game) in games.iter_mut() {
        if let Some(correction) = team_corrections.get_mut(&identity) {
            game.correct_box_score(correction);
        }

        //apply player corrections
        for player in game.roster_mut() {
            let mut player_identity = identity.clone();

            player_identity.player_id = Some(player.player_id());

            if let Some(correction) = player_corrections.get_mut(&player_identity) {
                player.correct_box_score(correction);
            }
        }
    }

    Ok(())
}
