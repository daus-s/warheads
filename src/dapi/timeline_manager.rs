use crate::types::GameDate;

use std::io;

use thiserror::Error;

pub fn get_next_n_dates(date: GameDate, n: usize) -> Vec<GameDate> {
    let mut dates = Vec::new();
    let mut current_date = date;

    for _ in 0..n {
        dates.push(current_date);
        current_date = current_date.next();
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
        let date = GameDate(NaiveDate::from_ymd_opt(2025, 10, 21).unwrap());
        let next_dates = get_next_n_dates(date, 3);

        assert_eq!(next_dates.len(), 3);

        assert_eq!(
            next_dates[0],
            GameDate(NaiveDate::from_ymd_opt(2025, 10, 21).unwrap())
        );
        assert_eq!(
            next_dates[1],
            GameDate(NaiveDate::from_ymd_opt(2025, 10, 22).unwrap())
        );
        assert_eq!(
            next_dates[2],
            GameDate(NaiveDate::from_ymd_opt(2025, 10, 23).unwrap())
        );
    }
}
