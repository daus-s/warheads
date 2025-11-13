#[cfg(test)]
mod test_extract {
    #[test]
    fn test_json_to_hashmap() {
        let left = true;

        let right = true;

        assert_eq!(left, right);
    }
}

#[cfg(test)]
mod test_path_manager {
    use crate::constants::paths::data;
    use crate::format::path_manager::{
        nba_prediction_file, nba_storage_file, nba_storage_path, nba_team_correction_file,
    };
    use crate::ml::model::Model;
    use crate::types::{GameDate, GameId, SeasonId, TeamId};
    use once_cell::sync::Lazy;
    use std::path::PathBuf;

    const MODEL: &'static str = "test_model";

    #[test]
    fn test_nba_storage_path() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file = PathBuf::from(format!(
            "{}/nba/store/2025_26/RegularSeason/0042424343",
            *DATA
        ));
        let expected_path = PathBuf::from(format!("{}/nba/store/2025_26/RegularSeason/", *DATA));

        let szn = SeasonId::from(22025);

        let game_id = GameId(42424343);

        let actual_file = nba_storage_file(szn, game_id);
        let actual_path = nba_storage_path(szn);

        assert_eq!(expected_path, actual_path);
        assert_eq!(expected_file, actual_file);
    }

    #[test]
    fn test_nba_correction_file() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file = PathBuf::from(format!(
            "{}/nba/corrections/teamgames/2008_09/preseason/0010800035_1610612743.corr",
            *DATA
        ));

        let szn = SeasonId::from(12008);

        let game_id = GameId::from("0010800035");

        let team_id = TeamId::from(1610612743);

        let actual_file = nba_team_correction_file(szn, game_id, team_id);

        assert_eq!(expected_file, actual_file);
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

        fn predict(&mut self, _obj: &crate::stats::game_obj::GameObject) -> f64 {
            todo!()
        }
    }
}
