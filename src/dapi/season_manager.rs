use std::ops::Range;

use chrono::{Local, NaiveDate, Datelike};

use crate::constants::constants::BEGINNING;

use crate::stats::season_phase::{Phase::{self, *}, season_phases};

use crate::format::parse::{destructure_dt, DestructuredDateTime};
use crate::types::SeasonId;

pub fn nba_lifespan() -> Range<i32> {
    let DestructuredDateTime { year, month, day } = destructure_dt(Local::now());
    let seasonal_depression = if month > 8 || month == 8 && day >= 14 { 1 } else { 0 };
    BEGINNING..year + seasonal_depression
}

const PHASES: [(Phase, u32, u32, u32); 4] = [
    (PreSeason, 10, 1, 0),
    (RegularSeason, 10, 20, 0),
    (PlayIn, 4, 13, 1),
    (PostSeason, 4, 17, 1)
];


fn recent_phases(date: NaiveDate) -> impl Iterator<Item=(NaiveDate, SeasonId)> {
    let y = date.year();
    (y..=y+1).flat_map(|szn_yr| {
        let phases = season_phases(szn_yr);
        PHASES.iter().map(move |(phase, m, d, off)| {
            (
            NaiveDate::from_ymd_opt(szn_yr + *off as i32, *m, *d).unwrap()
            ,
            SeasonId::from((szn_yr + *off as i32, *phase))
            )
        })
        .filter(move |(_date, id)| phases.contains(&id))
    })
}

// this is approximate and is not actually synchronized with the NBA calendar.
// (currently set to match 2025-26 schedule)
fn nba_lifespan_at(date: NaiveDate) -> Vec<SeasonId> {
    let y = date.year();

    let mut phases: Vec<SeasonId> = (BEGINNING..y).flat_map(season_phases).collect();

    let mut recent: Vec<(NaiveDate, SeasonId)> = recent_phases(date).filter(|(d, _)| date >= *d).collect();
    recent.sort_by_key(|(date, _id)| *date);

    phases.extend(recent.into_iter().map(|(_date, id)| id));

    phases
}

pub fn nba_lifespan_period() -> Vec<SeasonId> {
    let today = chrono::Utc::now().date_naive();

    nba_lifespan_at(today)
}

pub fn get_current_era() -> SeasonId {
    let today = chrono::Utc::now().date_naive();

    recent_phases(today).filter(|(date, _id)| today >= *date).max_by_key(|(date, _id)| *date).unwrap().1
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

    #[test]
    fn test_era_order() {
        let expected: Vec<SeasonId> = vec![
            SeasonId::from((1946, RegularSeason)),
            SeasonId::from((1946, PostSeason)),
            SeasonId::from((1947, RegularSeason)),
            SeasonId::from((1947, PostSeason)),
            SeasonId::from((1948, RegularSeason)),
            SeasonId::from((1948, PostSeason)),
            SeasonId::from((1949, RegularSeason)),
            SeasonId::from((1949, PostSeason)),
            SeasonId::from((1950, RegularSeason)),
            SeasonId::from((1950, PostSeason)),
            SeasonId::from((1951, RegularSeason)),
            SeasonId::from((1951, PostSeason)),
            SeasonId::from((1952, RegularSeason)),
            SeasonId::from((1952, PostSeason)),
            SeasonId::from((1953, RegularSeason)),
            SeasonId::from((1953, PostSeason)),
            SeasonId::from((1954, RegularSeason)),
            SeasonId::from((1954, PostSeason)),
            SeasonId::from((1955, RegularSeason)),
            SeasonId::from((1955, PostSeason)),
            SeasonId::from((1956, RegularSeason)),
            SeasonId::from((1956, PostSeason)),
            SeasonId::from((1957, RegularSeason)),
            SeasonId::from((1957, PostSeason)),
            SeasonId::from((1958, RegularSeason)),
            SeasonId::from((1958, PostSeason)),
            SeasonId::from((1959, RegularSeason)),
            SeasonId::from((1959, PostSeason)),
            SeasonId::from((1960, RegularSeason)),
            SeasonId::from((1960, PostSeason)),
            SeasonId::from((1961, RegularSeason)),
            SeasonId::from((1961, PostSeason)),
            SeasonId::from((1962, RegularSeason)),
            SeasonId::from((1962, PostSeason)),
            SeasonId::from((1963, RegularSeason)),
            SeasonId::from((1963, PostSeason)),
            SeasonId::from((1964, RegularSeason)),
            SeasonId::from((1964, PostSeason)),
            SeasonId::from((1965, RegularSeason)),
            SeasonId::from((1965, PostSeason)),
            SeasonId::from((1966, RegularSeason)),
            SeasonId::from((1966, PostSeason)),
            SeasonId::from((1967, RegularSeason)),
            SeasonId::from((1967, PostSeason)),
            SeasonId::from((1968, RegularSeason)),
            SeasonId::from((1968, PostSeason)),
            SeasonId::from((1969, RegularSeason)),
            SeasonId::from((1969, PostSeason)),
            SeasonId::from((1970, RegularSeason)),
            SeasonId::from((1970, PostSeason)),
            SeasonId::from((1971, RegularSeason)),
            SeasonId::from((1971, PostSeason)),
            SeasonId::from((1972, RegularSeason)),
            SeasonId::from((1972, PostSeason)),
            SeasonId::from((1973, RegularSeason)),
            SeasonId::from((1973, PostSeason)),
            SeasonId::from((1974, RegularSeason)),
            SeasonId::from((1974, PostSeason)),
            SeasonId::from((1975, RegularSeason)),
            SeasonId::from((1975, PostSeason)),
            SeasonId::from((1976, RegularSeason)),
            SeasonId::from((1976, PostSeason)),
            SeasonId::from((1977, RegularSeason)),
            SeasonId::from((1977, PostSeason)),
            SeasonId::from((1978, RegularSeason)),
            SeasonId::from((1978, PostSeason)),
            SeasonId::from((1979, RegularSeason)),
            SeasonId::from((1979, PostSeason)),
            SeasonId::from((1980, RegularSeason)),
            SeasonId::from((1980, PostSeason)),
            SeasonId::from((1981, RegularSeason)),
            SeasonId::from((1981, PostSeason)),
            SeasonId::from((1982, RegularSeason)),
            SeasonId::from((1982, PostSeason)),
            SeasonId::from((1983, RegularSeason)),
            SeasonId::from((1983, PostSeason)),
            SeasonId::from((1984, RegularSeason)),
            SeasonId::from((1984, PostSeason)),
            SeasonId::from((1985, RegularSeason)),
            SeasonId::from((1985, PostSeason)),
            SeasonId::from((1986, RegularSeason)),
            SeasonId::from((1986, PostSeason)),
            SeasonId::from((1987, RegularSeason)),
            SeasonId::from((1987, PostSeason)),
            SeasonId::from((1988, RegularSeason)),
            SeasonId::from((1988, PostSeason)),
            SeasonId::from((1989, RegularSeason)),
            SeasonId::from((1989, PostSeason)),
            SeasonId::from((1990, RegularSeason)),
            SeasonId::from((1990, PostSeason)),
            SeasonId::from((1991, RegularSeason)),
            SeasonId::from((1991, PostSeason)),
            SeasonId::from((1992, RegularSeason)),
            SeasonId::from((1992, PostSeason)),
            SeasonId::from((1993, RegularSeason)),
            SeasonId::from((1993, PostSeason)),
            SeasonId::from((1994, RegularSeason)),
            SeasonId::from((1994, PostSeason)),
            SeasonId::from((1995, RegularSeason)),
            SeasonId::from((1995, PostSeason)),
            SeasonId::from((1996, RegularSeason)),
            SeasonId::from((1996, PostSeason)),
            SeasonId::from((1997, RegularSeason)),
            SeasonId::from((1997, PostSeason)),
            SeasonId::from((1998, RegularSeason)),
            SeasonId::from((1998, PostSeason)),
            SeasonId::from((1999, RegularSeason)),
            SeasonId::from((1999, PostSeason)),
            SeasonId::from((2000, RegularSeason)),
            SeasonId::from((2000, PostSeason)),
            SeasonId::from((2001, RegularSeason)),
            SeasonId::from((2001, PostSeason)),
            SeasonId::from((2002, RegularSeason)),
            SeasonId::from((2002, PostSeason)),
            SeasonId::from((2003, PreSeason)),
            SeasonId::from((2003, RegularSeason)),
            SeasonId::from((2003, PostSeason)),
            SeasonId::from((2004, PreSeason)),
            SeasonId::from((2004, RegularSeason)),
            SeasonId::from((2004, PostSeason)),
            SeasonId::from((2005, PreSeason)),
            SeasonId::from((2005, RegularSeason)),
            SeasonId::from((2005, PostSeason)),
            SeasonId::from((2006, PreSeason)),
            SeasonId::from((2006, RegularSeason)),
            SeasonId::from((2006, PostSeason)),
            SeasonId::from((2007, PreSeason)),
            SeasonId::from((2007, RegularSeason)),
            SeasonId::from((2007, PostSeason)),
            SeasonId::from((2008, PreSeason)),
            SeasonId::from((2008, RegularSeason)),
            SeasonId::from((2008, PostSeason)),
            SeasonId::from((2009, PreSeason)),
            SeasonId::from((2009, RegularSeason)),
            SeasonId::from((2009, PostSeason)),
            SeasonId::from((2010, PreSeason)),
            SeasonId::from((2010, RegularSeason)),
            SeasonId::from((2010, PostSeason)),
            SeasonId::from((2011, PreSeason)),
            SeasonId::from((2011, RegularSeason)),
            SeasonId::from((2011, PostSeason)),
            SeasonId::from((2012, PreSeason)),
            SeasonId::from((2012, RegularSeason)),
            SeasonId::from((2012, PostSeason)),
            SeasonId::from((2013, PreSeason)),
            SeasonId::from((2013, RegularSeason)),
            SeasonId::from((2013, PostSeason)),
            SeasonId::from((2014, PreSeason)),
            SeasonId::from((2014, RegularSeason)),
            SeasonId::from((2014, PostSeason)),
            SeasonId::from((2015, PreSeason)),
            SeasonId::from((2015, RegularSeason)),
            SeasonId::from((2015, PostSeason)),
            SeasonId::from((2016, PreSeason)),
            SeasonId::from((2016, RegularSeason)),
            SeasonId::from((2016, PostSeason)),
            SeasonId::from((2017, PreSeason)),
            SeasonId::from((2017, RegularSeason)),
            SeasonId::from((2017, PostSeason)),
            SeasonId::from((2018, PreSeason)),
            SeasonId::from((2018, RegularSeason)),
            SeasonId::from((2018, PostSeason)),
            SeasonId::from((2019, PreSeason)),
            SeasonId::from((2019, RegularSeason)),
            SeasonId::from((2019, PlayIn)),
            SeasonId::from((2019, PostSeason)),
            SeasonId::from((2020, PreSeason)),
            SeasonId::from((2020, RegularSeason)),
            SeasonId::from((2020, PlayIn)),
            SeasonId::from((2020, PostSeason)),
            SeasonId::from((2021, PreSeason)),
            SeasonId::from((2021, RegularSeason)),
            SeasonId::from((2021, PlayIn)),
            SeasonId::from((2021, PostSeason)),
            SeasonId::from((2022, PreSeason)),
            SeasonId::from((2022, RegularSeason)),
            SeasonId::from((2022, PlayIn)),
            SeasonId::from((2022, PostSeason)),
            SeasonId::from((2023, PreSeason)),
            SeasonId::from((2023, RegularSeason)),
            SeasonId::from((2023, PlayIn)),
            SeasonId::from((2023, PostSeason)),
            SeasonId::from((2024, PreSeason)),
            SeasonId::from((2024, RegularSeason)),
            SeasonId::from((2024, PlayIn)),
            SeasonId::from((2024, PostSeason)),
            SeasonId::from((2025, PreSeason)),
            SeasonId::from((2025, RegularSeason)),
            SeasonId::from((2025, PlayIn)),
            SeasonId::from((2025, PostSeason)),
        ];

        let actual = nba_lifespan_at(NaiveDate::from_ymd_opt(2026, 8, 14).unwrap());

        assert_eq!(expected, actual);
    }
}
