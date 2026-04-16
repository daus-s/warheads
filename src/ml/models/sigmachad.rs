use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

use serde_json::Value;

use crate::dapi::write::write_serializable_with_directory;

use crate::format::path_manager::{self, results_path};

use crate::ml::gradient_descent::GradientDescent;
use crate::ml::log_loss::LogLossTracker;
use crate::ml::logisitic_regression::LogisticRegression;
use crate::ml::model::{self, TrainingError};
use crate::ml::models::registration::Registration;
use crate::ml::observation::Observation;
use crate::ml::vector::Vector;

use crate::stats::chronology::Chronology;
use crate::stats::gamecard::GameCard;
use crate::stats::nba_kind::NBAStatKind;

inventory::submit!(Registration {
    model_name: SIGMA_VERSION,
    args_schema: || clap::Command::new("logistic regression over team performance"),
    factory: |_| { Box::new(SigmaChadModel::new()) },
});

pub struct SigmaChadModel {
    model: LogisticRegression,
}

const SIGMA_VERSION: &'static str = "sigmachad-v1";

impl model::Model for SigmaChadModel {
    fn model_name(&self) -> String {
        SIGMA_VERSION.to_owned()
    }

    fn initialize(&mut self) -> Result<(), ()> {
        todo!() //read trained logisitc model from file (custom human readable format ideally)
    }

    fn train(&mut self, chrono: Chronology) -> Result<(), model::TrainingError> {
        let (training_data, testing_data) = chrono
            .as_pure_regression_data(NBAStatKind::Team, 0.7)
            .map_err(|e| TrainingError::VolumeLoadingError(e))?;

        let mut sums = Vector::origin(18); //this will included ignored features
        let mut n = 0;

        for box_score in training_data.iter().chain(testing_data.iter()) {
            sums += box_score;
            n += 1;
        }

        let averages = sums / n as f64;

        let training_data: Vec<(Vector, u8)> = training_data
            .into_iter()
            .map(|mut vec| {
                // normalize the vector by the average of each feature
                let inv: Vector = averages.iter().map(|x| 1f64 / x).collect::<Vec<_>>().into();
                vec = vec.element_wise_multiplication(&inv);

                let wl = vec.slice(17) as u8;
                let _ignore_plus_minus = vec.slice(16);
                let _ignore_minutes = vec.slice(0);

                (vec, wl)
            })
            .collect();

        self.model.gradient_descent(&training_data, 0.01);

        let mut tracker = LogLossTracker::new();

        testing_data.into_iter().for_each(|mut vec| {
            let inv: Vector = averages.iter().map(|x| 1f64 / x).collect::<Vec<_>>().into();
            vec = vec.element_wise_multiplication(&inv);

            let wl = vec.slice(17) as u8;
            let _ignore_plus_minus = vec.slice(16);
            let _ignore_minutes = vec.slice(0);

            vec /= n as f64;

            let vec: Vector = vec.into();

            let prob = self.model.predict(&vec);

            let obs = Observation::new(wl, prob);

            tracker.add_observation(obs);
        });

        self.save(&tracker)?;

        Ok(())
    }

    fn evaluate(&self) -> HashMap<String, f64> {
        if let Ok(file) = fs::read_to_string(results_path(self)) {
            if let Ok(json) = serde_json::from_str::<HashMap<Value, Value>>(&file) {
                return json
                    .into_iter()
                    .map(|(k, v)| (k.as_str().unwrap().to_string(), v.as_f64().unwrap()))
                    .collect();
            }
        }

        return HashMap::new();
    }

    fn predict(&mut self, _obj: &GameCard) -> f64 {
        todo!()
    }
}

impl SigmaChadModel {
    fn new() -> Self {
        Self {
            model: LogisticRegression::new(Vector::origin(15), 0.0),
        }
    }

    fn named_model(&self) -> NamedLog {
        let mut map = HashMap::new();

        for (order, weight) in self.model.params().iter().enumerate() {
            map.insert(
                match order {
                    0 => "fgm",
                    1 => "fga",
                    2 => "fg3m",
                    3 => "fg3a",
                    4 => "ftm",
                    5 => "fta",
                    6 => "oreb",
                    7 => "dreb",
                    8 => "reb",
                    9 => "ast",
                    10 => "stl",
                    11 => "blk",
                    12 => "tov",
                    13 => "pf",
                    14 => "pts",
                    _ => unimplemented!("💀 unknown field"),
                }
                .to_owned(),
                (order, weight),
            );
        }

        map.insert(String::from("bias"), (15, self.model.bias()));

        NamedLog::new(map)
    }

    fn save(&self, tracker: &LogLossTracker) -> Result<(), TrainingError> {
        let mut model_path = path_manager::model_dir(self);

        let _ = fs::create_dir_all(&model_path);
        model_path.push("model.md");

        let model_file = model_path;

        fs::write(model_file, format!("{:?}", self.named_model()))
            .map_err(|e| TrainingError::ArtifactSaveError(e))?;

        write_serializable_with_directory(results_path(self), &tracker)
            .map_err(|e| TrainingError::ArtifactSaveError(e))
    }
}

impl Debug for NamedLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        {
            let mut s = String::from("$P_w=\\frac{1}{1+e^{-z}}$\nwhere\n");

            s.push_str("$$\\\\ z=\\begin{aligned}\\\\");

            let mut ordered: Vec<(String, (usize, f64))> = self.map.clone().into_iter().collect();

            ordered.sort_by_key(|(_, (i, _))| *i);

            ordered.iter().for_each(|(xi, (i, w))| {
                if *i != 15 {
                    s.push_str(&format!("&+{xi}\\cdot{w} \\\\"))
                };
            });

            s.push_str(&format!(
                "&+{}",
                self.map
                    .get("bias")
                    .expect("💀 NamedLog does not have a bias field. ")
                    .1
            ));
            s.push_str("\\end{aligned}");

            s.push_str("$$");

            write!(f, "{}", s)
        }
    }
}

struct NamedLog {
    map: HashMap<String, (usize, f64)>,
}

impl NamedLog {
    fn new(map: HashMap<String, (usize, f64)>) -> Self {
        Self { map }
    }
}

#[test]
fn test_named_logistic_regression() {
    let mut map = HashMap::new();

    map.insert("fgm".into(), (0, 8.095358160092806));
    map.insert("fga".into(), (1, -73.77097150047216));
    map.insert("fg3m".into(), (2, 17.617562463279555));
    map.insert("fg3a".into(), (3, -4.5088904598934105));
    map.insert("ftm".into(), (4, 2.994567420702568));
    map.insert("fta".into(), (5, -29.97773132492458));
    map.insert("oreb".into(), (6, 22.61070800894604));
    map.insert("dreb".into(), (7, 20.067750601939657));
    map.insert("reb".into(), (8, 54.99805066857796));
    map.insert("ast".into(), (9, 6.62138372957267));
    map.insert("stl".into(), (10, 73.57472609372095));
    map.insert("blk".into(), (11, 19.11615264640488));
    map.insert("tov".into(), (12, -70.36496252867897));
    map.insert("pf".into(), (13, -18.798908308026324));
    map.insert("pts".into(), (14, 37.788566871548454));
    map.insert("bias".into(), (15, -8.139505434305125));
    //insert the rest of the columns based on this order

    let log = NamedLog::new(map);

    let expected = "$P_w=\\frac{1}{1+e^{-z}}$\nwhere\n$$\\\\ z=\\begin{aligned}\\\\&+fgm\\cdot8.095358160092806 \\\\&+fga\\cdot-73.77097150047216 \\\\&+fg3m\\cdot17.617562463279555 \\\\&+fg3a\\cdot-4.5088904598934105 \\\\&+ftm\\cdot2.994567420702568 \\\\&+fta\\cdot-29.97773132492458 \\\\&+oreb\\cdot22.61070800894604 \\\\&+dreb\\cdot20.067750601939657 \\\\&+reb\\cdot54.99805066857796 \\\\&+ast\\cdot6.62138372957267 \\\\&+stl\\cdot73.57472609372095 \\\\&+blk\\cdot19.11615264640488 \\\\&+tov\\cdot-70.36496252867897 \\\\&+pf\\cdot-18.798908308026324 \\\\&+pts\\cdot37.788566871548454 \\\\&+-8.139505434305125\\end{aligned}$$";

    println!("{:?}", log);

    pretty_assertions::assert_eq!(format!("{:?}", log), expected);
}
