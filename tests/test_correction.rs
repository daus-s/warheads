use std::collections::HashMap;
use warheads::corrections::correction::Correction;
use warheads::corrections::correction_loader::load_corrections;
use warheads::format::path_manager::nba_correction_dir;
use warheads::stats::nba_kind::NBAStatKind;
use warheads::stats::nba_kind::NBAStatKind::Player;
use warheads::stats::se::SerdeEnum;
use warheads::stats::season_type::SeasonPeriod;
use warheads::stats::season_type::SeasonPeriod::RegularSeason;
use warheads::stats::stat_column::StatColumn;
use warheads::stats::stat_column::StatColumn::*;
use warheads::stats::stat_value::StatValue;
use warheads::stats::types::GameResult::{Loss, Win};

#[test]
pub fn test_correction() {

    let year = 1959;
    let season = year + 20000;
    let kind = Player;
    let period = RegularSeason;

    let mut expected_corrections = expected_corrections(season, kind, period);

    let mut actual_corrections = load_corrections(year, kind, period)
        .unwrap_or_else(|e| panic!(
            "Failed to load corrections from: {}\n{e}",
            nba_correction_dir(year, kind, period)
        ));

    expected_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));
    actual_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));

    assert_eq!(actual_corrections, expected_corrections);
}

fn expected_corrections(season: i32, kind: NBAStatKind, period: SeasonPeriod) -> Vec<Correction> {
    vec![
        Correction {
            game_id: "0025900249".to_string(),
            season,
            player_id: Some(76160),
            team_id: 1610612744,
            team_abbr: "PHW".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Loss.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900179".to_string(),
            season,
            player_id: Some(77131),
            team_id: 1610612744,
            team_abbr: "PHW".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Win.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900010".to_string(),
            season,
            player_id: Some(76136),
            team_id: 1610612744,
            team_abbr: "PHW".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Win.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900033".to_string(),
            season,
            player_id: Some(78013),
            team_id: 1610612758,
            team_abbr: "CIN".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Loss.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900033".to_string(),
            season,
            player_id: Some(78040),
            team_id: 1610612744,
            team_abbr: "PHW".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Win.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900079".to_string(),
            season,
            player_id: Some(78216),
            team_id: 1610612747,
            team_abbr: "MNL".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Loss.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900080".to_string(),
            season,
            player_id: Some(78223),
            team_id: 1610612744,
            team_abbr: "PHW".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Win.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900207".to_string(),
            season,
            player_id: Some(76658),
            team_id: 1610612747,
            team_abbr: "MNL".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Loss.evaluate()));

                cs
            },
        },
        Correction {
            game_id: "0025900253".to_string(),
            season,
            player_id: Some(76783),
            team_id: 1610612755,
            team_abbr: "SYR".to_string(),
            kind,
            period,
            delete: true,
            corrections: Default::default(),
        },
        Correction {
            game_id: "0025900257".to_string(),
            season,
            player_id: Some(76783),
            team_id: 1610612752,
            team_abbr: "NYK".to_string(),
            kind,
            period,
            delete: false,
            corrections: {
                let mut cs: HashMap<StatColumn, StatValue> = HashMap::new();

                cs.insert(WL, StatValue::with_value(Loss.evaluate()));

                cs
            },
        }
    ]
}