use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde_json::{json, Value};
use serde_json::Value::Number;
use warheads::corrections::correction::Correction;
use warheads::corrections::correction_loader::load_corrections;
use warheads::corrections::corrector::Corrector;
use warheads::format::extract::{json_to_hashmap};
use warheads::format::language::{partition, Columnizable};
use warheads::format::path_manager::nba_correction_dir;
use warheads::stats::id::Identifiable;
use warheads::stats::nba_kind::NBAStatKind;
use warheads::stats::nba_kind::NBAStatKind::Player;
use warheads::stats::se::SerdeEnum;
use warheads::stats::season_type::SeasonPeriod;
use warheads::stats::season_type::SeasonPeriod::RegularSeason;
use warheads::stats::stat_column::{player_column_index, StatColumn};
use warheads::stats::stat_column::StatColumn::*;
use warheads::stats::stat_value::StatValue;
use warheads::stats::types::GameResult::{Loss, Win};
use pretty_assertions::{assert_eq};

#[test]
pub fn test_load_correction() {

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

                cs.insert(WL, StatValue::from_value(Loss.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Win.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Win.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Loss.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Win.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Loss.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Win.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Loss.evaluate()));

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

                cs.insert(WL, StatValue::from_value(Loss.evaluate()));

                cs
            },
        }
    ]
}

#[test]
fn test_apply_corrections() {
    let corrections = vec![
        Correction {
            game_id: "12345678".to_string(),
            season: 24,
            player_id: Some(69420),
            team_id: 32768,
            team_abbr: "BOM".to_string(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections:  HashMap::from([
                (FG3M, StatValue::from_value(json!(2))),
            ]),
        },
        Correction {
            game_id: "12345678".to_string(),
            season: 24,
            player_id: Some(14141),
            team_id: 32768,
            team_abbr: "BOM".to_string(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections:  HashMap::from([
                (FGM, StatValue::from_value(json!(6))),
                (FG3M, StatValue::from_value(json!(3))),
            ]),
        },
        Correction {
            game_id: "11235813".to_string(),
            season: 24,
            player_id: Some(69420),
            team_id: 32768,
            team_abbr: "BOM".to_string(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: false,
            corrections:  HashMap::from([
                (FG_PCT, StatValue::from_value(json!( 3f32 / 7f32))),
            ]),
        },
        Correction {
            game_id: "11235813".to_string(),
            season: 24,
            player_id: Some(66666),
            team_id: 16384,
            team_abbr: "TRA".to_string(),
            kind: Player,
            period: SeasonPeriod::RegularSeason,
            delete: true,
            corrections:  HashMap::new(),
        },
    ];

    let contents = bad_data();

    let value = serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse JSON from test file wtf: {e}"));


    let mut games_by_id = json_to_hashmap(&value)
        .unwrap_or_else(|e|
            panic!("failed to create hashmap of identity -> string \n\
                    from the file `tests/data/data.json`: {e}"));

    // let game_list = get_rows_from_file(PathBuf::from("tests/data/data.json"))?;

    let mut to_remove = Vec::new();

    for correction in corrections {

        let id = correction.identity();

        if let Some(game) = games_by_id.get(&id) {
            if correction.delete {
                to_remove.push(id.clone());
            } else {
                games_by_id.insert(id, correction.correct(game.clone()));
            }
        }
    }


    for deletion in to_remove {
        games_by_id.remove(&deletion);
    }

    // dbg!(&games_by_id);

    let mut games= games_by_id.into_values().collect::<Vec<String>>();

    games.sort();

    for game in games.iter() {
        eprintln!("        {}", game);
    }

    let expected = expected_file().to_string();

    let corrected = partition(contents, games);
    //
    eprintln!("Corrected ==========================================================================\n{}\n====================================================================================", &corrected);

    // eprintln!("Expected ===========================================================================\n{}\n====================================================================================", &expected);


    // eprintln!("corrected chars: {}\nexpected chars: {}", corrected.len(), expected.len());
    //
    // let mut i: usize = 0;
    //
    // let mut exp_chars = expected.chars();
    //
    // let mut cor_chars = expected.chars();
    //

    // loop {
    //     if i == 1277 {
    //         break;
    //     }
    //
    //     if exp_chars.nth(i) != cor_chars.nth(i) {
    //         eprintln!("different characters occur at index {i}.");
    //     }
    //     else {
    //         eprintln!("ok ({i}/1276)");
    //     }
    //
    //     i = i + 1;
    // }
    //

    assert_eq!(expected, corrected)
}

fn bad_data() -> String {
    fs::read_to_string("tests/data/readable_data.json").unwrap()

}

fn expected_file() -> String {
    fs::read_to_string("tests/data/readable_corrected.json").unwrap()
}