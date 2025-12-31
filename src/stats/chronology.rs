use crate::dapi::season_manager::nba_lifespan_period;

use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;
use crate::stats::record::Record;

use crate::storage::read_disk::{read_nba_season, NBAReadError};

use crate::types::{GameId, PlayerId, SeasonId, TeamId};

use std::cmp::max;
use std::collections::HashMap;

use thiserror::Error;

/// an efficient way to query through historical games
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

    pub fn from_era(era: SeasonId) -> Self {
        let mut timeline = Chronology::new();

        if let Err(_) = timeline.load_era(era) {
            Chronology::new()
        } else {
            timeline
        }
    }

    pub fn load_era(&mut self, era: SeasonId) -> Result<(), ChronologyError> {
        if self.era.is_some() && era == self.era.unwrap() {
            return Ok(());
        }

        let season = read_nba_season(era).map_err(|e| ChronologyError::ReadSeasonError(e))?;

        self.era = Some(era);
        self.games = Some(season);

        Ok(())
    }

    pub fn next(&mut self) -> Result<(), ChronologyError> {
        let current_era = self.era.unwrap();
        let next_era = current_era.next();

        self.load_era(next_era)
    }

    pub fn prev(&mut self) -> Result<(), ChronologyError> {
        let current_era = self.era.unwrap();
        let previous_era = current_era.prev();

        self.load_era(previous_era)
    }

    pub fn as_training_data(mut self) -> Result<Vec<(GameCard, GameObject)>, ChronologyError> {
        let mut games = Vec::new();

        for era in nba_lifespan_period() {
            if let Err(e) = self.load_era(era) {
                return Err(e);
            }

            games.extend(
                self.games
                    .clone()
                    .ok_or(ChronologyError::ChronologyMemoryError)?,
            )
        }

        let mut pairs: Vec<(GameCard, GameObject)> =
            games.into_iter().map(|game| (game.card(), game)).collect();

        for (card, game) in &mut pairs {
            card.add_away_ratings(self.get_expected_roster(game.away_team_id(), game.game_id()));
            card.add_home_ratings(self.get_expected_roster(game.home_team_id(), game.game_id()));
        }

        Ok(pairs)
    }

    fn n_most_recent_games(&self, n: usize, team_id: TeamId, game_id: GameId) -> Vec<GameObject> {
        if !self.is_initialized() {
            panic!("💀 tried to run most_recent_games on an uninitialized Chronology object.")
        }

        let games_for_team = self
            .games
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|game| {
                if game.away_team_id() == team_id || game.home_team_id() == team_id {
                    Some(game.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<GameObject>>();

        let index = games_for_team
            .iter()
            .position(|g| g.game_id() == game_id)
            .unwrap_or(games_for_team.len());

        let mut last_n_games = Vec::new();

        for i in max(index as i64 - n as i64, 0) as usize..index {
            last_n_games.push(games_for_team[i].clone());
        }

        last_n_games
    }

    //todo: improve this with win sharing function as elo parameter
    /// calculates the expected roster for a given team and game based on previous N games
    ///
    /// # function
    ///
    /// N = 5
    pub fn get_expected_roster(&self, team_id: TeamId, game_id: GameId) -> Vec<PlayerId> {
        if !self.is_initialized() {
            panic!("💀 tried to run get_expected_roster on an uninitialized Chronology object.")
        }

        //map players and appearances
        let mut players: HashMap<PlayerId, u8> = HashMap::new();

        let recent_games = self.n_most_recent_games(5, team_id, game_id);

        let mut max_roster_size = 0;

        for game in recent_games {
            let team = game.team(team_id);

            for player in team.roster() {
                players
                    .entry(player)
                    .and_modify(|games_played| *games_played += 1)
                    .or_insert(1);
            }

            max_roster_size = max(max_roster_size, team.roster().len())
        }

        let mut playing_frequencies = players.into_iter().collect::<Vec<(PlayerId, u8)>>();

        playing_frequencies.sort_by_key(|(p, f)| (-1 * (*f as i16), p.0)); //sort on p.0 for stability

        playing_frequencies
            .iter()
            .take(max_roster_size)
            .map(|(p, _f)| *p)
            .collect()
    }

    fn is_initialized(&self) -> bool {
        self.games.is_some() && self.era.is_some()
    }

    pub fn calculate_record(&self, team_id: TeamId) -> Record {
        if !self.is_initialized() {
            return Record { wins: 0, losses: 0 };
        }

        let mut wins = 0;
        let mut losses = 0;

        self.games
            .as_ref()
            .unwrap()
            .iter()
            .filter(|game| game.had_participant(team_id))
            .for_each(|game| {
                if game.winner() == team_id {
                    wins += 1
                } else {
                    losses += 1
                }
            });

        Record { wins, losses }
    }

    pub fn games(&self) -> &Option<Vec<GameObject>> {
        &self.games
    }
}

#[derive(Debug, Error)]
pub enum ChronologyError {
    #[error("❌ failed to read season data from storage")]
    ReadSeasonError(NBAReadError),
    #[error(
        "❌ chronology implementation error \n❌ couldn't access games after a successful load"
    )]
    ChronologyMemoryError,
}

#[cfg(test)]
mod test_chronology {
    use super::*;

    use crate::stats::season_period::SeasonPeriod::*;

    #[test]
    fn test_last_n_games_early_season() {
        let chronology = Chronology::from_era(SeasonId::from((2024, RegularSeason)));
        let expected: Vec<GameId> =
            vec![GameId(0022400062), GameId(0022400085), GameId(0022400096)];

        let game = GameId(0022400111);

        let team = TeamId(1610612747);

        let actual = chronology
            .n_most_recent_games(5, team, game)
            .iter()
            .map(|x| x.game_id())
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_last_n_games() {
        let chronology = Chronology::from_era(SeasonId::from((2024, RegularSeason)));

        let expected: Vec<GameId> = vec![
            GameId(0022400062),
            GameId(0022400085),
            GameId(0022400096),
            GameId(0022400111),
            GameId(0022400118),
            GameId(0022400137),
            GameId(0022400156),
            GameId(0022400174),
            GameId(0022400195),
            GameId(0022400211),
            GameId(0022400225),
            GameId(0022400015),
            GameId(0022400231),
            GameId(0022400026),
            GameId(0022400263),
            GameId(0022400270),
            GameId(0022400039),
            GameId(0022400296),
            GameId(0022400048),
            GameId(0022400311),
            GameId(0022400318),
            GameId(0022400321),
            GameId(0022400334),
            GameId(0022400358),
            GameId(0022401210),
            GameId(0022401220),
            GameId(0022400372),
            GameId(0022400376),
            GameId(0022400404),
            GameId(0022400408),
            GameId(0022400435),
            GameId(0022400454),
            GameId(0022400468),
            GameId(0022400477),
            GameId(0022400491),
            GameId(0022400504),
            GameId(0022400552),
            GameId(0022400570),
            GameId(0022400585),
            GameId(0022400596),
            GameId(0022400610),
            GameId(0022400629),
            GameId(0022400644),
            GameId(0022400648),
            GameId(0022400660),
            GameId(0022400674),
            GameId(0022400692),
            GameId(0022400715),
            GameId(0022400731),
            GameId(0022400742),
            GameId(0022400768),
            GameId(0022400781),
            GameId(0022400524),
            GameId(0022400796),
            GameId(0022400808),
            GameId(0022400835),
            GameId(0022400849),
            GameId(0022400859),
            GameId(0022400874),
            GameId(0022400890),
            GameId(0022400903),
            GameId(0022400918),
            GameId(0022400930),
            GameId(0022400955),
            GameId(0022400965),
            GameId(0022400977),
            GameId(0022400537),
            GameId(0022401006),
            GameId(0022400996),
            GameId(0022401028),
            GameId(0022401038),
            GameId(0022401055),
            GameId(0022401063),
            GameId(0022401078),
            GameId(0022401096),
            GameId(0022401117),
            GameId(0022401126),
            GameId(0022401135),
            GameId(0022401153),
            GameId(0022401159),
            GameId(0022401185),
        ];

        let actual = chronology
            .n_most_recent_games(81, TeamId(1610612747), GameId::from(0022401199))
            .iter()
            .map(|x| {
                // i am going to assume the rest of the data is right if the gameid's are right. other tests should take care of that
                x.game_id()
            })
            .collect::<Vec<GameId>>();

        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 81);
    }

    #[test]
    fn test_get_expected_roster() {
        let chronology = Chronology::from_era(SeasonId::from((2024, RegularSeason)));

        let actual = chronology.get_expected_roster(TeamId(1610612747), GameId::from(0022401199));

        let expected: Vec<PlayerId> = vec![
            PlayerId(2544),
            PlayerId(1627827),
            PlayerId(1629020),
            PlayerId(1629029),
            PlayerId(1629216),
            PlayerId(1629637),
            PlayerId(1630559),
            PlayerId(1630692),
            PlayerId(1642261),
            PlayerId(1641998),
            PlayerId(1629003),
            PlayerId(1629060),
            PlayerId(1642355),
        ];

        assert_eq!(actual, expected);
    }
}
