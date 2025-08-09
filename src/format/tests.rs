mod test_extract {
    #[test]
    fn test_json_to_hashmap() {
        let left = true;

        let right = true;

        assert_eq!(left, right);
    }
}

mod test_path_manager {
    
    
    
    
    
    

    #[test]
    fn test_nba_storage_path() {
        static DATA: Lazy<String> = Lazy::new(data);

        let expected_file = PathBuf::from(format!(
            "{}/nba/store/2025_26/RegularSeason/42424343",
            *DATA
        ));
        let expected_path = PathBuf::from(format!("{}/nba/store/2025_26/RegularSeason/", *DATA));

        let szn = SeasonId::from(22025);

        let game_id = GameId(42424343);

        let id = Identity {
            season_id: szn,
            player_id: None,
            team_id: TeamId(69420),
            team_abbr: TeamAbbreviation("SON".to_string()),
            game_id,
            game_date: GameDate(Default::default()),
        };

        let actual_file = nba_storage_file((szn, game_id));
        let actual_path = nba_storage_path(szn);

        assert_eq!(expected_path, actual_path);
        assert_eq!(expected_file, actual_file);
    }
}
