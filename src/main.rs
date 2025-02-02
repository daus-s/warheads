use dapi::rip::{process_nba_games};
use dapi::gather::read_nba;

use stats::kind::{NBAStatKind, NBAStatType};


use sss;
use stats::player_box_score::PlayerBoxScore;
use stats::team_box_score::TeamBoxScore;

#[tokio::main]
async fn main() {
    println!("hello, lisan al-gaib!"); //TODO: make this say hi to the user with auth/name

    let player_games = match process_nba_games(&read_nba(2023, NBAStatKind::Player), NBAStatKind::Player) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                NBAStatType::Player(p) => Some(p),
                _ => None,
            })
            .collect::<Vec<PlayerBoxScore>>(),
        Err(e) => unreachable!("{}", e),
    };

    let mut team_games = match process_nba_games(&read_nba(2023, NBAStatKind::Team), NBAStatKind::Team) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                NBAStatType::Team(t) => Some(t),
                _ => None,
            })
            .collect::<Vec<TeamBoxScore>>(),
        Err(e) => unreachable!("{}", e),
    };

    let msg = format!("team games has {} entries", &team_games.len());
    dbg!(msg);

    for player in &player_games {
        for team in &mut team_games {
            if player.played_in(team.clone()) {
                team.add_player_stats(player.clone());
            }
        }
    }

    let client = sss::client::create().await;

    for game in &team_games {
        sss::store_three::save_nba_game(client.clone(), game.clone()).await.unwrap();
    }
}
