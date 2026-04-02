use std::collections::HashMap;

use clap::Arg;

use crate::ml::elo::elo_params::EloParams;
use crate::ml::model::{Model, TrainingError};
use crate::ml::models::elo_models::elo_tracker::EloTracker;
use crate::ml::models::registration::Registration;
use crate::ml::nelder_mead::nelder_mead;
use crate::ml::simplex::Simplex;
use crate::ml::vector::Vector;

use crate::stats::chronology::Chronology;
use crate::stats::game_obj::GameObject;
use crate::stats::gamecard::GameCard;

/// this elo algorithm is optimized on (k, f) pairs,
/// the algorithm also uses a initial rating of 0 for symmetry.
pub struct NelderMeadEloTracker {
    mapping: HashMap<u128, f64>,
    is_trained: bool,
    params: Vector,
    performance: f64,
}

impl NelderMeadEloTracker {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
            is_trained: false,
            params: Vector::from(vec![32., 400.]),
            performance: 0.0,
        }
    }
}

impl Model for NelderMeadEloTracker {
    fn model_name(&self) -> String {
        "nelder-mead elo v1".to_string()
    }

    fn initialize(&mut self) -> Result<(), ()> {
        todo!()
    }

    fn train(&mut self, chrono: Chronology) -> Result<(), TrainingError> {
        let mut simplex = Simplex::from(&vec![
            Vector::from(vec![32., 400.]),
            Vector::from(vec![24., 400.]),
            Vector::from(vec![32., 500.]),
        ]);

        let cost = |v: &Vector| -> f64 {
            let mut tracker = EloTracker::params(EloParams::new(v));

            tracker.train(chrono.clone());

            println!(
                "{}/{}=>{}",
                tracker.freq(),
                tracker.log_loss(),
                tracker.freq() / tracker.log_loss()
            );

            tracker.freq() / tracker.log_loss()
        };

        let baseline = 0.46304378813918995;

        while self.performance < baseline * 0.7 {
            nelder_mead(cost, &mut simplex);

            for v in simplex.iter() {
                let new_performance = cost(v);
                let mut hash: u128 = 0;

                hash |= v.x().to_bits() as u128;
                hash |= v.y().to_bits() as u128 >> 64;

                self.mapping.insert(hash, new_performance);

                if new_performance > self.performance {
                    self.performance = new_performance;
                }
            }
        }

        println!(
            "Score: {}\nBaseline: {}\nstep: {}\tscale factor: {}",
            self.performance,
            baseline,
            self.params.x(),
            self.params.y()
        );

        Ok(())
    }

    fn evaluate(&self) -> HashMap<String, f64> {
        if !self.is_trained {
            panic!("💀 model not trained");
        }

        todo!()
    }

    fn predict(&mut self, _obj: &crate::stats::gamecard::GameCard) -> f64 {
        if !self.is_trained {
            panic!("💀 model not trained");
        }
        todo!()
    }
}

#[cfg(test)]
mod test_nelder_mead_elo {
    use crate::stats::chronology::Chronology;

    use super::*;

    #[allow(dead_code)]
    fn get_optimal_params() {
        let mut tracker = NelderMeadEloTracker::new();

        tracker.train(Chronology::new());

        assert!(tracker.performance > 0.46304378813918995 * 0.7);
    }
}

inventory::submit!(Registration {
    model_name: "nelder-mead elo v1",
    args_schema: || clap::Command::new("nelder-mead optimizated elo. ").arg(
        Arg::new("mesh")
            .long("mesh")
            .value_parser(clap::value_parser!(bool))
            .default_value("false")
    ),
    factory: |_args| Box::new(NelderMeadEloTracker::new()),
});
