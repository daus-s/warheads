use crate::dapi::hunting::load_nba_season_from_file;
use crate::dapi::team_box_score::TeamBoxScore;
use crate::format::season::season_fmt;
use crate::stats::game_obj::GameObject;
use crate::stats::id::Identity;
use indicatif::{ProgressBar, ProgressStyle};

pub async fn save_nba_season(year: i32) {
    let team_games = load_nba_season_from_file(year);

    sub_save(team_games).await;
}

async fn sub_save(season: Vec<(Identity, TeamBoxScore)>) {
    // let client = crate::storage::client::create().await;

    if season.len() == 0 {
        return;
    }

    let pairs = pair_off(season);

    let num_games = pairs.len() as u64;

    if num_games == 0 {
        return;
    }

    let szn = pairs[0].season().year();

    let pb = ProgressBar::new(num_games);

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

    for game in &pairs {
        crate::storage::store_disk::save_nba_game(game).unwrap();

        pb.inc(1);
    }

    pb.finish_with_message(format!("saved {} season.", season_fmt(szn)));
}

fn pair_off(mut games: Vec<(Identity, TeamBoxScore)>) -> Vec<GameObject> {
    let mut pairs = Vec::new();

    for (id1, box_score1) in games.iter() {
        for (id2, box_score2) in games.iter() {
            if id1.game_id == id2.game_id {
                // while these subsequent checks are redundant it seems like a good check as if they
                // don't match then our data is malformed so we will exit the program.
                if id1.game_date == id2.game_date || id1.season_id == id2.season_id {
                    panic!("💀 malformed game's with matching GameId's have inconsistent data.")
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

    pairs
}
