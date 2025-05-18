use crate::format::path_manager::nba_data_path;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::season_type::SeasonPeriod;
use crate::stats::stat_column::StatColumn::*;
use crate::stats::stat_value::StatValue;
use crate::stats::statify::StatPair;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Domain = (i32, NBAStatKind, SeasonPeriod);
pub fn json_to_hashmap(value: &Value) -> Result<HashMap<Identity, String>, String> {
    let result_set = get_result_set(&value)?;

    let rows = get_rows(&result_set)?;

    Ok((&rows)
        .iter()
        .map(|v| (v.identity(), box_score_value_to_string(v)))
        .collect())
}

pub fn get_result_set(v: &Value) -> Result<Value, String> {
    let result_sets = v
        .get("resultSets")
        .and_then(|rs| rs.as_array())
        .ok_or_else(|| "resultSets is not an array or is missing")?;

    let result_set = result_sets
        .get(0)
        .ok_or_else(|| "resultSets array is empty")?;

    Ok(result_set.clone())
}

pub fn headers(s: &Value) -> Result<Vec<String>, String> {
    Ok(s.get("headers")
        .and_then(|h| h.as_array())
        .ok_or_else(|| "Missing or invalid 'headers' field".to_string())?
        .iter()
        .filter_map(|h| Option::from(h.to_string()))
        .collect())
}

pub fn get_rows(set: &Value) -> Result<Vec<Value>, String> {
    Ok(set
        .get("rowSet")
        .and_then(|r| r.as_array())
        .ok_or_else(|| "Missing or invalid 'rowSet' field")?
        .clone())
}

fn get_rows_from_file(filepath: PathBuf) -> Result<Vec<Value>, String> {
    let content =
        fs::read_to_string(&filepath).map_err(|_| format!("failed to read file {:?}", filepath))?;

    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("failed to parse JSON from file: {}", e))?;

    let set = get_result_set(&json).map_err(|e| format!("failed to get result set: {}", e))?;

    let rows = get_rows(&set).map_err(|e| format!("failed to get rows: {}", e))?;

    Ok(rows)
}

fn box_score_value_to_string(value: &Value) -> String {
    let slices = value.as_array().unwrap().as_slice();

    match slices {
        [season_id, player_id, player_name, team_id, team_abbr, team_name, game_id, game_date, matchup, wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, stl, blk, tov, pf, pts, plus_minus, fantasy_pts, video_available] =>
        {
            format!("[\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {}\n        ]",
                 StatPair(SEASON_ID, StatValue::from_value(season_id.clone())),
                 StatPair(PLAYER_ID, StatValue::from_value(player_id.clone())),
                 StatPair(PLAYER_NAME, StatValue::from_value(player_name.clone())),
                 StatPair(TEAM_ID, StatValue::from_value(team_id.clone())),
                 StatPair(TEAM_ABBREVIATION, StatValue::from_value(team_abbr.clone())),
                 StatPair(TEAM_NAME, StatValue::from_value(team_name.clone())),
                 StatPair(GAME_ID, StatValue::from_value(game_id.clone())),
                 StatPair(GAME_DATE, StatValue::from_value(game_date.clone())),
                 StatPair(MATCHUP, StatValue::from_value(matchup.clone())),
                 StatPair(WL, StatValue::from_value(wl.clone())),
                 StatPair(MIN, StatValue::from_value(min.clone())),
                 StatPair(FGM, StatValue::from_value(fgm.clone())),
                 StatPair(FGA, StatValue::from_value(fga.clone())),
                 StatPair(FG_PCT, StatValue::from_value(fg_pct.clone())),
                 StatPair(FG3M, StatValue::from_value(fg3m.clone())),
                 StatPair(FG3A, StatValue::from_value(fg3a.clone())),
                 StatPair(FG3_PCT, StatValue::from_value(fg3_pct.clone())),
                 StatPair(FTM, StatValue::from_value(ftm.clone())),
                 StatPair(FTA, StatValue::from_value(fta.clone())),
                 StatPair(FT_PCT, StatValue::from_value(ft_pct.clone())),
                 StatPair(OREB, StatValue::from_value(oreb.clone())),
                 StatPair(DREB, StatValue::from_value(dreb.clone())),
                 StatPair(REB, StatValue::from_value(reb.clone())),
                 StatPair(AST, StatValue::from_value(ast.clone())),
                 StatPair(STL, StatValue::from_value(stl.clone())),
                 StatPair(BLK, StatValue::from_value(blk.clone())),
                 StatPair(TOV, StatValue::from_value(tov.clone())),
                 StatPair(PF, StatValue::from_value(pf.clone())),
                 StatPair(PTS, StatValue::from_value(pts.clone())),
                 StatPair(PLUS_MINUS, StatValue::from_value(plus_minus.clone())),
                 StatPair(FANTASY_PTS, StatValue::from_value(fantasy_pts.clone())),
                 StatPair(VIDEO_AVAILABLE, StatValue::from_value(video_available.clone())),
             )
        }
        [season_id, team_id, team_abbr, team_name, game_id, game_date, matchup, wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, stl, blk, tov, pf, pts, plus_minus, video_available] =>
        {
            format!("        [\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n          {},\n]",
                      StatPair(SEASON_ID, StatValue::from_value(season_id.clone())),
                      StatPair(TEAM_ID, StatValue::from_value(team_id.clone())),
                      StatPair(TEAM_ABBREVIATION, StatValue::from_value(team_abbr.clone())),
                      StatPair(TEAM_NAME, StatValue::from_value(team_name.clone())),
                      StatPair(GAME_ID, StatValue::from_value(game_id.clone())),
                      StatPair(GAME_DATE, StatValue::from_value(game_date.clone())),
                      StatPair(MATCHUP, StatValue::from_value(matchup.clone())),
                      StatPair(WL, StatValue::from_value(wl.clone())),
                      StatPair(MIN, StatValue::from_value(min.clone())),
                      StatPair(FGM, StatValue::from_value(fgm.clone())),
                      StatPair(FGA, StatValue::from_value(fga.clone())),
                      StatPair(FG_PCT, StatValue::from_value(fg_pct.clone())),
                      StatPair(FG3M, StatValue::from_value(fg3m.clone())),
                      StatPair(FG3A, StatValue::from_value(fg3a.clone())),
                      StatPair(FG3_PCT, StatValue::from_value(fg3_pct.clone())),
                      StatPair(FTM, StatValue::from_value(ftm.clone())),
                      StatPair(FTA, StatValue::from_value(fta.clone())),
                      StatPair(FT_PCT, StatValue::from_value(ft_pct.clone())),
                      StatPair(OREB, StatValue::from_value(oreb.clone())),
                      StatPair(DREB, StatValue::from_value(dreb.clone())),
                      StatPair(REB, StatValue::from_value(reb.clone())),
                      StatPair(AST, StatValue::from_value(ast.clone())),
                      StatPair(STL, StatValue::from_value(stl.clone())),
                      StatPair(BLK, StatValue::from_value(blk.clone())),
                      StatPair(TOV, StatValue::from_value(tov.clone())),
                      StatPair(PF, StatValue::from_value(pf.clone())),
                      StatPair(PTS, StatValue::from_value(pts.clone())),
                      StatPair(PLUS_MINUS, StatValue::from_value(plus_minus.clone())),
                      StatPair(VIDEO_AVAILABLE, StatValue::from_value(video_available.clone())),
             )
        }
        _ => panic!("⚠️couldn't parse box score! unrecognized JSON format."),
    }
}
