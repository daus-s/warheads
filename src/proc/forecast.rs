use thiserror::Error;

use crate::dapi::timeline_manager::get_next_n_dates;

use crate::format::parse::{parse_gamecards, ParseError};

use crate::proc::query::{get_gamecard_json, NBAQueryError};

use crate::stats::gamecard::GameCard;
use crate::stats::prediction::Prediction;

use crate::types::GameDate;

pub(crate) async fn forecast_nba(
    forecaster: impl Forecaster,
    days: usize,
) -> Result<Vec<Prediction>, ForecastError> {
    let cards = get_upcoming_games(days).await?;

    Ok(forecaster.forecast(cards))
}

async fn get_upcoming_games(n: usize) -> Result<Vec<GameCard>, ForecastError> {
    // Forecast NBA games
    let today = GameDate::today();

    let mut upcoming_games = Vec::new();
    // find upcoming games todo: optionally, set this to run until the end of the regular season.
    for day in get_next_n_dates(today, n) {
        // Fetch games for the day
        let daily_game_card = get_gamecard_json(day)
            .await
            .map_err(|e| ForecastError::APIError(e))?;

        let gamecards =
            parse_gamecards(daily_game_card).map_err(|e| ForecastError::ParseError(e))?;

        upcoming_games.extend(gamecards);
    }

    Ok(upcoming_games)
}

#[derive(Debug, Error)]
pub enum ForecastError {
    #[error("{0}\n❌ failed to access NBA API. ")]
    APIError(NBAQueryError),
    #[error("{0}\n❌ failed to parse JSON response as gamecards. ")]
    ParseError(ParseError),
}

pub trait Forecaster {
    fn forecast(&self, cards: Vec<GameCard>) -> Vec<Prediction>;
}
