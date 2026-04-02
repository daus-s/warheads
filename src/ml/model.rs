use std::collections::HashMap;
use std::io;

use thiserror::Error;

use crate::ml::models::elo_tracker::EloTrackerError;
use crate::proc::error;
use crate::stats::chronology::{Chronology, ChronologyError};
use crate::stats::gamecard::GameCard;

pub trait Model {
    fn model_name(&self) -> String;
    fn initialize(&mut self) -> Result<(), ()>;

    fn train(&mut self, data: Chronology) -> Result<(), TrainingError>;
    fn evaluate(&self) -> HashMap<String, f64>; // this could return a vec of measurements or a like structure
    fn predict(&mut self, obj: &GameCard) -> f64;
}
// the idea is
// model name is
// predict - given some trait like predictable or something?
// evaluate - calculate log loss or residuals or something? calculate the Objective. Create an ObjectiveTrait that does somethign idk
//
//

#[derive(Debug, Error)]
pub enum TrainingError {
    #[error("{0}\n❌ failed to load training data from chronlogy")]
    VolumeLoadingError(ChronologyError),
    #[error("❌ {0}\n❌ failed to serialize model artifact")]
    WincodeSerializationError(wincode::WriteError),
    #[error("❌ {0}\n❌ failed to serialize model artifact")]
    JsonSerializationError(serde_json::Error),
    #[error("❌ {0}\n❌ failed to save model artifact to file:")]
    ArtifactSaveError(io::Error),
    #[error("{0}\n❌ failed to save elo artifacts after training")]
    EloSaveError(EloTrackerError),
}

impl Model for Box<dyn Model> {
    fn model_name(&self) -> String {
        (**self).model_name()
    }

    fn initialize(&mut self) -> Result<(), ()> {
        (**self).initialize()
    }

    fn train(&mut self, data: Chronology) -> Result<(), TrainingError> {
        (**self).train(data)
    }

    fn evaluate(&self) -> HashMap<String, f64> {
        (**self).evaluate()
    }

    fn predict(&mut self, obj: &GameCard) -> f64 {
        (**self).predict(obj)
    }
}
