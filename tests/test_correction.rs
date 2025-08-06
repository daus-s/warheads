use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use chrono::NaiveDate;
use warheads::corrections::correction::Correction;
use warheads::corrections::correction_loader::load_corrections;
use warheads::corrections::corrector::Corrector;
use warheads::dapi::extract::json_to_hashmap;
use warheads::format::language::partition;
use warheads::format::path_manager::nba_correction_dir;
use warheads::stats::id::Identifiable;
use warheads::stats::nba_kind::NBAStatKind;
use warheads::stats::nba_kind::NBAStatKind::Player;
use warheads::stats::se::SerdeEnum;
use warheads::stats::season_period::SeasonPeriod;
use warheads::stats::stat_column::StatColumn;
use warheads::stats::stat_column::StatColumn::*;
use warheads::stats::stat_value::StatValue;
use warheads::types::GameResult::{Loss, Win};
use warheads::types::{GameDate, GameId, PlayerId, SeasonId, TeamAbbreviation, TeamId};

#[test]
pub fn test_load_correction() {
    let season_id = SeasonId::from(21959);

    let kind = Player;

    let mut expected_corrections = expected_corrections(season_id, kind);

    let mut actual_corrections = load_corrections(season_id.year(), kind).unwrap_or_else(|e| {
        panic!(
            "Failed to load corrections from: {}\n{e}",
            nba_correction_dir(season_id, kind)
        )
    });

    expected_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));
    actual_corrections.sort_by_key(|c| (c.game_id.clone(), c.player_id));

    assert_eq!(actual_corrections, expected_corrections);
}

fn expected_corrections(season_id: SeasonId, kind: NBAStatKind) -> Vec<Correction> {
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
            game_date: GameDate(NaiveDate::from_ymd_opt(1959,11,12).unwrap()),
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
            game_date: GameDate(NaiveDate::from_ymd_opt(1959,11,12).unwrap()),
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
            game_date: GameDate(NaiveDate::from_ymd_opt(1959,12,03).unwrap()),
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
            delete: true,
            corrections: Default::default(),
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
    ]
}

#[test]
fn test_apply_corrections() {
    let corrections = vec![
        Correction {
            game_id: GameId::from("12345678"),
            game_date: GameDate(Default::default()),
            season: SeasonId::from(20024),
            player_id: Some(PlayerId(69420)),
            team_id: TeamId(32768),
            team_abbr: TeamAbbreviation::from_str("BOM").unwrap(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections: HashMap::from([(FG3M, json!(2))]),
        },
        Correction {
            game_id: GameId::from("12345678"),
            game_date: GameDate(Default::default()),
            season: SeasonId::from(20024),
            player_id: Some(PlayerId(14141)),
            team_id: TeamId(32768),
            team_abbr: TeamAbbreviation::from_str("BOM").unwrap(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections: HashMap::from([
                (FGM, json!(6)),
                (FG3M, json!(3)),
            ]),
        },
        Correction {
            game_id: GameId::from("11235813"),
            game_date: GameDate(Default::default()),
            season: SeasonId::from(20024),
            player_id: Some(PlayerId(69420)),
            team_id: TeamId(32768),
            team_abbr: TeamAbbreviation::from_str("BOM").unwrap(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections: HashMap::from([(FG_PCT, json!(3f32 / 7f32))]),
        },
        Correction {
            game_id: GameId::from("11235813"),
            game_date: GameDate(Default::default()),
            season: SeasonId::from(20024),
            player_id: Some(PlayerId(66666)),
            team_id: TeamId(16384),
            team_abbr: TeamAbbreviation::from_str("TRA").unwrap(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: true,
            corrections: HashMap::new(),
        },
    ];

    let contents = bad_data();

    let mut daps = HashMap::new();

    let domain = (SeasonId::from(20024), Player);

    daps.insert(domain, contents);

    match corrections.apply(&mut daps) {
        Ok(_) => {
            println!("success");
        }
        Err(_) => {
            println!("failure");
        }
    };

    let expected = expected_file();

    let corrected = daps.get(&domain).unwrap().to_string();

    assert_eq!(expected, corrected)
}

fn bad_data() -> String {
    fs::read_to_string("tests/data/data.json").unwrap()
}

fn expected_file() -> String {
    fs::read_to_string("tests/data/corrected.json").unwrap()
}
