use std::error::Error;
use std::path::PathBuf;

use crate::format::path_manager::nba_timeline_path;
use crate::stats::gamecard::GameCard;
use crate::types::{GameDate, SeasonId};

fn load_date_gamecards(file: PathBuf) -> Option<Vec<GameCard>> {
    let contents = std::fs::read_to_string(file).ok()?;

    let gamecards = serde_json::from_str(&contents).ok()?;

    Some(gamecards)
}

fn load_era_gamecards(era: SeasonId) -> Result<Vec<GameCard>, Box<dyn std::error::Error>> {
    let dir_path = nba_timeline_path(era);

    let files = std::fs::read_dir(dir_path)?;

    let mut era_events = Vec::new();

    for entry in files {
        let entry = entry?;
        let path = entry.path();

        let data: Vec<GameCard> = load_date_gamecards(path).unwrap_or_default();
        // Process each file
        era_events.extend(data);
    }

    Ok(era_events)
}

#[cfg(test)]
mod test_timeline_manager {
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
}
