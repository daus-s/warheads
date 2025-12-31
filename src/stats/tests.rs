#[cfg(test)]
mod test_column_indices {
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

#[cfg(test)]
mod test_serialize_game_obj {
    use crate::stats::game_obj::GameObject;
    use crate::stats::season_period::SeasonPeriod;
    use crate::types::{GameDate, SeasonId, TeamId};

    #[test]
    fn test_deserialize_game_obj() {
        let expected_content = r#"
            {
              "season_id": "22005",
              "game_date": "2006-02-01",
              "game_id": "0020500673",
              "home": {
                "team_id": 1610612760,
                "team_abbreviation": "SEA",
                "team_name": "Seattle SuperSonics",
                "visiting": "Home",
                "roster": [
                  {
                    "player_id": 1740,
                    "player_name": "Rashard Lewis",
                    "box_score": {
                      "wl": "W",
                      "min": 34,
                      "fgm": 9,
                      "fga": 19,
                      "fg3m": 0,
                      "fg3a": 1,
                      "ftm": 8,
                      "fta": 8,
                      "oreb": 3,
                      "dreb": 3,
                      "reb": 6,
                      "ast": 4,
                      "stl": 2,
                      "blk": 1,
                      "tov": 0,
                      "pf": 2,
                      "pts": 26,
                      "plus_minus": 8
                    }
                  },
                  {
                    "player_id": 101130,
                    "player_name": "Johan Petro",
                    "box_score": {
                      "wl": "W",
                      "min": 15,
                      "fgm": 1,
                      "fga": 2,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 3,
                      "dreb": 2,
                      "reb": 5,
                      "ast": 0,
                      "stl": 0,
                      "blk": 1,
                      "tov": 2,
                      "pf": 5,
                      "pts": 2,
                      "plus_minus": 8
                    }
                  },
                  {
                    "player_id": 1630,
                    "player_name": "Mikki Moore",
                    "box_score": {
                      "wl": "W",
                      "min": 5,
                      "fgm": 0,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 3,
                      "fta": 4,
                      "oreb": 0,
                      "dreb": 0,
                      "reb": 0,
                      "ast": 2,
                      "stl": 0,
                      "blk": 0,
                      "tov": 0,
                      "pf": 1,
                      "pts": 3,
                      "plus_minus": 1
                    }
                  },
                  {
                    "player_id": 2557,
                    "player_name": "Luke Ridnour",
                    "box_score": {
                      "wl": "W",
                      "min": 39,
                      "fgm": 6,
                      "fga": 12,
                      "fg3m": 0,
                      "fg3a": 1,
                      "ftm": 5,
                      "fta": 6,
                      "oreb": 0,
                      "dreb": 2,
                      "reb": 2,
                      "ast": 6,
                      "stl": 0,
                      "blk": 0,
                      "tov": 4,
                      "pf": 3,
                      "pts": 17,
                      "plus_minus": 7
                    }
                  },
                  {
                    "player_id": 2043,
                    "player_name": "Mateen Cleaves",
                    "box_score": {
                      "wl": "W",
                      "min": 5,
                      "fgm": 0,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 1,
                      "reb": 1,
                      "ast": 1,
                      "stl": 0,
                      "blk": 0,
                      "tov": 0,
                      "pf": 1,
                      "pts": 0,
                      "plus_minus": -3
                    }
                  },
                  {
                    "player_id": 2209,
                    "player_name": "Vladimir Radmanovic",
                    "box_score": {
                      "wl": "W",
                      "min": 26,
                      "fgm": 5,
                      "fga": 10,
                      "fg3m": 0,
                      "fg3a": 2,
                      "ftm": 0,
                      "fta": 1,
                      "oreb": 2,
                      "dreb": 9,
                      "reb": 11,
                      "ast": 1,
                      "stl": 0,
                      "blk": 1,
                      "tov": 4,
                      "pf": 2,
                      "pts": 10,
                      "plus_minus": 12
                    }
                  },
                  {
                    "player_id": 2436,
                    "player_name": "Flip Murray",
                    "box_score": {
                      "wl": "W",
                      "min": 10,
                      "fgm": 2,
                      "fga": 4,
                      "fg3m": 1,
                      "fg3a": 1,
                      "ftm": 4,
                      "fta": 5,
                      "oreb": 1,
                      "dreb": 1,
                      "reb": 2,
                      "ast": 0,
                      "stl": 0,
                      "blk": 0,
                      "tov": 1,
                      "pf": 1,
                      "pts": 9,
                      "plus_minus": 0
                    }
                  },
                  {
                    "player_id": 2741,
                    "player_name": "Robert Swift",
                    "box_score": {
                      "wl": "W",
                      "min": 27,
                      "fgm": 4,
                      "fga": 7,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 2,
                      "fta": 4,
                      "oreb": 2,
                      "dreb": 7,
                      "reb": 9,
                      "ast": 0,
                      "stl": 0,
                      "blk": 1,
                      "tov": 0,
                      "pf": 5,
                      "pts": 10,
                      "plus_minus": -2
                    }
                  },
                  {
                    "player_id": 2555,
                    "player_name": "Nick Collison",
                    "box_score": {
                      "wl": "W",
                      "min": 22,
                      "fgm": 1,
                      "fga": 5,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 2,
                      "reb": 2,
                      "ast": 0,
                      "stl": 0,
                      "blk": 0,
                      "tov": 0,
                      "pf": 2,
                      "pts": 2,
                      "plus_minus": -8
                    }
                  },
                  {
                    "player_id": 2863,
                    "player_name": "Damien Wilkins",
                    "box_score": {
                      "wl": "W",
                      "min": 15,
                      "fgm": 0,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 2,
                      "reb": 2,
                      "ast": 1,
                      "stl": 1,
                      "blk": 0,
                      "tov": 1,
                      "pf": 2,
                      "pts": 0,
                      "plus_minus": -7
                    }
                  },
                  {
                    "player_id": 951,
                    "player_name": "Ray Allen",
                    "box_score": {
                      "wl": "W",
                      "min": 42,
                      "fgm": 12,
                      "fga": 20,
                      "fg3m": 4,
                      "fg3a": 10,
                      "ftm": 4,
                      "fta": 5,
                      "oreb": 1,
                      "dreb": 3,
                      "reb": 4,
                      "ast": 3,
                      "stl": 1,
                      "blk": 0,
                      "tov": 3,
                      "pf": 1,
                      "pts": 32,
                      "plus_minus": 4
                    }
                  }
                ],
                "box_score": {
                  "wl": "W",
                  "min": 240,
                  "fgm": 40,
                  "fga": 82,
                  "fg3m": 5,
                  "fg3a": 15,
                  "ftm": 26,
                  "fta": 33,
                  "oreb": 12,
                  "dreb": 32,
                  "reb": 44,
                  "ast": 18,
                  "stl": 4,
                  "blk": 4,
                  "tov": 15,
                  "pf": 25,
                  "pts": 111,
                  "plus_minus": 4
                }
              },
              "away": {
                "team_id": 1610612744,
                "team_abbreviation": "GSW",
                "team_name": "Golden State Warriors",
                "visiting": "Away",
                "roster": [
                  {
                    "player_id": 2554,
                    "player_name": "Mickael Pietrus",
                    "box_score": {
                      "wl": "L",
                      "min": 36,
                      "fgm": 5,
                      "fga": 11,
                      "fg3m": 3,
                      "fg3a": 5,
                      "ftm": 2,
                      "fta": 2,
                      "oreb": 0,
                      "dreb": 3,
                      "reb": 3,
                      "ast": 2,
                      "stl": 0,
                      "blk": 0,
                      "tov": 0,
                      "pf": 4,
                      "pts": 15,
                      "plus_minus": -9
                    }
                  },
                  {
                    "player_id": 101145,
                    "player_name": "Monta Ellis",
                    "box_score": {
                      "wl": "L",
                      "min": 13,
                      "fgm": 0,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 0,
                      "reb": 0,
                      "ast": 0,
                      "stl": 0,
                      "blk": 0,
                      "tov": 1,
                      "pf": 0,
                      "pts": 0,
                      "plus_minus": 3
                    }
                  },
                  {
                    "player_id": 2560,
                    "player_name": "Zarko Cabarkapa",
                    "box_score": {
                      "wl": "L",
                      "min": 3,
                      "fgm": 0,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 0,
                      "reb": 0,
                      "ast": 0,
                      "stl": 0,
                      "blk": 0,
                      "tov": 0,
                      "pf": 0,
                      "pts": 0,
                      "plus_minus": 1
                    }
                  },
                  {
                    "player_id": 1502,
                    "player_name": "Adonal Foyle",
                    "box_score": {
                      "wl": "L",
                      "min": 11,
                      "fgm": 1,
                      "fga": 1,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 1,
                      "fta": 1,
                      "oreb": 0,
                      "dreb": 1,
                      "reb": 1,
                      "ast": 0,
                      "stl": 0,
                      "blk": 1,
                      "tov": 2,
                      "pf": 1,
                      "pts": 3,
                      "plus_minus": -6
                    }
                  },
                  {
                    "player_id": 2211,
                    "player_name": "Troy Murphy",
                    "box_score": {
                      "wl": "L",
                      "min": 42,
                      "fgm": 5,
                      "fga": 14,
                      "fg3m": 2,
                      "fg3a": 4,
                      "ftm": 6,
                      "fta": 7,
                      "oreb": 5,
                      "dreb": 11,
                      "reb": 16,
                      "ast": 2,
                      "stl": 3,
                      "blk": 0,
                      "tov": 2,
                      "pf": 2,
                      "pts": 18,
                      "plus_minus": 4
                    }
                  },
                  {
                    "player_id": 2740,
                    "player_name": "Andris Biedrins",
                    "box_score": {
                      "wl": "L",
                      "min": 32,
                      "fgm": 7,
                      "fga": 10,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 2,
                      "fta": 5,
                      "oreb": 2,
                      "dreb": 6,
                      "reb": 8,
                      "ast": 3,
                      "stl": 1,
                      "blk": 0,
                      "tov": 1,
                      "pf": 4,
                      "pts": 16,
                      "plus_minus": -4
                    }
                  },
                  {
                    "player_id": 1884,
                    "player_name": "Baron Davis",
                    "box_score": {
                      "wl": "L",
                      "min": 42,
                      "fgm": 11,
                      "fga": 26,
                      "fg3m": 4,
                      "fg3a": 14,
                      "ftm": 9,
                      "fta": 13,
                      "oreb": 1,
                      "dreb": 2,
                      "reb": 3,
                      "ast": 9,
                      "stl": 1,
                      "blk": 0,
                      "tov": 5,
                      "pf": 4,
                      "pts": 35,
                      "plus_minus": -10
                    }
                  },
                  {
                    "player_id": 2399,
                    "player_name": "Mike Dunleavy",
                    "box_score": {
                      "wl": "L",
                      "min": 27,
                      "fgm": 1,
                      "fga": 7,
                      "fg3m": 0,
                      "fg3a": 2,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 2,
                      "dreb": 4,
                      "reb": 6,
                      "ast": 4,
                      "stl": 1,
                      "blk": 1,
                      "tov": 2,
                      "pf": 4,
                      "pts": 2,
                      "plus_minus": 4
                    }
                  },
                  {
                    "player_id": 965,
                    "player_name": "Derek Fisher",
                    "box_score": {
                      "wl": "L",
                      "min": 27,
                      "fgm": 7,
                      "fga": 11,
                      "fg3m": 2,
                      "fg3a": 4,
                      "ftm": 0,
                      "fta": 0,
                      "oreb": 0,
                      "dreb": 1,
                      "reb": 1,
                      "ast": 1,
                      "stl": 1,
                      "blk": 0,
                      "tov": 0,
                      "pf": 5,
                      "pts": 16,
                      "plus_minus": 3
                    }
                  },
                  {
                    "player_id": 101113,
                    "player_name": "Ike Diogu",
                    "box_score": {
                      "wl": "L",
                      "min": 7,
                      "fgm": 0,
                      "fga": 2,
                      "fg3m": 0,
                      "fg3a": 0,
                      "ftm": 2,
                      "fta": 2,
                      "oreb": 1,
                      "dreb": 1,
                      "reb": 2,
                      "ast": 0,
                      "stl": 1,
                      "blk": 0,
                      "tov": 0,
                      "pf": 1,
                      "pts": 2,
                      "plus_minus": -6
                    }
                  }
                ],
                "box_score": {
                  "wl": "L",
                  "min": 240,
                  "fgm": 37,
                  "fga": 84,
                  "fg3m": 11,
                  "fg3a": 29,
                  "ftm": 22,
                  "fta": 30,
                  "oreb": 11,
                  "dreb": 29,
                  "reb": 40,
                  "ast": 21,
                  "stl": 8,
                  "blk": 2,
                  "tov": 14,
                  "pf": 25,
                  "pts": 107,
                  "plus_minus": -4
                }
              }
            }
"#;

        let deserialized_object: GameObject = serde_json::from_str(&expected_content)
            .expect("💀 failed to parse expected json string. ");

        dbg!(&deserialized_object);

        assert_eq!(
            deserialized_object.season(),
            SeasonId::from((2005, SeasonPeriod::RegularSeason))
        );

        assert!(deserialized_object.had_participant(TeamId(1610612760)));
        assert!(deserialized_object.had_participant(TeamId(1610612744)));

        assert_eq!(
            "2006-02-01".parse::<GameDate>().unwrap(),
            deserialized_object.game_date
        );
    }
}
