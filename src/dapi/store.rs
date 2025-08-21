use crate::corrections::correction::Correction;
use crate::corrections::correction_builder::{self, CorrectionBuilder};
use crate::corrections::corrector::Corrector;
use crate::dapi::archive::typed_domain_archive_pairs;
use crate::dapi::hunting::load_nba_season_from_file;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::format::season::season_fmt;
use crate::stats::domain::Domain;
use crate::stats::game_obj::GameObject;
use crate::stats::id::Identity;
use crate::stats::nba_kind::NBAStatKind::{Player, Team};
use crate::types::GameId;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::path::PathBuf;

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

            // for cs in correction_builders.iter() {
            //     dbg!(cs);
            // }

            let corrections: Vec<Correction> = correction_builders
                .iter_mut()
                .map(|corr| corr.create())
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

type TeamGame = (Identity, TeamBoxScore);

fn pair_off(games: Vec<TeamGame>) -> Result<Vec<GameObject>, Vec<CorrectionBuilder>> {
    let mut pairs = HashMap::<GameId, (Option<TeamGame>, Option<TeamGame>)>::new();
    let mut corrections: Vec<CorrectionBuilder> = Vec::new();

    let l = games.len();

    for (id, game) in games.into_iter() {
        match pairs.get_mut(&id.game_id) {
            Some((game1, game2)) => match game1 {
                Some(_game1) => {
                    *game2 = Some((id, game));
                }
                None => {
                    *game1 = Some((id, game));
                }
            },
            None => {
                pairs.insert(id.game_id, (Some((id, game)), None));
            }
        }
    }

    //find and report unpaired unboxed games
    for (id, pair) in pairs.iter() {
        match pair {
            (Some(game), None) | (None, Some(game)) => {
                eprintln!(
                    "⚠️ unpaired game: {} season: {}",
                    id,
                    pair.0.as_ref().unwrap().0.season_id
                );

                let Identity {
                    season_id,
                    player_id,
                    team_id,
                    team_abbr,
                    game_id,
                    game_date,
                } = game.0.clone();

                let mut correction_builder = correction_builder::CorrectionBuilder::new(
                    game_id,
                    season_id,
                    player_id,
                    team_id,
                    team_abbr,
                    match player_id {
                        Some(_id) => Player,
                        None => Team,
                    },
                    game_date,
                );

                correction_builder.set_delete(true);

                corrections.push(correction_builder);
            }
            _ => {
                //do nothing
            }
        }
    }

    let mut games: Vec<GameObject> = Vec::with_capacity(l / 2);

    for (_game_id, (a, b)) in pairs.into_iter() {
        match (a, b) {
            (Some((id1, game1)), Some((id2, game2))) => {
                match GameObject::try_create(id1, game1, id2, game2) {
                    Ok(game_object) => {
                        games.push(game_object);
                    }
                    Err(mut corrections_builders) => {
                        corrections.append(&mut corrections_builders);
                    }
                }
            }
            _ => {}
        }
    }

    if corrections.len() > 0 {
        Err(corrections)
    } else {
        Ok(games)
    }
}
