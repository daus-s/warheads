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

    pub fn load_year(&mut self, era: SeasonId) -> Result<(), Box<dyn Error>> {
        self.era = Some(era);

        let season = read_nba_season(self.era.unwrap())?;

        self.games = Some(season);

        Ok(())
    }

    pub fn next_era(&mut self) -> Result<(), Box<dyn Error>> {
        let current_era = self.era.unwrap();
        let next_era = current_era.next();

        self.load_year(next_era)
    }
}
