use indicatif::{ProgressBar, ProgressStyle};
use stats::team_box_score::TeamBoxScore;
use crate::hunting::load_nba_season_from_file;

pub async fn save_nba_season(year: i32) {

    let team_games = load_nba_season_from_file(year);

    sub_save(team_games).await;
}

async fn sub_save(games: Vec<TeamBoxScore>) {
    let client = sss::client::create().await;

    let num_games = games.len() as u64;

    let pb = ProgressBar::new(num_games);

    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {bar:40} | {pos}/{len} [{eta}]")
        .unwrap()
        .progress_chars("#>-"));

    for game in &games {
        sss::store_three::save_nba_game(client.clone(), game.clone()).await.unwrap();

        pb.inc(1);
    }

    pb.finish_with_message(format!("saved {} season.", games[0].season_str()));
}