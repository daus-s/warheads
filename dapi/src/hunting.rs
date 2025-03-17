/*
    ITS HUNTING SEASON.
    PROFESSIONAL SPORTS LEAGUE SEASONS MANAGEMENT DONE HERE

    GOOD WILL HUNTING
*/

use stats::kind::{NBAStatKind, NBAStat};
use stats::kind::NBAStat::*;
use stats::player_box_score::PlayerBoxScore;
use stats::team_box_score::TeamBoxScore;
use crate::rip::process_nba_games;

use chrono;
use chrono::{DateTime, Datelike, Local};
use indicatif::{ProgressBar, ProgressStyle};

fn player_games(year: i32) -> Vec<PlayerBoxScore> {
    match process_nba_games(year, NBAStatKind::Player) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                Player(p) => Some(p),
                _ => None,
            })
            .collect::<Vec<PlayerBoxScore>>(),
        Err(corrections) => {
            for mut correction in corrections {
                correction.create(); //if this function depends on user input does this execution around it pause?

                correction.apply().expect("couldn't open file to [rw]");

                // id like to make this recursive in the following case as well. essentially
            }
            player_games(year)
        }
    }
}

fn team_games(year: i32, roster: Vec<PlayerBoxScore>) -> Vec<TeamBoxScore> {
    let mut games = match process_nba_games(year, NBAStatKind::Team) {
        Ok(games) => games
            .into_iter()
            .filter_map(|x| match x {
                Team(t) => Some(t),
                _ => None,
            })
            .collect::<Vec<TeamBoxScore>>(),
        Err(corrections) => {
            for mut correction in corrections {
                correction.create(); // If this function depends on user input, execution will pause here

                correction.apply().expect("couldn't open file to [rw]");

                // Recursively retry after applying corrections
            }
            team_games(year, roster.clone()) // Recursive call to retry after corrections
        }
    };

    for player in roster {
        for team in &mut games {
            if player.played_in(&team) {
                team.add_player_stats(player.clone());
            }
        }
    }

    games

}
pub fn load_nba_season(year: i32) -> Vec<TeamBoxScore> {
    let player_games = player_games(year);

    team_games(year, player_games)
}

pub async fn save_nba_season(year: i32) {

    let team_games = load_nba_season(year);

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

/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let todays_date = Local::now();

    let D {year, month, day} = destructure_dt(todays_date);

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 { 1 } else { 0 };// august14th

    let begin = 1946; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        save_nba_season(szn).await;
    }
}

fn destructure_dt(dt: DateTime<Local>) -> D {
    D {
        year: dt.year(),
        month: dt.month(),
        day: dt.day(),
    }
}

struct D {
    year: i32,
    month: u32,
    day: u32,
}
pub fn process_elo() {
    todo!("assign elo values to players on a game by game basis")
}