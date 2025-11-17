// an efficient way to query through historical games
use crate::stats::game_obj::GameObject;

use crate::storage::read_disk::read_nba_season;

use crate::types::SeasonId;

use std::error::Error;

pub struct Chronology {
    games: Option<Vec<GameObject>>,
    era: Option<SeasonId>,
}

impl Chronology {
    pub fn new() -> Self {
        Self {
            games: None,
            era: None,
        }
    }

    pub fn with_era(&self, era: SeasonId) -> Self {
        let mut timeline = Chronology::new();

        if let Err(_) = timeline.load_year(era) {
            Chronology::new()
        } else {
            timeline
        }
    }

    pub fn load_year(&mut self, era: SeasonId) -> Result<(), Box<dyn Error>> {
        if let Err(e) = read_nba_season(era) {
            return Err(Box::new(e));
        } else if let Ok(season) = read_nba_season(era) {
            self.era = Some(era);
            self.games = Some(season);
        }
        Ok(())
    }

    pub fn next(&mut self) -> Result<(), Box<dyn Error>> {
        let current_era = self.era.unwrap();
        let next_era = current_era.next();

        self.load_year(next_era)
    }

    pub fn prev(&mut self) -> Result<(), Box<dyn Error>> {
        let current_era = self.era.unwrap();
        let previous_era = current_era.prev();

        self.load_year(previous_era)
    }
}
