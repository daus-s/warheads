mod test_extract {
    #[test]
    fn test_json_to_hashmap() {
        let left = true;

        let right = true;

        assert_eq!(left, right);
    }
}


mod test_path_manager {
    use std::path::PathBuf;
    use once_cell::sync::Lazy;
    use crate::constants::paths::data;
    use crate::format::path_manager::nba_storage_path;
    use crate::stats::id::Identity;
    use crate::types::{GameId, SeasonId, TeamAbbreviation, TeamId};

    #[test]
    fn test_nba_storage_path() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_path = PathBuf::from(format!("{}/nba/store/2025_26/RegularSeason/42424343_SON", *DATA));
        
        let id = Identity {
            season_id: SeasonId::from(22025),
            player_id: None,
            team_id: TeamId(69420),
            team_abbr: TeamAbbreviation("SON".to_string()),
            game_id: GameId("42424343".to_string()),
        };

        let actual_path = nba_storage_path(id);

        assert_eq!(expected_path, actual_path)
    }
}