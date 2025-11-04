use crate::dapi::timeline_manager::get_next_n_dates;
use crate::types::GameDate;

pub async fn forecast_nba() {
    get_upcoming_games();
}

async fn get_upcoming_games() {
    // Forecast NBA games
    let today = GameDate::today();

    // find upcoming games todo: optionally, set this to run until the end of the regular season.
    for day in get_next_n_dates(today, 7) {
        // Fetch games for the day
        todo!()
    }
}
