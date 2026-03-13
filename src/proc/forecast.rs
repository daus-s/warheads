use thiserror::Error;

use crate::dapi::season_manager::get_current_era;
use crate::dapi::timeline_manager::get_next_n_dates;

use crate::format::parse::{parse_gamecards, ParseError};

use crate::ml::model::Model;
use crate::proc::query::{get_gamecard_json, NBAQueryError};

use crate::stats::chronology::Chronology;
use crate::stats::gamecard::GameCard;
use crate::stats::prediction::Prediction;

use crate::types::GameDate;

pub(crate) async fn forecast_nba(
    mut model: impl Model,
    days: usize,
) -> Result<Vec<Prediction>, ForecastError> {
    println!("📥 fetching upcoming games from nba.com...");

    let mut cards = get_upcoming_games(days).await?;

    println!("🗓️  successfully got upcoming nba schedule");

    println!("📜 loading current era chronology...");

    let chronology = Chronology::from_era(get_current_era());

    println!("📖 loaded current era chronology.");

    for card in cards.iter_mut() {
        card.add_away_roster(chronology.get_expected_roster(card.away().team_id(), card.game_id()));
        card.add_home_roster(chronology.get_expected_roster(card.home().team_id(), card.game_id()));
    }
    println!("📖 assigned expected rosters to all cards");

    println!("🔮 generating predictions...");
    Ok(cards
        .iter()
        .map(|card| Prediction::new(card, model.predict(&card)))
        .collect())
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
