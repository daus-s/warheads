#[cfg(test)]
mod test_path_manager {
    use crate::constants::paths::data;
    use crate::format::path_manager::{nba_prediction_file, nba_storage_path};
    use crate::ml::model::{Model, TrainingError};
    use crate::stats::chronology::Chronology;
    use crate::stats::game_obj::GameObject;
    use crate::stats::gamecard::GameCard;
    use crate::types::{GameDate, SeasonId};
    use once_cell::sync::Lazy;
    use std::collections::HashMap;
    use std::path::PathBuf;

    const MODEL: &'static str = "test_model";

    #[test]
    fn test_nba_storage_path() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_path = PathBuf::from(format!("{}/nba/volumes/2025_regularseason.vol", *DATA));

        let szn = SeasonId::from(22025);

        let actual_path = nba_storage_path(szn);

        assert_eq!(expected_path, actual_path);
    }

    #[test]
    fn test_nba_prediction_file() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file =
            PathBuf::from(format!("{}/nba/{}/predictions/2025_04_30", *DATA, MODEL));

        let actual_file = nba_prediction_file(&TestModel, GameDate::ymd(2025, 4, 30).unwrap());

        assert_eq!(expected_file, actual_file);
    }

    #[test]
    fn test_nba_prediction_file_short_day() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file =
            PathBuf::from(format!("{}/nba/{}/predictions/2025_04_09", *DATA, MODEL));

        let actual_file = nba_prediction_file(&TestModel, GameDate::ymd(2025, 4, 9).unwrap());

        assert_eq!(expected_file, actual_file);
    }

    #[test]
    fn test_nba_prediction_file_short_month() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file =
            PathBuf::from(format!("{}/nba/{}/predictions/2025_01_09", *DATA, MODEL));

        let actual_file = nba_prediction_file(&TestModel, GameDate::ymd(2025, 1, 9).unwrap());

        assert_eq!(expected_file, actual_file);
    }

    struct TestModel;

    impl Model for TestModel {
        fn model_name(&self) -> String {
            "test_model".to_owned()
        }

        fn initialize(&mut self) -> Result<(), ()> {
            todo!()
        }

        fn train(&mut self, _data: Chronology) -> Result<(), TrainingError> {
            todo!()
        }

        fn evaluate(&self) -> HashMap<String, f64> {
            todo!()
        }

        fn predict(&mut self, _obj: &crate::stats::gamecard::GameCard) -> f64 {
            todo!()
        }
    }
}
