#[cfg(test)]
mod tests {
    use crate::corrections::correction::Correction;
    use crate::stats::nba_kind::NBAStatKind;
    use crate::stats::season_type::SeasonPeriod;
    use crate::stats::stat_column::StatColumn;
    use crate::stats::stat_value::StatValue;
    use serde_json::json;
    use std::collections::HashMap;
    use crate::types::{GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};

    #[test]
    fn test_correct_wl() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::PTS, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_min() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::MIN, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fgm() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FGM, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fga() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FGA, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fg_pct() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FG_PCT, StatValue::from_value(json!(0.500000)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0.500000,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fg3m() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FG3M, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fg3a() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FG3A, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fg3_pct() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FG3_PCT, StatValue::from_value(json!(0.500000)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0.500000,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_ftm() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FTM, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_fta() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FTA, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_ft_pct() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::FT_PCT, StatValue::from_value(json!(0.500000)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0.500000,0,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_oreb() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::OREB, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_dreb() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::DREB, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_reb() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::REB, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_ast() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::AST, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_stl() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::STL, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_blk() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::BLK, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_tov() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::TOV, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_pf() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::PF, StatValue::from_value(json!(5)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_pts() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::PTS, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0,0]"#
        )
    }

    #[test]
    fn test_correct_plus_minus_negative() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::PLUS_MINUS, StatValue::from_value(json!(-10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-10,0,0]"#
        )
    }

    #[test]
    fn test_correct_plus_minus_positive() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::PLUS_MINUS, StatValue::from_value(json!(10)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,0,0]"#
        )
    }

    #[test]
    fn test_correct_fantasy_pts() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction.corrections.insert(
            StatColumn::FANTASY_PTS,
            StatValue::from_value(json!(69.420)),
        ); //assert that the value is being truncated properly

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,69.4,0]"#
        )
    }

    #[test]
    fn test_correct_video_available() {
        let data = raw_test_data();

        let mut correction = sample_correction();

        correction
            .corrections
            .insert(StatColumn::VIDEO_AVAILABLE, StatValue::from_value(json!(1)));

        assert_eq!(
            correction.correct_string(data),
            r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]"#
        )
    }

    //
    // helper functions
    //
    fn sample_correction() -> Correction {
        Correction {
            game_id: GameId("123".to_string()),
            season: SeasonId(20024),
            player_id: Some(PlayerId(23)),
            team_id: TeamId(151),
            team_abbr: TeamAbbreviation("LOL".to_string()),
            kind: NBAStatKind::Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections: HashMap::new(),
        }
    }

    fn raw_test_data() -> String {
        r#"["20024",23,"Tested McNutsack",151,"LOL","Los Orleans Losers","123","0024-03-31","LOL vs. DOT","W",0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]"#.to_string()
    }
}
