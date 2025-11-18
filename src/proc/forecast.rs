use crate::dapi::timeline_manager::get_next_n_dates;

use crate::format::parse::parse_gamecards;
use crate::ml::elo_tracker::EloTracker;

use crate::proc::query::get_gamecard_json;
use crate::stats::chronology::Chronology;
use crate::stats::gamecard::GameCard;
use crate::types::GameDate;

pub async fn forecast_nba(elo: &EloTracker) {
    let mut chronology = Chronology::new();

    let upcoming_games = get_upcoming_games()
        .await
        .expect("Failed to fetch upcoming games");

    for game in upcoming_games {
        chronology
            .load_year(game.season())
            .expect("Failed to load year from storage");

        let (home, away) = (game.home(), game.away());

        let home_roster = chronology
            .get_expected_roster(home.team_id(), game.game_id())
            .expect("Failed to get home roster");

        let away_roster = chronology
            .get_expected_roster(away.team_id(), game.game_id())
            .expect("Failed to get away roster");

        todo!();
        let home_rating = 1;
    }
}

async fn get_upcoming_games() -> Result<Vec<GameCard>, Box<dyn std::error::Error>> {
    // Forecast NBA games
    let today = GameDate::today();

    // find upcoming games todo: optionally, set this to run until the end of the regular season.
    for day in get_next_n_dates(today, 7) {
        // Fetch games for the day
        let daily_game_card = get_gamecard_json(day).await?;

        let gamecards = parse_gamecards(daily_game_card)?;
    }

    Ok(vec![])
}
