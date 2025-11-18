// an efficient way to query through historical games
use crate::stats::game_obj::GameObject;

use crate::storage::read_disk::read_nba_season;

use crate::types::{GameId, PlayerId, SeasonId, TeamId};

use std::collections::HashSet;

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
        if self.era.is_some() && era == self.era.unwrap() {
            return Ok(());
        }

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

    pub fn last_n_games(&self, n: usize, team: TeamId, game: GameId) -> Option<Vec<GameObject>> {
        if !self.is_initialized() {
            return None;
        }

        let games = self.games.as_ref().unwrap().clone();

        let starting_index = games.iter().find_map(|x| {
            if x.game_id == game {
                Some(x.clone())
            } else {
                None
            }
        });

        Some(vec![])
    }

    pub fn get_expected_roster(&self, home: TeamId, game: GameId) -> Option<Vec<PlayerId>> {
        if !self.is_initialized() {
            return None;
        }

        let mut players = HashSet::new();

        let recent_games = self.last_n_games(5, home, game)?;

        players.into_iter().collect()
    }

    fn is_initialized(&self) -> bool {
        self.games.is_some() && self.era.is_some()
    }
}
