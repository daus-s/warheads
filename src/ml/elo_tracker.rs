use std::collections::HashMap;

use crate::constants::paths::data;
use crate::dapi::load::load_nba_season_games_objects;
use crate::dapi::season_manager::nba_lifespan;
use crate::ml::elo::Elo;
use crate::types::{GameId, PlayerId};
use csv::Writer;
use once_cell::sync::Lazy;

pub struct EloTracker {
    historical_ratings: Vec<Elo>,
    current_ratings: HashMap<PlayerId, i64>,
}

impl EloTracker {
    pub fn process_elo(&mut self) {
        // todo: assign elo values to players on a game by game basis

        // todo: load season by season, don't nuke the memory will all the history
        for season in nba_lifespan() {
            let season_games = load_nba_season_games_objects(season);

            for game in season_games {
                let mut home_rating = game.home.get_team_rating(&mut self.current_ratings);
                let mut away_rating = game.away.get_team_rating(&mut self.current_ratings);

                let delta = home_rating - away_rating;

                todo!("Implement Elo calculation for the game")
            }
        }
    }

    //todo: add correct formating s.t. every row is the same number of characters for easy indexing
    pub fn save(&self) -> Result<(), String> {
        let model_name = "elo";

        let filename = Self::save_path(&format!("{model_name}.csv"));

        let mut writer = Writer::from_path(&filename)
            .map_err(|e| format!("❌ failed to open a writer for {filename}: {e}"))?;

        for elo in &self.historical_ratings {
            let Elo {
                game_id,
                player_id,
                rating,
            } = *elo;

            //todo: implement this to have each row have a same
            match writer.serialize(&[game_id.0 as i64, player_id.0 as i64, rating]) {
                Ok(_) => {
                    eprintln!(
                        "✅ successfully wrote record for {player_id} in {game_id}: {}{rating}",
                        match rating < 0 {
                            true => "",
                            false => "+",
                        }
                    );
                }
                Err(e) => {
                    return Err(format!(
                        "❌ failed to write record for {player_id} in {game_id}: {e}"
                    ));
                }
            };
        }

        Ok(())
    }

    fn save_path(filename: &str) -> String {
        static DATA: Lazy<String> = Lazy::new(data);

        format!("{}/nba/elo/{}", *DATA, filename)
    }
}
