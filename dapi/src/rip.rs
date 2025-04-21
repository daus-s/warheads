use crate::gather::read_nba_file;
use crate::parse::*;
use corrections::correction::Correction;
use corrections::correction_builder::CorrectionBuilder;
use corrections::corrector::Corrector;
use serde_json::Value::Null;
use serde_json::{from_str, Value};
use stats::extract::{get_result_set, headers, rows};
use stats::game_info::GameInfo;
use stats::nba_kind::NBAStatKind;
use stats::nba_kind::NBAStatKind::{LineUp, Player, Team};
use stats::nba_stat::NBAStat;
use stats::player_box_score::{PlayerBoxScore, PlayerBoxScoreBuilder};
use stats::season_type::SeasonPeriod;
use stats::stat_column::StatColumn::{GAME_DATE, MATCHUP, PLAYER_NAME, TEAM_ID, TEAM_NAME, WL};
use stats::stat_value::StatValue;
use stats::team_box_score::{TeamBoxScore, TeamBoxScoreBuilder};
use stats::types::MatchupString;
use std::collections::HashMap;
use format::path_manager::data_path;

pub(crate) fn fetch_and_process_nba_games(
    year: i32,
    stat: NBAStatKind,
    period: SeasonPeriod,
) -> Vec<NBAStat> {
    match process_nba_games(year, stat, period) {
        Ok(games) => games,

        // handle corrections, maybe use something other than `result` in the future
        Err(corrections_meta) => {
            let corrections: Vec<Correction> = corrections_meta
                .into_iter()
                .map(|(corr, info)| CorrectionBuilder::new(corr, info).create())
                .collect();

            corrections.iter().for_each(|c| {
                let _ = c.save(); // we assume t that saving to the file is allowed and disregard
                                  // the result. todo: add more robust error handling here.
            });

            corrections
                .apply()
                .map(|_| fetch_and_process_nba_games(year, stat, period))
                .unwrap_or_else(|e| panic!("failed to apply corrections: {}", e))
        }
    }
}

///
/// rips through the json using the header provided as per NBA apis convention/schema.
/// return a Result of Ok(Vec<NBAStat>) or Err(Vec<Correction>). it is important to remember
/// an NBAStat is a BoxScore.
///
/// process games will crash if the JSON is poorly shaped.
///

fn process_nba_games(
    year: i32,
    stat: NBAStatKind,
    period: SeasonPeriod,
) -> Result<Vec<NBAStat>, Vec<(Correction, GameInfo)>> {
    let file_path = data_path(year, stat, period);

    let json = &read_nba_file(file_path);

    let (rows, headers) = parse_season(from_str(json).unwrap());

    season(rows, headers, stat, period)
}

fn season(
    rows: Vec<Value>,
    headers: Vec<String>,
    stat: NBAStatKind,
    period: SeasonPeriod,
) -> Result<Vec<NBAStat>, Vec<(Correction, GameInfo)>> {
    let mut season: Vec<NBAStat> = Vec::new();
    let mut corrections: Vec<(Correction, GameInfo)> = Vec::new();

    for row in rows {
        if let Some(row_data) = row.as_array() {
            let fields: HashMap<String, Value> = headers
                .iter()
                .zip(row_data.iter())
                .map(|(name, value)| (name.to_string(), value.clone()))
                .collect();

            match stat {
                Player => match fields_to_player_box_score(&fields, period) {
                    Ok(box_score) => {
                        season.push(NBAStat::Player(box_score));
                    }
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                Team => match fields_to_team_box_score(&fields, period) {
                    Ok(box_score) => {
                        season.push(NBAStat::Team(box_score));
                    }
                    Err(e) => {
                        corrections.push(e);
                    }
                },
                LineUp => panic!("lineup stats are not yet supported."),
            };
        }
    }

    if corrections.len() == 0 {
        Ok(season)
    } else {
        Err(corrections)
    }
}

fn fields_to_team_box_score(
    s: &HashMap<String, Value>,
    period: SeasonPeriod,
) -> Result<TeamBoxScore, (Correction, GameInfo)> {
    let gameid = parse_string(s.get("GAME_ID"));
    let teamid = parse_u64(s.get("TEAM_ID")).unwrap();
    let season = str_to_num(s.get("SEASON_ID")) as i32;
    let team_abbr = parse_string(s.get("TEAM_ABBREVIATION"));

    let mut correction = Correction::new(
        gameid.replace("\"", ""),
        season,
        None,
        teamid,
        team_abbr,
        Team,
        period,
    );

    // Handle optional fields
    let required_fields = [
        ("TEAM_NAME", TEAM_NAME),
        ("TEAM_ID", TEAM_ID),
        ("GAME_DATE", GAME_DATE),
        ("MATCHUP", MATCHUP),
        ("WL", WL),
    ];

    // Check required fields and add to correction if missing
    let mut missing_fields = Vec::new();
    for (field_name, field_type) in required_fields {
        match field_type {
            _ => {
                if s.get(field_name).is_none() || s.get(field_name) == Some(&Null) {
                    correction.add_missing_field(field_type, StatValue::new());
                    missing_fields.push(field_name);
                }
            }
        }
    }

    // Return error if any corrections needed
    if !correction.corrections.is_empty() {
        let matchup_string: MatchupString = parse_string(s.get("MATCHUP"))
            .parse::<MatchupString>()
            .map_err(|e| {
                eprintln!("Matchup parse error: {}", e);
                correction.add_missing_field(MATCHUP, StatValue::new());
            })
            .unwrap_or(MatchupString("invalid matchup".to_string()));

        return Err((
            correction,
            GameInfo::new(
                matchup_string,
                parse_date(s.get("GAME_DATE")).unwrap_or_default(),
                Some(parse_string(s.get("PLAYER_NAME"))),
                parse_string(s.get("TEAM_NAME")),
                parse_string(s.get("TEAM_ABBREVIATION")),
            ),
        ));
    }

    // Build TeamBoxScore if no corrections needed
    Ok(TeamBoxScoreBuilder::default()
        .game_id(gameid)
        .season_id(season)
        .team_id(teamid)
        .ast(parse_u32(s.get("AST")))
        .plus_minus(parse_i32(s.get("PLUS_MINUS")))
        .reb(parse_u32(s.get("REB")))
        .min(parse_u32(s.get("MIN")))
        .wl(parse_wl(s.get("WL")).unwrap())
        .team_name(s.get("TEAM_NAME").unwrap().to_string())
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
        .game_date(parse_date(s.get("GAME_DATE")).unwrap())
        .team_abbreviation(parse_string(s.get("TEAM_ABBREVIATION")))
        .matchup(MatchupString(parse_string(s.get("MATCHUP"))))
        .roster(Vec::new()) // Empty roster by default
        .build()
        .unwrap())
}

///
/// fields_to_player_box_score returns a result of either a player box score or a correction.
/// if the function returns a correction, the correction acts as a form that needs to be
/// completed before that entry can be finalized. as such, seemingly inconsequentially,
/// the player stats must always be ripped from file before team results.
///
fn fields_to_player_box_score(
    s: &HashMap<String, Value>,
    period: SeasonPeriod,
) -> Result<PlayerBoxScore, (Correction, GameInfo)> {
    //if it fails to parse the identifier then it will crash
    let gameid = parse_string(s.get("GAME_ID"));
    let playerid = parse_u64(s.get("PLAYER_ID")).unwrap();
    let season = str_to_num(s.get("SEASON_ID")) as i32;
    let teamid = parse_u64(s.get("TEAM_ID")).unwrap();
    let team_abbr = parse_string(s.get("TEAM_ABBREVIATION"));

    let mut correction = Correction::new(
        gameid.replace("\"", ""),
        season,
        None,
        teamid,
        team_abbr,
        Team,
        period,
    );

    let required_fields = [
        ("TEAM_NAME", TEAM_NAME),
        ("TEAM_ID", TEAM_ID),
        ("GAME_DATE", GAME_DATE),
        ("MATCHUP", MATCHUP),
        ("PLAYER_NAME", PLAYER_NAME),
        ("WL", WL),
    ];

    let mut missing_fields = Vec::new();
    for (field_name, field_type) in required_fields {
        if s.get(field_name).is_none() || s.get(field_name) == Some(&Null) {
            correction.add_missing_field(field_type, StatValue::new());
            missing_fields.push(field_name);
        }
    }

    if !correction.corrections.is_empty() {
        let matchup_string: MatchupString = parse_string(s.get("MATCHUP"))
            .parse::<MatchupString>()
            .map_err(|e| {
                eprintln!("Matchup parse error: {}", e);
                correction.add_missing_field(MATCHUP, StatValue::new());
            })
            .unwrap_or(MatchupString("invalid matchup".to_string()));

        return Err((
            correction,
            GameInfo::new(
                matchup_string,
                parse_date(s.get("GAME_DATE")).unwrap_or_default(),
                Some(parse_string(s.get("PLAYER_NAME"))),
                parse_string(s.get("TEAM_ABBREVIATION")),
                parse_string(s.get("TEAM_NAME")),
            ),
        ));
    }

    Ok(PlayerBoxScoreBuilder::default()
        .ast(parse_u32(s.get("AST")))
        .plus_minus(parse_i32(s.get("PLUS_MINUS")))
        .season_id(season)
        .game_id(gameid.clone())
        .reb(parse_u32(s.get("REB")))
        .min(parse_u32(s.get("MIN")))
        .wl(parse_wl(s.get("WL")).unwrap())
        .team_name(parse_string(s.get("TEAM_NAME")))
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
        .game_date(parse_date(s.get("GAME_DATE")).unwrap())
        .team_abbreviation(parse_string(s.get("TEAM_ABBREVIATION")))
        .matchup(MatchupString(parse_string(s.get("MATCHUP"))))
        .player_name(parse_string(s.get("PLAYER_NAME")))
        .player_id(playerid)
        .team_id(parse_u64(s.get("TEAM_ID")).unwrap())
        .elo(3000)
        .build()
        .unwrap())
}
