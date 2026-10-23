use std::io;

use chrono::{Local, NaiveDate};
use thiserror::Error;

pub fn is_today(date: NaiveDate) -> bool {
    let today = Local::now().date_naive();
    date == today

}

pub fn get_next_n_dates(date: NaiveDate, n: usize) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut current_date = date;

    for _ in 0..n {
        dates.push(current_date);
        current_date = current_date.succ_opt().unwrap();
    }

    dates
}

#[derive(Error, Debug)]
pub enum LoadGameCardError {
    #[error("❌ {0}\n❌ Failed to load game card data from directory. ")]
    DirError(io::Error),
    #[error("❌ {0}\n❌ Failed to load game card data from file.")]
    FileError(io::Error),
}

#[cfg(test)]
mod test_timeline_manager {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_get_next_n_dates() {
        let date = NaiveDate::from_ymd_opt(2025, 10, 21).unwrap();
        let next_dates = get_next_n_dates(date, 3);

        assert_eq!(next_dates.len(), 3);

        assert_eq!(
            next_dates[0],
            NaiveDate::from_ymd_opt(2025, 10, 21).unwrap()
        );
        assert_eq!(
            next_dates[1],
            NaiveDate::from_ymd_opt(2025, 10, 22).unwrap()
        );
        assert_eq!(
            next_dates[2],
            NaiveDate::from_ymd_opt(2025, 10, 23).unwrap()
        );
    }
}
