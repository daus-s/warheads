use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;

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
use crate::stats::nba_kind::NBAStatKind;

inventory::submit!(Registration {
    model_name: SIGMA_VERSION,
    args_schema: || clap::Command::new("logisitic regression over team performance"),
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
            .as_regression_data(NBAStatKind::Team, 0.7)
            .map_err(|e| TrainingError::VolumeLoadingError(e))?;

        let training_data: Vec<(Vector, u8)> = training_data
            .into_iter()
            .map(|mut vec| {
                //result
                let wl = vec.slice(17) as u8;

                let _ignore_plus_minus = vec.slice(16);

                (vec, wl)
            })
            .collect();

        self.model.gradient_descent(&training_data, 0.01);

        let mut tracker = LogLossTracker::new();

        testing_data.into_iter().for_each(|vec| {
            let outcome = vec[18] as u8;
            let vec: Vector = vec.into();

            dbg!(&vec);

            println!("{:?}", self.named_model());

            let prob = self.model.predict(&vec);

            let obs = Observation::new(outcome, prob);

            tracker.add_observation(obs);
        });

        let mut model_path = path_manager::model_dir(self);

        model_path.push("model.md");

        fs::write(model_path, format!("{:?}", self.named_model()))
            .map_err(|e| TrainingError::ArtifactSaveError(e))?;

        write_serializable_with_directory(results_path(self), &tracker)
            .map_err(|e| TrainingError::ArtifactSaveError(e))?;

        Ok(())
    }

    fn evaluate(&self) -> std::collections::HashMap<String, f64> {
        todo!()
    }

    fn predict(&mut self, obj: &crate::stats::gamecard::GameCard) -> f64 {
        todo!()
    }
}

impl SigmaChadModel {
    fn new() -> Self {
        Self {
            model: LogisticRegression::new(Vector::origin(16), 0.0),
        }
    }

    fn named_model(&self) -> NamedLog {
        let mut map = HashMap::new();

        for (order, weight) in self.model.params().iter().enumerate() {
            map.insert(
                match order {
                    0 => "min",
                    1 => "fgm",
                    2 => "fga",
                    3 => "fg3m",
                    4 => "fg3a",
                    5 => "ftm",
                    6 => "fta",
                    7 => "oreb",
                    8 => "dreb",
                    9 => "reb",
                    10 => "ast",
                    11 => "stl",
                    12 => "blk",
                    13 => "tov",
                    14 => "pf",
                    15 => "pts",
                    _ => unimplemented!("💀 unknown field"),
                }
                .to_owned(),
                (order, weight),
            );
        }

        map.insert(String::from("bias"), (16, self.model.bias()));

        NamedLog::new(map)
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
                if *i != 17 {
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

    map.insert("min".into(), (0, -2.131780970172711));
    map.insert("fgm".into(), (1, 8.095358160092806));
    map.insert("fga".into(), (2, -73.77097150047216));
    map.insert("fg3m".into(), (3, 17.617562463279555));
    map.insert("fg3a".into(), (4, -4.5088904598934105));
    map.insert("ftm".into(), (5, 2.994567420702568));
    map.insert("fta".into(), (6, -29.97773132492458));
    map.insert("oreb".into(), (7, 22.61070800894604));
    map.insert("dreb".into(), (8, 20.067750601939657));
    map.insert("reb".into(), (9, 54.99805066857796));
    map.insert("ast".into(), (10, 6.62138372957267));
    map.insert("stl".into(), (11, 73.57472609372095));
    map.insert("blk".into(), (12, 19.11615264640488));
    map.insert("tov".into(), (13, -70.36496252867897));
    map.insert("pf".into(), (14, -18.798908308026324));
    map.insert("pts".into(), (15, 37.788566871548454));
    map.insert("bias".into(), (16, -8.139505434305125));
    //insert the rest of the columns based on this order

    let log = NamedLog::new(map);

    let expected = "$P_w=\\frac{1}{1+e^{-z}}$\nwhere\n$$\\\\ z=\\begin{aligned}\\\\&+min\\cdot-2.131780970172711 \\\\&+fgm\\cdot8.095358160092806 \\\\&+fga\\cdot-73.77097150047216 \\\\&+fg3m\\cdot17.617562463279555 \\\\&+fg3a\\cdot-4.5088904598934105 \\\\&+ftm\\cdot2.994567420702568 \\\\&+fta\\cdot-29.97773132492458 \\\\&+oreb\\cdot22.61070800894604 \\\\&+dreb\\cdot20.067750601939657 \\\\&+reb\\cdot54.99805066857796 \\\\&+ast\\cdot6.62138372957267 \\\\&+stl\\cdot73.57472609372095 \\\\&+blk\\cdot19.11615264640488 \\\\&+tov\\cdot-70.36496252867897 \\\\&+pf\\cdot-18.798908308026324 \\\\&+pts\\cdot37.788566871548454 \\\\&+bias\\cdot-8.139505434305125 \\\\&+-8.139505434305125\\end{aligned}$$";

    println!("{:?}", log);

    pretty_assertions::assert_eq!(format!("{:?}", log), expected);
}
