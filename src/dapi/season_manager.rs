use std::ops::Range;

use chrono::Local;

use crate::constants::constants::BEGINNING;

use crate::format::parse::{destructure_dt, DestructuredDateTime};

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
