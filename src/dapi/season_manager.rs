use std::ops::Range;

use chrono::Local;

use crate::constants::constants::BEGINNING;

use crate::stats::season_period::{minimum_spanning_era, SeasonPeriod::*};

use crate::format::parse::{destructure_dt, DestructuredDateTime};
use crate::types::{GameDate, SeasonId};

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

// this is approximate and is not actually synchronized with the NBA calendar.
// (currently set to match 2025-26 schedule)
pub fn nba_lifespan_period() -> Vec<SeasonId> {
    let DestructuredDateTime {
        year: curr_year,
        month,
        day,
    } = destructure_dt(Local::now());

    let mut seasons = Vec::new();

    for year in BEGINNING..(curr_year - 1) {
        seasons.extend(minimum_spanning_era(year))
    }

    let prev_year = curr_year - 1;

    match month {
        1 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
        }
        2 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
        }
        3 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
        }
        4 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
            match day {
                1..=14 => {}
                15..=17 => {
                    seasons.push(SeasonId::from((prev_year, PlayIn)));
                }
                18..=30 => {
                    seasons.push(SeasonId::from((prev_year, PlayIn)));
                    seasons.push(SeasonId::from((prev_year, PostSeason)));
                }
                _ => unreachable!("💀 30 days in April"),
            }
        }
        5 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
            seasons.push(SeasonId::from((prev_year, PlayIn)));
            seasons.push(SeasonId::from((prev_year, PostSeason)));
        }
        6..=9 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
            seasons.push(SeasonId::from((prev_year, PlayIn)));
            seasons.push(SeasonId::from((prev_year, PostSeason)));
        }
        10 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
            seasons.push(SeasonId::from((prev_year, PlayIn)));
            seasons.push(SeasonId::from((prev_year, PostSeason)));
            match day {
                1..=20 => {
                    seasons.push(SeasonId::from((curr_year, PreSeason)));
                }
                21..=31 => {
                    seasons.push(SeasonId::from((curr_year, PreSeason)));
                    seasons.push(SeasonId::from((curr_year, RegularSeason)));
                }
                _ => unreachable!("💀 31 days in April."),
            }
        }
        11..=12 => {
            seasons.push(SeasonId::from((prev_year, PreSeason)));
            seasons.push(SeasonId::from((prev_year, RegularSeason)));
            seasons.push(SeasonId::from((prev_year, PlayIn)));
            seasons.push(SeasonId::from((prev_year, PostSeason)));
            seasons.push(SeasonId::from((curr_year, PreSeason)));
            seasons.push(SeasonId::from((curr_year, RegularSeason)));
        }
        _ => unreachable!(""),
    };

    seasons
}

pub fn get_current_era() -> SeasonId {
    let today = GameDate::today();

    get_era_by_date(today)
}

pub fn get_era_by_date(date: GameDate) -> SeasonId {
    let (year, month, day) = date.destructure();

    let prev_year = year - 1;

    match month {
        1 => SeasonId::from((prev_year, RegularSeason)),
        2 => SeasonId::from((prev_year, RegularSeason)),
        3 => SeasonId::from((prev_year, RegularSeason)),
        4 => match day {
            1..=14 => SeasonId::from((prev_year, RegularSeason)),
            15..=17 => SeasonId::from((prev_year, PlayIn)),
            18..=30 => SeasonId::from((prev_year, PostSeason)),
            _ => unreachable!("💀 30 days in April"),
        },
        5..=9 => SeasonId::from((prev_year, PostSeason)),
        10 => match day {
            1..=20 => SeasonId::from((year, PreSeason)),
            21..=31 => SeasonId::from((year, RegularSeason)),
            _ => unreachable!("💀 31 days in October."),
        },
        11..=12 => SeasonId::from((year, RegularSeason)),
        _ => unreachable!("💀 12 months in a year."),
    }
}

#[cfg(test)]
mod test_season_manager {
    use super::*;

    #[test]
    fn test_last_era_in_lifespan_is_current_eta() {
        let current_era = get_current_era();
        let last_era_in_lifespan = nba_lifespan_period()[nba_lifespan_period().len() - 1];
        assert_eq!(last_era_in_lifespan, current_era);
    }
}
