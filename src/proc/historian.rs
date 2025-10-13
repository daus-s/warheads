use chrono::Local;

use crate::format::parse::{destructure_dt, DT};
use crate::proc::hunting::BEGINNING;
use crate::proc::store::save_nba_season;

/// this module contains functions for writing the history of the nba stats
/// you can build around this function but not from it... this is the one function to start the nba into memory then iterate over elo.
pub async fn chronicle_nba() {
    let DT { year, month, day } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    let begin = BEGINNING; //first year of the nba in record is 1946-1947 szn

    for szn in begin..year + seasonal_depression {
        save_nba_season(szn).await;
    }
}
