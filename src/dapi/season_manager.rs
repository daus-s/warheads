use std::ops::Range;

use chrono::Local;

use crate::constants::constants::BEGINNING;

use crate::stats::season_period::SeasonPeriod::*;

use crate::format::parse::{destructure_dt, DestructuredDateTime};
use crate::stats::season_period::minimum_spanning_era;
use crate::types::SeasonId;

pub fn nba_lifespan() -> Range<i32> {
    let DestructuredDateTime {
        year: curr_year,
        month,
        day,
    } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        1
    } else {
        0
    }; // august14th

    BEGINNING..curr_year + seasonal_depression
}

//this is approximate and is not actually synchronized with the NBA calendar.
pub fn nba_lifespan_period() -> Range<SeasonId> {
    let DestructuredDateTime {
        year: curr_year,
        month,
        day,
    } = destructure_dt(Local::now());

    let seasonal_depression = if month > 8 || month == 8 && day >= 14 {
        PreSeason
    } else if month >= 11 || month == 10 && day >= 21 {
        RegularSeason
    } else if month == 4 && day < 18 && day >= 10 {
        PlayIn
    } else if month > 4 && month <= 7 || month == 4 && day < 18 {
        PostSeason
    } else {
        unreachable!()
    };

    SeasonId::from((BEGINNING, RegularSeason))..SeasonId::from((curr_year, seasonal_depression))
}
