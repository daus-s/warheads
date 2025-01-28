use std::collections::{HashMap};
use serde_json::{from_str, Value};
use time::{Date, macros::format_description};
use stats::nba::{GameResult, PlayerBoxScore, PlayerBoxScoreBuilder, TeamBoxScore, TeamBoxScoreBuilder};
use stats::nba::GameResult::{Draw, Loss, Win};
use stats::kind::{NBAStatKind, NBAStatType};
use stats::kind::NBAStatKind::{Player, LineUp, Team};
//rips through the json using the header provided as per NBA apis convention/schema.
//output the file to a (headed) csv to match the pff outputs we will be using.

pub fn process_nba_games(json: &str, stat: NBAStatKind) -> Result<Vec<NBAStatType>, &'static str> {
    let v: Value = from_str(json).unwrap();

    let set = get_set(&v)?;

    let headers = headers(&set)?;

    let rows = rows(&set)?;

    Ok(season(rows, headers,  stat))
}

fn fields_to_team_box_score(s: &HashMap<String, Value>) -> TeamBoxScore {

    let box_score = TeamBoxScoreBuilder::default()
        .ast(parse_u32(s.get("AST")))
        .plus_minus(parse_i32(s.get("PLUS_MINUS")))
        .season_id(parse_str(s.get("SEASON_ID")) as u32)
        .game_id(parse_str(s.get("GAME_ID")))
        .reb(parse_u32(s.get("REB")))
        .min(parse_u32(s.get("MIN")))
        .wl(parse_wl(s.get("WL")))
        .team_name(s.get("TEAM_NAME").unwrap().as_str().unwrap().to_string())
        .dreb(parse_u32(s.get("DREB")))
        .oreb(parse_u32(s.get("OREB")))
        .stl(parse_u32(s.get("STL")))
        .blk(parse_u32(s.get("BLK")))
        .fg3a(parse_u32(s.get("FG3A")))
        .fg3m(parse_u32(s.get("FG3M")))
        .fga(parse_u32(s.get("FGA")))
        .fgm(parse_u32(s.get("FGM")))
        .fta(parse_u32(s.get("FTA")))
        .ftm(parse_u32(s.get("FTM")))
        .tov(parse_u32(s.get("TOV")))
        .pts(parse_u32(s.get("PTS")))
        .pf(parse_u32(s.get("PF")))
        .game_date(parse_date(s.get("GAME_DATE")))
        .team_abbreviation(s.get("TEAM_ABBREVIATION").unwrap().as_str().unwrap().to_string())
        .matchup(s.get("MATCHUP").unwrap().as_str().unwrap().to_string())
        .team_id(parse_u64(s.get("TEAM_ID")))
        .build()
        .unwrap();

    box_score

}

fn fields_to_player_box_score(s: &HashMap<String, Value>) -> PlayerBoxScore {

    let box_score = PlayerBoxScoreBuilder::default()
        .ast(parse_u32(s.get("AST")))
        .plus_minus(parse_i32(s.get("PLUS_MINUS")))
        .season_id(parse_str(s.get("SEASON_ID")) as u32)
        .game_id(parse_str(s.get("GAME_ID")))
        .reb(parse_u32(s.get("REB")))
        .min(parse_u32(s.get("MIN")))
        .wl(parse_wl(s.get("WL")))
        .team_name(s.get("TEAM_NAME").unwrap().as_str().unwrap().to_string())
        .dreb(parse_u32(s.get("DREB")))
        .oreb(parse_u32(s.get("OREB")))
        .stl(parse_u32(s.get("STL")))
        .blk(parse_u32(s.get("BLK")))
        .fg3a(parse_u32(s.get("FG3A")))
        .fg3m(parse_u32(s.get("FG3M")))
        .fga(parse_u32(s.get("FGA")))
        .fgm(parse_u32(s.get("FGM")))
        .fta(parse_u32(s.get("FTA")))
        .ftm(parse_u32(s.get("FTM")))
        .tov(parse_u32(s.get("TOV")))
        .pts(parse_u32(s.get("PTS")))
        .pf(parse_u32(s.get("PF")))
        .fantasy_pts(parse_f32(s.get("FANTASY_PTS")))
        .game_date(parse_date(s.get("GAME_DATE")))
        .team_abbreviation(s.get("TEAM_ABBREVIATION").unwrap().as_str().unwrap().to_string())
        .matchup(string(s.get("MATCHUP")))
        .player_name(string(s.get("PLAYER_NAME")))
        .player_id(parse_u64(s.get("PLAYER_ID")))
        .team_id(parse_u64(s.get("TEAM_ID")))
        .build()
        .unwrap();

    box_score

}

fn string(s: Option<&Value>) -> String {
    s.unwrap().as_str().unwrap().to_string()
}
fn parse_u64(value: Option<&Value>) -> u64 {
    value.unwrap().as_u64().expect(format!("could not parse {:?} as a unsigned 64-bit integer\n", value ).as_str())
}

fn parse_u32(value: Option<&Value>) -> u32 {
    value.unwrap().as_u64().expect(format!("could not parse {:?} as a unsigned 32-bit integer\n", value ).as_str()) as u32
}

fn parse_i32(value: Option<&Value>) -> i32 {
    value.unwrap().as_i64().expect(format!("could not parse {:?} as a signed 32-bit integer\n", value ).as_str()) as i32
}

fn parse_f32(value: Option<&Value>) -> f32 {
    value.unwrap().as_f64().expect(format!("could not parse {:?} as a 64-bit floating point number\n", value ).as_str()) as f32
}

fn parse_str(value: Option<&Value>) -> u64 {
    value.unwrap().as_str().unwrap().parse::<u64>().expect(format!("could not parse {:?} into unsigned 64-bit integer\n", value ).as_str())
}

fn parse_wl(value: Option<&Value>) -> GameResult {
    match value.unwrap().as_str().unwrap() {
        "W" => Win,
        "L" => Loss,
        "D" => Draw,
        x => panic!(
            "Unknown game result: {}. Acceptable results are: [\"W\", \"L\", \"D\"]",
            x
        ),
    }
}

fn parse_date(value: Option<&Value>) -> Date {

    let date_str = value.unwrap().as_str().unwrap();

    // Define the format for the date string
    let format = format_description!("[year]-[month]-[day]");

    // Parse the string into a `Date`
    let date = Date::parse(date_str, &format).expect("Failed to parse date");

    date
}

fn get_set(v: &Value) -> Result<Value, &'static str> {
    let result_sets = v.get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets.get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    Ok(result_set.clone())
}

fn headers(s: &Value) -> Result<Vec<&str>, &'static str> {
    Ok(
        s.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field")?.iter().filter_map(|h| h.as_str()).collect()
    )
}

fn rows(set: &Value ) -> Result<Vec<Value>, &'static str> {
    Ok(
        set.get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?.clone()
    )
}

fn season(rows: Vec<Value>, headers: Vec<&str>, stat: NBAStatKind) -> Vec<NBAStatType> {
    let mut season:Vec<NBAStatType> = Vec::new();

    for row in rows {
        if let Some(row_data) = row.as_array() {
            let fields: HashMap<String, Value> = headers.iter()
                .zip(row_data.iter())
                .map(|(name, value)  |(name.to_string(), value.clone()))
                .collect();

            let box_score = match stat {
                Player => NBAStatType::Player(fields_to_player_box_score(&fields)),
                Team => NBAStatType::Team(fields_to_team_box_score(&fields)),
                LineUp => panic!("lineup stats are not yet supported.")
            };


            season.push(box_score); //this means mutable reference
        }
    }

    season
}