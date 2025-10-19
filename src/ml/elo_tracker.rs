use crate::constants::paths::data;

use crate::dapi::season_manager::nba_lifespan;

use crate::ml::cdf::prob;
use crate::ml::elo::{self, Elo};

use crate::stats::game_obj::GameObject;
use crate::stats::season_period::minimum_spanning_era;

use crate::storage::read_disk::read_nba_season;

use crate::types::PlayerId;

use csv::Writer;

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct EloTracker {
    historical_ratings: Vec<Elo>,
    current_ratings: HashMap<PlayerId, i64>,
}

impl EloTracker {
    pub fn new() -> Self {
        Self {
            historical_ratings: Vec::new(),
            current_ratings: HashMap::new(),
        }
    }

    pub fn process_elo(&mut self) {
        // todo: assign elo values to players on a game by game basis
        for year in nba_lifespan() {
            let mut season_games = Vec::new();
            for period in minimum_spanning_era(year) {
                match read_nba_season(period) {
                    Ok(games) => {
                        season_games.extend(games);
                    }
                    Err(e) => {
                        eprintln!("Failed to read season {period}: {e}");
                    }
                }
            }

            for game in season_games {
                let home_rating = game
                    .home
                    .get_normalized_team_rating(&mut self.current_ratings);
                let away_rating = game
                    .away
                    .get_normalized_team_rating(&mut self.current_ratings);

                let delta = home_rating - away_rating;

                //R'=R+K∙(S-E) where s is the score and e is the expected (1 for win - win prob)

                self.update_ratings(&game, delta);
            }
        }
    }

    //todo: implement a rating share function as a parameter
    fn update_ratings(&mut self, game: &GameObject, delta: f64) {
        let mut step_home = (elo::K as f64 * (1.0 - prob(delta))) as i64; //this is the winners step, the losers step is -step
        let mut step_away = (elo::K as f64 * (1.0 - prob(-1f64 * delta))) as i64;

        if game.winner() == game.home.team_id {
            step_away = -1 * (step_away as i64);
        } else if game.winner() == game.away.team_id {
            step_home = -1 * (step_home as i64);
        } else {
            panic!("💀 Game must have a winner. somehow passed the win/loss check in GameObject::try_create");
        }

        for player in game.home.roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += step_home)
                .or_insert(elo::INITIAL_RATING);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: game.game_id,
                rating: self.current_ratings[&id],
            });
        }
        for player in game.away.roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += step_away)
                .or_insert(elo::INITIAL_RATING);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: game.game_id,
                rating: self.current_ratings[&id],
            });
        }
    }

    // SERIALIZATION

    //todo: add correct formating s.t. every row is the same number of characters for easy indexing
    pub fn save(&self) -> Result<(), String> {
        let model_name = "elo";

        let filename = Self::save_path(&format!("{model_name}.csv"));

        let mut writer = match Writer::from_path(&filename) {
            Ok(writer) => writer,
            Err(e) => {
                eprintln!("❌ failed to open a writer for {}: {e}", filename.display());

                fs::create_dir_all(filename.parent().unwrap()).map_err(|e| {
                    format!(
                        "❌ failed to create directory for {}: {e}",
                        filename.display()
                    )
                })?;

                Writer::from_path(&filename).map_err(|e| {
                    format!("❌ failed to open a writer for {}: {e}", filename.display())
                })?
            }
        };

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

    fn save_path(filename: &str) -> PathBuf {
        static DATA: Lazy<String> = Lazy::new(data);

        PathBuf::from(format!("{}/nba/elo/{}", *DATA, filename))
    }
}
