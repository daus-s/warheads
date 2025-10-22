use crate::constants::paths::data;

use crate::dapi::season_manager::nba_lifespan_period;

use crate::ml::cdf::prob;
use crate::ml::elo::{self, Elo};

use crate::ml::elo_writer::EloWriter;
use crate::ml::log_loss::LogLossTracker;
use crate::ml::measurement::Measurement;
use crate::stats::game_obj::GameObject;

use crate::storage::read_disk::read_nba_season;

use crate::types::PlayerId;

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct EloTracker {
    historical_ratings: Vec<Elo>,
    current_ratings: HashMap<PlayerId, i64>,
    log_loss: LogLossTracker,
}

impl EloTracker {
    pub fn new() -> Self {
        Self {
            historical_ratings: Vec::new(),
            current_ratings: HashMap::new(),
            log_loss: LogLossTracker::new(),
        }
    }

    pub fn process_elo(&mut self) -> Result<(), ()> {
        // todo: assign elo values to players on a game by game basis
        for period in nba_lifespan_period() {
            let mut games = read_nba_season(period).map_err(|e| println!("NBAReadError: {}", e))?;

            if !games.is_sorted_by_key(|game| game.game_date.0) {
                games.sort_by_key(|game| game.game_date.0);
            }

            for game in games {
                let home_rating = game
                    .home()
                    .get_normalized_team_rating(&mut self.current_ratings);
                let away_rating = game
                    .away()
                    .get_normalized_team_rating(&mut self.current_ratings);

                let delta = home_rating - away_rating;

                //R'=R+K∙(S-E) where s is the score and e is the expected (1 for win - win prob)

                self.update_ratings(&game, delta);
                self.track_log_loss(&game, delta);
            }
        }

        Ok(())
    }

    //todo: implement a rating share function as a parameter
    fn update_ratings(&mut self, game: &GameObject, delta: f64) {
        let mut home_step = (elo::K as f64 * (1.0 - prob(delta))) as i64; //this is the winners step, the losers step is -step
        let mut away_step = (elo::K as f64 * (1.0 - prob(-1f64 * delta))) as i64;

        if game.winner() == game.home_team_id() {
            away_step = -1 * (away_step as i64);
        } else if game.winner() == game.away_team_id() {
            home_step = -1 * (home_step as i64);
        } else {
            panic!("💀 Game must have a winner that was a participant. Somehow passed the win/loss check in GameObject::try_create");
        }

        for player in game.away_roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += home_step)
                .or_insert(elo::INITIAL_RATING);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: game.game_id,
                rating: self.current_ratings[&id],
            });
        }
        for player in game.away_roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += away_step)
                .or_insert(elo::INITIAL_RATING);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: game.game_id,
                rating: self.current_ratings[&id],
            });
        }
    }

    fn track_log_loss(&mut self, game: &GameObject, delta: f64) {
        let p = prob(delta);

        let a = if game.winner() == game.home_team_id() {
            1
        } else if game.winner() == game.away_team_id() {
            0
        } else {
            panic!("💀  game doesnt have a winner that participated in the game.");
        };

        let m = Measurement::new(a, p);

        self.log_loss.add_measurement(m);
    }

    // SERIALIZATION

    //todo: add correct formating s.t. every row is the same number of characters for easy indexing
    pub fn save(&self) -> Result<(), String> {
        let model_name = self.get_model_name();

        let records_filename = Self::records_path(&format!("{model_name}.csv"));

        let mut writer = EloWriter::new(records_filename).expect("💀 failed to create EloWriter");

        for record in &self.historical_ratings {
            let _ = writer.serialize_elo(&record);
        }

        let results_filename = Self::results_path(&format!("{model_name}_results"));

        let _ = fs::write(results_filename, format!("{}", self.log_loss));

        Ok(())
    }

    fn records_path(filename: &str) -> PathBuf {
        static DATA: Lazy<String> = Lazy::new(data);

        PathBuf::from(format!("{}/nba/elo/records/{}", *DATA, filename))
    }

    /// results_path generates the path to where the model accuracy is stored.
    fn results_path(filename: &str) -> PathBuf {
        static DATA: Lazy<String> = Lazy::new(data);

        PathBuf::from(format!("{}/nba/elo/results/{}", *DATA, filename))
    }

    //todo: implement get_model_name for custom models
    fn get_model_name(&self) -> String {
        "elo".to_string()
    }
}
