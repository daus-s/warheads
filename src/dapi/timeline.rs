use thiserror::Error;

use crate::dapi::season_manager::nba_lifespan_period;

use crate::stats::gamecard::GameCard;
use crate::stats::visiting::Visiting;

use crate::storage::read_disk::{read_nba_season, NBAReadError};

use crate::types::GameDate;

use std::collections::HashMap;

/// sequence_nba creates an in memory timeline of all games in NBA history based on already saved games
pub fn nba_timeline() -> Result<HashMap<GameDate, Vec<GameCard>>, NBATimelineError> {
    // create chronological timeline of all game events
    let mut dates = HashMap::<GameDate, Vec<GameCard>>::new();
    for era in nba_lifespan_period() {
        let mut games = read_nba_season(era).map_err(|e| NBATimelineError::SeasonReadError(e))?;

        games.sort_by_key(|game| game.game_id);

        for game in games.iter() {
            let date = game.game_date;
            dates.entry(date).or_insert_with(Vec::new).push(game.card());
        }

        for game in games {
            let date = game.game_date;

            let winner = game.winning_side();

            let cards = dates.get_mut(&date).unwrap(); //it must be in here if we added it. i think u can prove this wont unwrap

            let gc = cards
                .iter_mut()
                .find(|card| card.game_id() == game.game_id())
                .unwrap(); //same protective level for this unwrap as before

            match winner {
                Visiting::Home => {
                    gc.mut_home().add_win();
                    gc.mut_away().add_loss();
                }
                Visiting::Away => {
                    gc.mut_away().add_win();
                    gc.mut_home().add_loss();
                }
            }
        }
    }

    Ok(dates)
}

#[derive(Error, Debug)]
pub enum NBATimelineError {
    #[error("❌ {0}\n❌ failed to read a season while generating a timeline.")]
    SeasonReadError(NBAReadError),
}
