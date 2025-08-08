use crate::corrections::correction::Correction;
use crate::corrections::correction_builder::CorrectionBuilder;
use crate::corrections::corrector::Corrector;
use crate::dapi::archive::typed_domain_archive_pairs;
use crate::dapi::hunting::load_nba_season_from_file;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::format::season::season_fmt;
use crate::stats::domain::Domain;
use crate::stats::game_metadata::GameDisplay;
use crate::stats::game_obj::GameObject;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind::Team;
use crate::types::{Matchup, SeasonId};
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::PathBuf;
use serde_json::Value::Null;
use crate::stats::stat_column::StatColumn::MATCHUP;

pub async fn save_nba_season(year: i32) {
    let team_games = load_nba_season_from_file(year);

    //correct any issues with pairing off
    let games = match pair_off(team_games) {
        Err(mut correction_builders) => {
            println!(
                "ℹ️ there are {} {} corrections to make for the {} season.",
                correction_builders.len(),
                Team,
                season_fmt(year)
            );

            for cs in correction_builders.iter() {
                dbg!(cs);
            }

            let corrections: Vec<Correction> = correction_builders
                .iter_mut()
                .map(|c| c.create())
                .collect();

            let mut domain_archive: HashMap<Domain, PathBuf> =
                typed_domain_archive_pairs(year, Team);

            corrections
                .apply(&mut domain_archive)
                .expect("💀 failed to apply corrections to team data.");

            pair_off(load_nba_season_from_file(year))
                .expect("💀 applied corrections successfully but did not resolve the issue.")
        }
        Ok(games) => games,
    };

    sub_save(games).await;
}

async fn sub_save(season: Vec<GameObject>) {
    // let client = crate::storage::client::create().await;

    let num_games = season.len();

    if num_games == 0 {
        return;
    }

    let szn = season[0].season().year();

    let pb = ProgressBar::new(num_games as u64);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {bar:40} | {pos}/{len} [{eta}]")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb.set_message(format!(
        "saving box scores for the {} season. ",
        season_fmt(szn)
    ));

    for game in &season {
        crate::storage::store_disk::save_nba_game(game).unwrap();

        pb.inc(1);
    }

    pb.finish_with_message(format!("saved {} season.", season_fmt(szn)));
}

fn pair_off(
    mut games: Vec<(Identity, TeamBoxScore)>,
) -> Result<Vec<GameObject>, Vec<CorrectionBuilder>> {
    let mut pairs = Vec::new();
    let mut corrections = Vec::new();

    for (id1, box_score1) in games.iter() {
        for (id2, box_score2) in games.iter() {
            if id1.game_id == id2.game_id && box_score1.team_id != box_score2.team_id {
                // while these subsequent checks are redundant it seems like a good check as if they
                // don't match then our data is malformed so we will exit the program.
                if id1.game_date != id2.game_date || id1.season_id != id2.season_id {
                    panic!("💀 malformed game's with matching GameId's have inconsistent data.")
                } else {
                    //any issues with pairing off the games will be added here.
                    if box_score1.visiting() == box_score2.visiting() {
                        let mut correction1 = CorrectionBuilder::new(
                            id1.game_id,
                            id1.season_id,
                            None,
                            box_score1.team_id,
                            box_score1.team_abbr(),
                            Team,
                            id1.game_date,
                        );

                        let display1 = GameDisplay::new(
                            Matchup::from_matchup(id1.team_abbr.clone(), id2.team_abbr.clone()),
                            id1.game_date,
                            None,
                            id1.team_abbr.clone(),
                            box_score1.team_name(),
                        );

                        correction1.update_meta(display1);

                        // let mut correction2 = CorrectionBuilder::new(
                        //     id2.game_id,
                        //     id2.season_id,
                        //     None,
                        //     box_score2.team_id,
                        //     box_score2.team_abbr(),
                        //     Team,
                        //     id2.game_date,
                        // );
                        //
                        // let display2 = GameDisplay::new(
                        //     Matchup::from_matchup(id2.team_abbr.clone(), id1.team_abbr.clone()),
                        //     id2.game_date,
                        //     None,
                        //     id2.team_abbr.clone(),
                        //     box_score2.team_name(),
                        // );
                        // correction2.update_meta(display2);


                        // add correction fields
                        correction1.add_missing_field(MATCHUP, Null);
                        // correction2.add_missing_field(MATCHUP, Null);

                        corrections.push(correction1);
                        // corrections.push(correction2);

                    } else {
                        let game_object = GameObject::create(
                            id1.season_id,
                            id1.game_date,
                            id1.game_id,
                            box_score1.clone(),
                            box_score2.clone(),
                        ); //would rather not clone

                        pairs.push(game_object);
                    }
                }
            }
        }
    }

    if corrections.len() == 0 {
        Ok(pairs)
    } else {
        Err(corrections)
    }
}
