use std::collections::HashMap;

use crate::dapi::timeline_manager::get_next_n_dates;

use crate::format::parse::parse_gamecards;

use crate::proc::prophet::write_predictions;
use crate::proc::query::get_gamecard_json;
use crate::stats::gamecard::GameCard;
use crate::types::{GameDate, PlayerId};

pub(crate) async fn forecast_nba(ratings: &mut HashMap<PlayerId, i64>) {
    let gamecard_response = get_upcoming_games(7).await;

    let upcoming_games = if let Ok(games) = gamecard_response {
        games
    } else if let Err(e) = gamecard_response {
        panic!("{e}\n💀 failed to fetch upcoming games")
    } else {
        unreachable!()
    };

    todo!();

    // let predictions = predict_cards(ratings, upcoming_games);

    // if let Err(e) = write_predictions(ratings, &predictions) {
    //     println!("{e}\n⚠️ predictions were generated but failed to write predictions to file. ")
    // };
}

async fn get_upcoming_games(n: usize) -> Result<Vec<GameCard>, Box<dyn std::error::Error>> {
    // Forecast NBA games
    let today = GameDate::today();

    let mut upcoming_games = Vec::new();
    // find upcoming games todo: optionally, set this to run until the end of the regular season.
    for day in get_next_n_dates(today, n) {
        // Fetch games for the day
        let daily_game_card = get_gamecard_json(day).await?;

        let gamecards = parse_gamecards(daily_game_card)?;

        upcoming_games.extend(gamecards);
    }

    Ok(upcoming_games)
}
