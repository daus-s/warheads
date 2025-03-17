use crate::gather::read_nba;
use corrections::correction::Correction;
use serde_json::{from_str, Value};
use stats::extract::{get_set, headers, rows};
use stats::kind::NBAStatKind::{LineUp, Player, Team};
use stats::kind::{NBAStat, NBAStatKind};
use stats::nba::GameResult::{Draw, Loss, Win};
use stats::nba::GameResult;
use stats::player_box_score::{PlayerBoxScore, PlayerBoxScoreBuilder};
use stats::stat_column::StatColumn::{GAME_DATE, WL};
use stats::stat_value::StatValue;
use stats::team_box_score::{TeamBoxScore, TeamBoxScoreBuilder};
use std::collections::HashMap;
use time::{macros::format_description, Date};
//rips through the json using the header provided as per NBA apis convention/schema.
//output the file to a (headed) csv to match the output format we will be using.

pub fn process_nba_games(szn: i32, stat: NBAStatKind) -> Result<Vec<NBAStat>, Vec<Correction>> {
    let json = &read_nba(szn, stat);

    let v: Value = from_str(json).unwrap();

    let set = get_set(&v).unwrap();

    let headers: Vec<&str> = headers(&set).unwrap();

    let rows: Vec<Value> = rows(&set).unwrap();

    season(rows, headers, stat)
}

fn fields_to_team_box_score(s: &HashMap<String, Value>) -> Result<TeamBoxScore, Correction> {

    let gameid = string(s.get("GAME_ID"));
    let teamid = parse_u64(s.get("TEAM_ID")).unwrap();
    let season = str_to_num(s.get("SEASON_ID")) as i32;

    let mut correction = Correction::new(gameid.clone(), teamid, season, Team);


    let wl= parse_wl(s.get("WL"));

    match wl {
        Some(_) => (),
        None => correction.add_missing_field(WL ,StatValue::new())
    }

    let dt = parse_date(s.get("GAME_DATE"));

    match dt {
        Some(_) => (),
        None => correction.add_missing_field(GAME_DATE, StatValue::new())
    }

    match correction.len() {
        0 => Ok(
            TeamBoxScoreBuilder::default()
            .game_id(gameid.clone())
            .season_id(season)
            .team_id(teamid)
            .ast(parse_u32(s.get("AST")))
            .plus_minus(parse_i32(s.get("PLUS_MINUS")))
            .reb(parse_u32(s.get("REB")))
            .min(parse_u32(s.get("MIN")))
            .wl(wl.unwrap())
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
            .game_date(dt.unwrap())
            .team_abbreviation(s.get("TEAM_ABBREVIATION").unwrap().as_str().unwrap().to_string())
            .matchup(stats::nba::MatchupString(s.get("MATCHUP").unwrap().as_str().unwrap().to_string()))
            .roster(Vec::new())
            .build().unwrap()
        ),
        _ => Err(correction),
    }

}


///
/// fields_to_player_box_score returns a result of either a player box score or a correction.
/// if the function returns a correction, the correction acts as a form that needs to be
/// completed before that entry can be finalized. as such, seemingly inconsequentially,
/// the player stats must always be ripped from file before team results.
///
fn fields_to_player_box_score(s: &HashMap<String, Value>) -> Result<PlayerBoxScore, Correction> {

    let gameid = string(s.get("GAME_ID"));
    let playerid = parse_u64(s.get("PLAYER_ID")).unwrap();
    let season = str_to_num(s.get("SEASON_ID")) as i32;

    let mut correction = Correction::new(gameid.clone(), playerid, season, Player);

    let wl = parse_wl(s.get("WL"));

    match wl {
        Some(_) => (),
        None => correction.add_missing_field(WL, StatValue::new()),
    }

    let dt = parse_date(s.get("GAME_DATE"));

    match dt {
        Some(_) => (),
        None => correction.add_missing_field(GAME_DATE, StatValue::new())
    }

    match correction.len() {
        0 => Ok(PlayerBoxScoreBuilder::default()
            .ast(parse_u32(s.get("AST")))
            .plus_minus(parse_i32(s.get("PLUS_MINUS")))
            .season_id(season)
            .game_id(gameid.clone())
            .reb(parse_u32(s.get("REB")))
            .min(parse_u32(s.get("MIN")))
            .wl(wl.unwrap())
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
            .game_date(dt.unwrap())
            .team_abbreviation(s.get("TEAM_ABBREVIATION").unwrap().as_str().unwrap().to_string())
            .matchup(stats::nba::MatchupString(string(s.get("MATCHUP"))))
            .player_name(string(s.get("PLAYER_NAME")))
            .player_id(playerid)
            .team_id(parse_u64(s.get("TEAM_ID")).unwrap())
            .elo(3000)
            .build().unwrap()),
        _ => Err(correction),
    }


}

fn string(s: Option<&Value>) -> String {
    s.unwrap().to_string()
}

fn parse_u64(value: Option<&Value>) -> Option<u64> {
    match value {
        Some(v) => match v.as_u64() {
            Some(x) => Some(x),
            None => {
                None
            }
        },
        None => {
            None
        }
    }
}

fn parse_u32(value: Option<&Value>) -> Option<u32> {
    match value {
        Some(v) => match v.as_u64() {
            Some(x) => {
                if x <= u32::MAX as u64 {
                    Some(x as u32)
                } else {
                    None
                }
            }
            None => {
                None
            }
        },
        None => {
            None
        }
    }
}

fn parse_i32(value: Option<&Value>) -> Option<i32> {
    match value {
        Some(v) => match v.as_i64() {
            Some(x) => {
                if x >= i32::MIN as i64 && x <= i32::MAX as i64 {
                    Some(x as i32)
                } else {
                    None
                }
            }
            None => {
                None
            }
        },
        None => {
            None
        }
    }
}

fn parse_f32(value: Option<&Value>) -> Option<f32> {
    match value {
        Some(v) =>
            match v.as_f64() {
                Some(x) => {

                    Some(x as f32)
                },
                None => {
                    None
                },
            },
        None => {
            None
        }
    }
}

fn str_to_num(value: Option<&Value>) -> u64 {
    value.unwrap().as_str().unwrap().parse::<u64>().expect(format!("could not parse {:?} into unsigned 64-bit integer\n", value ).as_str())
}

fn parse_wl(value: Option<&Value>) -> Option<GameResult> {
    match value { // the fuck?
        Some(wl) => match wl.as_str() {
            Some("W") => Some(Win),
            Some("L") => Some(Loss),
            Some("D") => Some(Draw),
            Some(x) => panic!(
                "Unknown game result: {}. Acceptable results are: [\"W\", \"L\", \"D\"]",
                x
            ),
            None => None,
        }
        None => panic!("could not unwrap a game result from the provided serde::Value {:#?}", value)
    }
}

fn parse_date(value: Option<&Value>) -> Option<Date> {

    // Define the format for the date string
    let format = format_description!("[year]-[month]-[day]");


    let d = value.unwrap().as_str();

    match d {
        Some(date_str) => match Date::parse(date_str, &format) {
            Ok(date) => Some(date), // Parse the string into a `Date`
            Err(_) => None,
        },
        None => None
    }
}



fn season(rows: Vec<Value>, headers: Vec<&str>, stat: NBAStatKind) -> Result<Vec<NBAStat>, Vec<Correction>> {
    let mut season:Vec<NBAStat> = Vec::new();
    let mut corrections: Vec<Correction> = Vec::new();

    for row in rows {
        if let Some(row_data) = row.as_array() {
            let fields: HashMap<String, Value> = headers.iter()
                .zip(row_data.iter())
                .map(|(name, value)  |(name.to_string(), value.clone()))
                .collect();

            match stat {
                Player => match fields_to_player_box_score(&fields) {
                    Ok(box_score) => {
                        season.push(NBAStat::Player(box_score));
                    },
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                Team => match fields_to_team_box_score(&fields) {
                    Ok(box_score) => {
                        season.push(NBAStat::Team(box_score));
                    },
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                LineUp => panic!("lineup stats are not yet supported.")
            };
        }
    }

    if corrections.len() == 0 {
        Ok(season)
    } else {
        Err(corrections)
    }
}

