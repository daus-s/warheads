use crate::format::path_manager::nba_timeline_path;

use crate::stats::gamecard::GameCard;

use crate::types::{GameDate, SeasonId};

use std::io;
use std::path::PathBuf;

use thiserror::Error;

fn load_date_gamecards(file: PathBuf) -> Option<Vec<GameCard>> {
    let contents = std::fs::read_to_string(file).ok()?;

    let gamecards = serde_json::from_str(&contents).ok()?;

    Some(gamecards)
}

pub fn load_era_gamecards(era: SeasonId) -> Result<Vec<GameCard>, LoadGameCardError> {
    let dir_path = nba_timeline_path(era);

    let files = std::fs::read_dir(dir_path).map_err(|e| LoadGameCardError::DirError(e))?;

    let mut era_events = Vec::new();

    for entry in files {
        let entry = entry.map_err(|e| LoadGameCardError::FileError(e))?;
        let path = entry.path();

        let data: Vec<GameCard> = load_date_gamecards(path).unwrap_or_default();
        // Process each file
        era_events.extend(data);
    }

    Ok(era_events)
}

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
    use once_cell::sync::Lazy;

    use crate::constants::paths::data;

    use super::*;

    #[test]
    fn test_load_era_games() {
        // Test loading games for a specific era
        let events = load_era_gamecards(SeasonId::from(22025))
            .expect("should be able to read test timeline files");

        assert!(events.len() > 0);
    }

    #[test]
    fn test_load_date_games() {
        static DATA: Lazy<String> = Lazy::new(data);

        // Test loading games for a specific date
        let events = load_date_gamecards(PathBuf::from(format!(
            "{}/nba/timeline/2025_26/RegularSeason/2025_10_21.game",
            *DATA
        )))
        .expect("should be able to read test timeline files");

        assert!(events.len() == 2);

        for event in events {
            println!("{}", event);
        }
    }

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
