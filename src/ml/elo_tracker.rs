use thiserror::Error;

use crate::dapi::season_manager::nba_lifespan_period;

use crate::format::path_manager::{records_path, results_path};

use crate::ml::cdf;
use crate::ml::elo::{self, Elo};
use crate::ml::elo_writer::{EloWriter, EloWriterError};
use crate::ml::log_loss::LogLossTracker;
use crate::ml::measurement::Measurement;
use crate::ml::model::Model;

use crate::proc::prophet::write_predictions;

use crate::stats::chronology::Chronology;
use crate::stats::game_obj::GameObject;

use crate::stats::gamecard::GameCard;
use crate::stats::prediction::Prediction;
use crate::stats::visiting::Visiting;
use crate::storage::read_disk::{read_nba_season, NBAReadError};

use crate::types::PlayerId;

use std::collections::HashMap;
use std::{fs, io};

const ELO_VERSION: &str = "elo v1";

pub struct EloTracker {
    historical_ratings: Vec<Elo>,
    current_ratings: HashMap<PlayerId, i64>,
    log_loss: LogLossTracker,
}

impl EloTracker {
    fn new() -> Self {
        Self {
            historical_ratings: Vec::new(),
            current_ratings: HashMap::new(),
            log_loss: LogLossTracker::model(ELO_VERSION.to_owned()),
        }
    }

    pub fn train() -> Result<Self, EloTrackerError> {
        let mut tracker = EloTracker::new();

        tracker.process_elo()?;

        tracker.save()?;

        Ok(tracker)
    }

    fn process_elo(&mut self) -> Result<(), EloTrackerError> {
        // todo: assign elo values to players on a game by game basis
        for period in nba_lifespan_period() {
            let mut games = read_nba_season(period).map_err(|e| EloTrackerError::ReaderError(e))?;

            if !games.is_sorted_by_key(|game| game.game_date.0) {
                games.sort_by_key(|game| game.game_date.0);
            }

            let mut predictions = Vec::new();

            for game in games {
                self.update_ratings(&game);

                let prediction = self.predict(&game);

                predictions.push(Prediction::from(game.card(), prediction));
            }

            write_predictions(self, predictions)
                .map_err(|e| EloTrackerError::WritePredictionError(e))?;
        }

        Ok(())
    }

    //todo: implement a rating share function as a parameter
    fn update_ratings(&mut self, game: &GameObject) {
        let home_rating = self.normalized_ratings_from_iter(game.home().roster().into_iter());
        let away_rating = self.normalized_ratings_from_iter(game.away().roster().into_iter());

        let delta = home_rating - away_rating;

        //R'=R+K∙(S-E) where s is the score and e is the expected (1 for win, 0 for loss - win probability)

        let mut home_step = (elo::K as f64 * (1.0 - cdf::prob(delta))) as i64;
        let mut away_step = (elo::K as f64 * (1.0 - cdf::prob(-1f64 * delta))) as i64;

        //this is the winners step, the losers step is -step
        if game.winner() == game.home_team_id() {
            away_step = -1 * (away_step as i64);
        } else if game.winner() == game.away_team_id() {
            home_step = -1 * (home_step as i64);
        } else {
            panic!("💀 Game must have a winner that was a participant. Somehow passed the win/loss check in GameObject::try_create");
        }

        for player in game.home_roster() {
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

        self.track_log_loss(game, delta);
    }

    fn track_log_loss(&mut self, game: &GameObject, delta: f64) {
        let p = cdf::prob(delta);

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

    pub fn save(&self) -> Result<(), EloTrackerError> {
        let records_filename = records_path(self);

        let mut writer = EloWriter::new(records_filename)
            .map_err(|e| EloTrackerError::WriterCreationError(e))?;

        for record in &self.historical_ratings {
            writer
                .serialize_elo(&record)
                .map_err(|e| EloTrackerError::EloWriteError(e))?;
        }

        let results_filename = results_path(self);

        let _ = fs::write(results_filename, format!("{}", self.log_loss));

        Ok(())
    }

    //todo: implement a less safe version of this function that accepts a immutable reference to the tracker
    //      panic the program if called incorrectly.
    //
    // mayeb dont add this and suffer a teensy performance hit
    pub fn normalized_ratings_from_iter(&mut self, iter: impl Iterator<Item = PlayerId>) -> f64 {
        let (count, sum) = iter.fold((0usize, 0i64), |acc, id| {
            (
                acc.0 + 1,
                acc.1
                    + *self
                        .current_ratings
                        .entry(id)
                        .or_insert(elo::INITIAL_RATING),
            )
        });

        sum as f64 / count as f64
    }

    pub fn predict_cards(&mut self, gamecards: Vec<GameCard>) -> Vec<Prediction> {
        let mut chronology = Chronology::new();
        let mut predictions = Vec::new();

        for mut game in gamecards.into_iter() {
            chronology
                .load_year(game.season())
                .expect("Failed to load year from storage");

            game.add_record(
                Visiting::Home,
                chronology.calculate_record(game.home().team_id()),
            );
            game.add_record(
                Visiting::Away,
                chronology.calculate_record(game.away().team_id()),
            );

            let home_roster = chronology.get_expected_roster(game.home().team_id(), game.game_id());
            let away_roster = chronology.get_expected_roster(game.away().team_id(), game.game_id());

            let home_rating = self.normalized_ratings_from_iter(home_roster.into_iter());
            let away_rating = self.normalized_ratings_from_iter(away_roster.into_iter());

            //that home wins
            let prob = cdf::prob(home_rating - away_rating);

            let prediction = Prediction::from(game, prob);

            predictions.push(prediction);
        }

        predictions
    }
}

#[derive(Debug, Error)]
pub enum EloTrackerError {
    #[error("❌ {0}\n❌ failed to load an elo writer from file.")]
    WriterCreationError(EloWriterError),
    #[error("❌ {0}\n❌ failed to load an nba games from file.")]
    ReaderError(NBAReadError),
    #[error("❌ {0}\n❌ error writing predictions to file. ")]
    WritePredictionError(io::Error),
    #[error("❌ {0}\n❌ error writing elo records to file. ")]
    EloWriteError(EloWriterError),
}

impl Model for EloTracker {
    fn predict(&mut self, obj: &GameObject) -> f64 {
        let home = obj.home();
        let away = obj.away();

        let home_rating = self.normalized_ratings_from_iter(home.roster().into_iter());
        let away_rating = self.normalized_ratings_from_iter(away.roster().into_iter());

        let diff = home_rating - away_rating;

        cdf::prob(diff)
    }

    fn model_name(&self) -> String {
        ELO_VERSION.to_string()
    }
}
