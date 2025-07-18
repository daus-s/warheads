use crate::dapi::hunting::load_nba_season_from_file;
use crate::dapi::team_box_score::TeamBoxScore;
use indicatif::{ProgressBar, ProgressStyle};
use crate::format::season::season_fmt;
use crate::stats::box_score::BoxScore;
use crate::stats::game_obj::GameObject;

pub async fn save_nba_season(year: i32) {
    let team_games = load_nba_season_from_file(year);

    sub_save(team_games).await;
}

async fn sub_save(season: Vec<TeamBoxScore>) {
    // let client = crate::storage::client::create().await;

    if season.len()  == 0 {
        return
    }

    let szn = season[0].season().year();

    let pairs = pair_off(season);

    let num_games = pairs.len() as u64;

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

fn pair_off(games: Vec<TeamBoxScore>) -> Vec<GameObject> {
    let mut pairs = Vec::new();

    // for game1 in games.iter_mut() {
    //     for game2 in games.iter_mut() {
    //         if game1.game_id() == game2.game_id() && game1.team_id() != game2.team_id() {
    //             // pairs.append(GameObject::create())
    //         }
    //     }
    // }

    pairs
}