use thiserror::Error;

use crate::format::path_manager::{records_path, results_path};

use crate::ml::cdf;
use crate::ml::elo::Elo;
use crate::ml::elo_params::EloParams;
use crate::ml::elo_writer::{EloWriter, EloWriterError};
use crate::ml::log_loss::LogLossTracker;
use crate::ml::measurement::Measurement;
use crate::ml::model::Model;

use crate::proc::prophet;

use crate::stats::chronology::Chronology;
use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;
use crate::stats::prediction::Prediction;
use crate::stats::visiting::Visiting;

use crate::storage::read_disk::NBAReadError;

use crate::storage::write::write_serializable_with_directory;
use crate::tui::game_ratings::GameRatings;
use crate::tui::tui_display::TuiDisplay;

use crate::types::PlayerId;

use std::collections::HashMap;
use std::io;

const ELO_VERSION: &str = "elo v1";

pub struct EloTracker {
    historical_ratings: Vec<Elo>,
    current_ratings: HashMap<PlayerId, i64>,
    log_loss: LogLossTracker,
    predictions: Vec<Prediction>,
    params: EloParams,
}

impl EloTracker {
    pub fn new() -> Self {
        Self {
            historical_ratings: Vec::new(),
            current_ratings: HashMap::new(),
            log_loss: LogLossTracker::model(ELO_VERSION.to_owned()),
            predictions: vec![],
            params: EloParams::default(),
        }
    }

    pub(crate) fn params(params: EloParams) -> Self {
        Self {
            historical_ratings: Vec::new(),
            current_ratings: HashMap::new(),
            log_loss: LogLossTracker::model(ELO_VERSION.to_owned()),
            predictions: vec![],
            params,
        }
    }

    fn process_elo(&mut self, games: &[(GameCard, GameObject)]) {
        // todo: assign elo values to players on a game by game basis
        // maybe assert ordered on the basis. no frick u man

        let mut predictions = Vec::new();

        for (slip, box_score) in games {
            // remember there is now way to predict
            // the first event other than fiftEE-fiftEE
            let prediction = self.predict(&slip);

            self.update_ratings(slip, box_score);

            predictions.push(Prediction::from(slip.clone(), prediction));
        }

        self.predictions.extend(predictions);
    }

    //todo: implement a rating share function as a parameter
    fn update_ratings(&mut self, slip: &GameCard, box_score: &GameObject) {
        let home_rating =
            self.normalized_ratings_from_iter(slip.home_roster().into_iter().map(|x| *x));
        let away_rating =
            self.normalized_ratings_from_iter(slip.away_roster().into_iter().map(|x| *x));

        let delta = home_rating - away_rating;
        //R'=R+K∙(S-E) where s is the score and e is the expected (1 for win, 0 for loss - win probability)
        let mut home_step =
            (self.step() as f64 * (cdf::prob(delta, self.scale_factor()))).round() as i64;
        let mut away_step =
            (self.step() as f64 * (1f64 - cdf::prob(delta, self.scale_factor()))).round() as i64;

        //this is the winners step, the losers step is -step
        if box_score.winner() == slip.home().team_id() {
            away_step = -1 * (away_step as i64);
        } else if box_score.winner() == slip.away().team_id() {
            home_step = -1 * (home_step as i64);
        } else {
            panic!("💀 Game must have a winner that was a participant. Somehow passed the win/loss check in GameObject::try_create");
        }

        let init = self.initial_rating();

        //no update based on what the scorecard reports (not initial gueses)
        for player in box_score.home_roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += home_step)
                .or_insert(init);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: slip.game_id(),
                rating: self.current_ratings[&id],
            });
        }
        for player in box_score.away_roster() {
            let id = player.player_id();

            self.current_ratings
                .entry(id)
                .and_modify(|rating| *rating += away_step)
                .or_insert(init);

            self.historical_ratings.push(Elo {
                player_id: id,
                game_id: slip.game_id(),
                rating: self.current_ratings[&id],
            });
        }

        self.track_log_loss(box_score, delta);

        // println!("======================================================================");
        // self.current_ratings.iter().for_each(|(player, rating)| {
        //     println!("{}: {}", player, rating);
        // });
        // println!("======================================================================");
    }

    fn track_log_loss(&mut self, game: &GameObject, delta: f64) {
        let p = cdf::prob(delta, self.scale_factor());

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

    pub(crate) fn freq(&self) -> f64 {
        self.log_loss.freq()
    }

    pub(crate) fn log_loss(&self) -> f64 {
        self.log_loss.log_loss()
    }

    // SERIALIZATION

    pub fn save(&self) -> Result<(), EloTrackerError> {
        self.save_records()?;

        self.save_results()?;

        self.save_predictions()?;

        Ok(())
    }

    fn save_records(&self) -> Result<(), EloTrackerError> {
        let records_filename = records_path(self);

        let mut writer = EloWriter::new(records_filename)
            .map_err(|e| EloTrackerError::WriterCreationError(e))?;

        for record in &self.historical_ratings {
            writer
                .serialize_elo(&record)
                .map_err(|e| EloTrackerError::WriteEloError(e))?;
        }

        Ok(())
    }

    fn save_results(&self) -> Result<(), EloTrackerError> {
        let results_filename = results_path(self);

        write_serializable_with_directory(results_filename, &self.log_loss)
            .map_err(|e| EloTrackerError::WriteResultsError(e))
    }

    fn save_predictions(&self) -> Result<(), EloTrackerError> {
        prophet::write_predictions(self, &self.predictions)
            .map_err(|e| EloTrackerError::WritePredictionError(e))
    }

    //todo: implement a less safe version of this function that accepts a immutable reference to the tracker
    //      panic the program if called incorrectly.
    //
    // mayeb dont add this and suffer a teensy performance hit
    pub fn normalized_ratings_from_iter(&mut self, iter: impl Iterator<Item = PlayerId>) -> f64 {
        let init = self.initial_rating();

        let (count, sum) = iter.fold((0usize, 0i64), |acc, id| {
            (
                acc.0 + 1,
                acc.1 + *self.current_ratings.entry(id).or_insert(init),
            )
        });

        if count == 0 {
            return self.initial_rating() as f64; // initial rating for both teams. this gives 0 diff for cdf,
                                                 //  if theres no prior data, then we have to assume 50-50
        }

        sum as f64 / count as f64
    }

    pub fn predict_cards(&mut self, gamecards: Vec<GameCard>) -> Vec<Prediction> {
        let mut chronology = Chronology::new();
        let mut predictions = Vec::new();

        for mut game in gamecards.into_iter() {
            chronology
                .load_era(game.season())
                .expect(&format!("💀 failed to load season_era: {}", game.season()));

            let game_ratings = GameRatings::new(&game, &mut chronology, &mut self.current_ratings);

            println!("{}", game_ratings.display());

            chronology
                .load_era(game.season())
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
            let prob = cdf::prob(home_rating - away_rating, self.scale_factor());

            let prediction = Prediction::from(game, prob);

            predictions.push(prediction);
        }

        predictions
    }

    fn scale_factor(&self) -> f64 {
        self.params.scale_factor()
    }

    fn initial_rating(&self) -> i64 {
        self.params.initial_rating()
    }

    fn step(&self) -> i64 {
        self.params.step()
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
    #[error("❌ {0}\n❌ error writing results to file. ")]
    WriteResultsError(io::Error),
    #[error("❌ {0}\n❌ error writing elo records to file. ")]
    WriteEloError(EloWriterError),
}

impl Model for EloTracker {
    fn predict(&mut self, card: &GameCard) -> f64 {
        let home_rating =
            self.normalized_ratings_from_iter(card.home_roster().into_iter().map(|x| *x));
        let away_rating =
            self.normalized_ratings_from_iter(card.away_roster().into_iter().map(|x| *x));

        let diff = home_rating - away_rating;

        cdf::prob(diff, self.scale_factor())
    }

    fn model_name(&self) -> String {
        ELO_VERSION.to_string()
    }

    fn evaluate(&self) -> f64 {
        if self.log_loss.is_empty() {
            panic!("💀 LogLossTracker is empty, no measurements have been recorded.")
        }

        let f = self.log_loss.freq();
        let logloss = self.log_loss.log_loss();

        f / logloss
    }

    fn train(&mut self, games: &[(GameCard, GameObject)]) {
        self.process_elo(games);
    }
}

#[cfg(test)]
mod test_elo_tracker {
    use crate::{ml::vector::Vector, types::SeasonId};

    use super::*;

    #[test]
    fn test_default_initial_rating() {
        let tracker = EloTracker::new();
        assert_eq!(tracker.initial_rating(), 3000);
    }

    #[test]
    fn test_default_step() {
        let tracker = EloTracker::new();
        assert_eq!(tracker.step(), 32);
    }

    #[test]
    fn test_default_scale_factor() {
        let tracker = EloTracker::new();
        assert_eq!(tracker.scale_factor(), 400.0);
    }

    #[test]
    fn test_process_elo() {
        let mut tracker = EloTracker::params(EloParams::new(&Vector::from(vec![64.0, 400.0])));
        let chronology = Chronology::from_era(SeasonId::from(42024));

        let mut pairs = chronology
            .games()
            .as_ref()
            .expect("chrono")
            .iter()
            .map(|game| (game.card(), game.clone()))
            .collect::<Vec<_>>();

        for (slip, _box_score) in pairs.iter_mut() {
            let away_expected =
                chronology.get_expected_roster(slip.away().team_id(), slip.game_id());
            let home_expected =
                chronology.get_expected_roster(slip.home().team_id(), slip.game_id());

            slip.add_away_ratings(away_expected);
            slip.add_home_ratings(home_expected);
        }

        dbg!(&pairs);

        tracker.process_elo(&pairs);
    }
}
