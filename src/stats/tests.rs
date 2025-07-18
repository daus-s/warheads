#[cfg(test)]
mod tests {
    use crate::stats::stat_column::player_column_index;
    use crate::stats::stat_column::StatColumn::*;

    #[test]
    fn test_season_id_column() {
        let index = player_column_index(&SEASON_ID).unwrap();

        assert_eq!(0usize, index);
    }

    #[test]
    fn test_player_id_column() {
        let index = player_column_index(&PLAYER_ID).unwrap();

        assert_eq!(1usize, index);
    }

    #[test]
    fn test_player_name_column() {
        let index = player_column_index(&PLAYER_NAME).unwrap();

        assert_eq!(2usize, index);
    }

    #[test]
    fn test_team_id_column() {
        let index = player_column_index(&TEAM_ID).unwrap();

        assert_eq!(3usize, index);
    }

    #[test]
    fn test_team_abbr_column() {
        let index = player_column_index(&TEAM_ABBREVIATION).unwrap();

        assert_eq!(4usize, index);
    }

    #[test]
    fn test_team_name_column() {
        let index = player_column_index(&TEAM_NAME).unwrap();

        assert_eq!(5usize, index);
    }

    #[test]
    fn test_game_id_column() {
        let index = player_column_index(&GAME_ID).unwrap();

        assert_eq!(6usize, index);
    }

    #[test]
    fn test_game_date_column() {
        let index = player_column_index(&GAME_DATE).unwrap();

        assert_eq!(7usize, index);
    }

    #[test]
    fn test_matchup_column() {
        let index = player_column_index(&MATCHUP).unwrap();

        assert_eq!(8usize, index);
    }

    #[test]
    fn test_wl_column() {
        let index = player_column_index(&WL).unwrap();

        assert_eq!(9usize, index);
    }

    #[test]
    fn test_min_column() {
        let index = player_column_index(&MIN).unwrap();

        assert_eq!(10usize, index);
    }

    #[test]
    fn test_fgm_column() {
        let index = player_column_index(&FGM).unwrap();

        assert_eq!(11usize, index);
    }

    #[test]
    fn test_fga_column() {
        let index = player_column_index(&FGA).unwrap();

        assert_eq!(12usize, index);
    }

    #[test]
    fn test_fg_pct_column() {
        let index = player_column_index(&FG_PCT).unwrap();

        assert_eq!(13usize, index);
    }

    #[test]
    fn test_fg3m_column() {
        let index = player_column_index(&FG3M).unwrap();

        assert_eq!(14usize, index);
    }

    #[test]
    fn test_fg3a_column() {
        let index = player_column_index(&FG3A).unwrap();

        assert_eq!(15usize, index);
    }

    #[test]
    fn test_fg3_pct_column() {
        let index = player_column_index(&FG3_PCT).unwrap();

        assert_eq!(16usize, index);
    }

    #[test]
    fn test_ftm_column() {
        let index = player_column_index(&FTM).unwrap();

        assert_eq!(17usize, index);
    }

    #[test]
    fn test_fta_column() {
        let index = player_column_index(&FTA).unwrap();

        assert_eq!(18usize, index);
    }

    #[test]
    fn test_ft_pct_column() {
        let index = player_column_index(&FT_PCT).unwrap();

        assert_eq!(19usize, index);
    }

    #[test]
    fn test_oreb_column() {
        let index = player_column_index(&OREB).unwrap();

        assert_eq!(20usize, index);
    }

    #[test]
    fn test_dreb_column() {
        let index = player_column_index(&DREB).unwrap();

        assert_eq!(21usize, index);
    }

    #[test]
    fn test_reb_column() {
        let index = player_column_index(&REB).unwrap();

        assert_eq!(22usize, index);
    }

    #[test]
    fn test_ast_column() {
        let index = player_column_index(&AST).unwrap();

        assert_eq!(23usize, index);
    }

    #[test]
    fn test_stl_column() {
        let index = player_column_index(&STL).unwrap();

        assert_eq!(24usize, index);
    }

    #[test]
    fn test_blk_column() {
        let index = player_column_index(&BLK).unwrap();

        assert_eq!(25usize, index);
    }

    #[test]
    fn test_tov_column() {
        let index = player_column_index(&TOV).unwrap();

        assert_eq!(26usize, index);
    }

    #[test]
    fn test_pf_column() {
        let index = player_column_index(&PF).unwrap();

        assert_eq!(27usize, index);
    }

    #[test]
    fn test_pts_column() {
        let index = player_column_index(&PTS).unwrap();

        assert_eq!(28usize, index);
    }

    #[test]
    fn test_plus_minus_column() {
        let index = player_column_index(&PLUS_MINUS).unwrap();

        assert_eq!(29usize, index);
    }

    #[test]
    fn test_fantasy_column() {
        let index = player_column_index(&FANTASY_PTS).unwrap();

        assert_eq!(30usize, index);
    }

    #[test]
    fn test_video_available_column() {
        let index = player_column_index(&VIDEO_AVAILABLE).unwrap();

        assert_eq!(31usize, index);
    }
}
