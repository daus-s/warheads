use std::collections::HashMap;

use thiserror::Error;

use crate::dapi::timeline_manager::LoadGameCardError;
use crate::dapi::{season_manager::nba_lifespan_period, timeline_manager::load_era_gamecards};
use crate::ml::model::Model;
use crate::stats::prediction::Prediction;
use crate::types::GameDate;

pub fn generate_predictions<M: Model>(
    model: M,
) -> Result<HashMap<GameDate, Vec<Prediction>>, PredictionGenerationError> {
    let mut predictions: HashMap<GameDate, Vec<Prediction>> = HashMap::new();

    for era in nba_lifespan_period() {
        let cards = load_era_gamecards(era).map_err(|e| PredictionGenerationError::LoadError(e))?;

        for card in cards {
            let prediction = Prediction::new(&card, model.predict(&card));

            predictions
                .entry(card.date())
                .or_insert(vec![])
                .push(prediction);
        }
    }

    Ok(predictions)
}

#[derive(Debug, Error)]
pub enum PredictionGenerationError {
    #[error("❌ {0}\n❌ Error loading game cards while generating predictions.")]
    LoadError(LoadGameCardError),
}
