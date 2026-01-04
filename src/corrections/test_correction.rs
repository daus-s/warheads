#[cfg(test)]
mod test_load_season_corrections {
    use crate::corrections::correction::Correction;
    use crate::corrections::correction_loader::load_season_corrections;
    use crate::format::path_manager::nba_correction_dir;
    use crate::stats::nba_kind::NBAStatKind::Player;
    use crate::stats::serde_enum::SerdeEnum;
    use crate::stats::stat_column::StatColumn;
    use crate::stats::stat_column::StatColumn::*;
    use crate::types::{
        GameDate, GameId,
        GameResult::{Loss, Win},
        PlayerId, SeasonId, TeamAbbreviation, TeamId,
    };
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;
    use serde_json::Value;
    use std::collections::HashMap;
    use std::str::FromStr;

    #[test]
    pub fn test_load_correction() {
        let season_id = SeasonId::from(21959);

        let kind = Player;

        let mut expected_corrections = expected_corrections();

        let mut actual_corrections = load_season_corrections(season_id.year(), kind)
            .unwrap_or_else(|e| {
                panic!(
                    "Failed to load corrections from: {}\n{e}",
                    nba_correction_dir(season_id, kind)
                )
            });

        expected_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));
        actual_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));

        assert_eq!(actual_corrections, expected_corrections);
    }

    fn expected_corrections() -> Vec<Correction> {
        let season_id = SeasonId::from(21959);
        let postseason_id = SeasonId::from(41959);
        let kind = Player;

        vec![
            Correction {
                game_id: GameId::from("0025900249"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 02, 18).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(76160)),
                team_id: TeamId(1610612744),
                team_abbr: TeamAbbreviation::from_str("PHW").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900179"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 1, 19).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(77131)),
                team_id: TeamId::from(1610612744),
                team_abbr: TeamAbbreviation::from_str("PHW").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Win.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900010"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1959, 10, 31).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(76136)),
                team_id: TeamId::from(1610612744),
                team_abbr: TeamAbbreviation::from_str("PHW").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Win.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900033"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1959, 11, 12).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(78013)),
                team_id: TeamId::from(1610612758),
                team_abbr: TeamAbbreviation::from_str("CIN").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900033"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1959, 11, 12).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(78040)),
                team_id: TeamId::from(1610612744),
                team_abbr: TeamAbbreviation::from_str("PHW").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Win.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900079"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1959, 12, 03).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(78216)),
                team_id: TeamId::from(1610612747),
                team_abbr: TeamAbbreviation::from_str("MNL").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900080"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1959, 12, 04).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(78223)),
                team_id: TeamId::from(1610612744),
                team_abbr: TeamAbbreviation::from_str("PHW").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Win.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900207"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 2, 01).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(76658)),
                team_id: TeamId::from(1610612747),
                team_abbr: TeamAbbreviation::from_str("MNL").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900253"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 02, 20).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(76783)),
                team_id: TeamId::from(1610612755),
                team_abbr: TeamAbbreviation::from_str("SYR").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0025900257"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 02, 21).unwrap()),
                season: season_id,
                player_id: Some(PlayerId(76783)),
                team_id: TeamId::from(1610612752),
                team_abbr: TeamAbbreviation::from_str("NYK").unwrap(),
                kind,
                period: season_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Loss.evaluate());

                    cs
                },
            },
            Correction {
                game_id: GameId::from("0045900321"),
                game_date: GameDate(NaiveDate::from_ymd_opt(1960, 03, 16).unwrap()),
                season: postseason_id,
                player_id: Some(PlayerId(77954)),
                team_id: TeamId::from(1610612738),
                team_abbr: "BOS".parse().unwrap(),
                kind: kind,
                period: postseason_id.period(),
                delete: false,
                corrections: {
                    let mut cs: HashMap<StatColumn, Value> = HashMap::new();

                    cs.insert(WL, Win.evaluate());

                    cs
                },
            },
        ]
    }
}
