use warheads::format::extract::{get_result_set, get_rows};
use warheads::format::path_manager::{nba_correction_dir, nba_data_path, nba_team_correction_file};

use warheads::stats::nba_kind::NBAStatKind::Team;
use warheads::stats::season_period::minimum_spanning_era;

use warheads::stats::stat_column::team_column_index;
use warheads::stats::stat_column::StatColumn;

use warheads::types::{GameDate, GameId, SeasonId, TeamId};

use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::read_dir;

#[ignore = "dead_code"]
fn zip_dates() {
    for year in 1946..2025 {
        let periods = minimum_spanning_era(year);

        for p in periods {
            let mut corrections = corrections_as_values(p);

            let rows = rows_as_values(p);

            let mut game_dates = zip_games_and_dates(rows).expect("unable to zip games and dates");

            for correction in corrections.iter_mut() {
                let correction_object = correction
                    .as_object_mut()
                    .expect("unable to parse correction object");

                let correction_game_id =
                    GameId::from(correction_object.get("game_id").unwrap().as_str().unwrap());

                let date = game_dates.get_mut(&correction_game_id).unwrap();

                correction_object
                    .insert(String::from("game_date"), Value::String(date.to_string()));

                let season_id =
                    SeasonId::try_from(correction_object.get("season").unwrap()).unwrap();

                let game_id =
                    GameId::from(correction_object.get("game_id").unwrap().as_str().unwrap());

                let team_id =
                    TeamId::from(correction_object.get("team_id").unwrap().as_u64().unwrap());

                let correction_file = nba_team_correction_file(season_id, game_id, team_id);

                let output = serde_json::to_string_pretty(correction_object).expect("bing bong");

                println!("Output: {}", &output);

                let write_result = fs::write(&correction_file, output);

                if write_result.is_err() {
                    println!("Error writing correction file: {:?}", write_result);
                } else {
                    println!(
                        "Correction file written successfully: {}",
                        correction_file.display()
                    );
                }
            }
        }
    }
}

fn rows_as_values(s: SeasonId) -> Vec<Value> {
    let file = nba_data_path(s, Team);

    let contents = fs::read_to_string(file).expect("unable to read file provided in test. ");

    let value_blob = serde_json::from_str(&contents)
        .expect("couldn't get json object from sourced NBA json object");

    let set = get_result_set(&value_blob).expect("nba data bad");

    let rows = get_rows(&set).expect("nba data even badder");

    rows
}

fn corrections_as_values(s: SeasonId) -> Vec<Value> {
    let dir = read_dir(nba_correction_dir(s, Team)).expect("test directory doesnt exist?");

    let mut corrections = Vec::new();

    for entry in dir.filter_map(|x| x.ok()) {
        let contents = fs::read_to_string(entry.path()).expect("couldn't read or something. ");

        if let Ok(c) = serde_json::from_str(&contents) {
            corrections.push(c)
        }
    }

    corrections
}

fn zip_games_and_dates(rows: Vec<Value>) -> Result<HashMap<GameId, GameDate>, String> {
    let mut games_and_dates = HashMap::new();

    for row in rows {
        let arr = row.as_array().unwrap();

        let id = arr[team_column_index(&StatColumn::GAME_ID).unwrap()].clone();

        let game_id = GameId::from(id.as_str().unwrap());

        let date = arr[team_column_index(&StatColumn::GAME_DATE).unwrap()].clone();

        let game_date =
            GameDate(NaiveDate::parse_from_str(&date.as_str().unwrap(), "%Y-%m-%d").unwrap());

        games_and_dates.insert(game_id, game_date);
    }

    Ok(games_and_dates)
}
