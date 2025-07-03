use crate::stats::stat_column::StatColumn::*;
use crate::stats::stat_value::StatValue;
use crate::stats::statify::StatPair;
use serde_json::Value;

// this file feels like Times New Roman. I am not sure why.
//

pub trait Columnizable {
    fn columns(&self) -> Vec<String>;
}

impl Columnizable for String {
    fn columns(&self) -> Vec<String> {
        self.replace(['[', ']'], "")
            .split(",")
            .map(|x| x.to_string().trim().into())
            .collect()
    }
}

/// partition takes the original string as a String and overwrites the entire bulk of the data with
/// the newly corrected data supplied as a list of GameStrings and returns the updated data.
///
/// partition deals with raw nba sourced data and as such must not have any formatting. the file
/// delivered via the NBA's stat s API has no whitespace (for JSON formatting) and so corrections
/// made to raw source files must also have no formatting preferences.
///
pub fn partition(txt: String, list: Vec<String>) -> String {
    let new_data = format!("[{}]", list.join(","));

    let beginning = "\"rowSet\":";

    let end_of_start = txt.find(beginning).unwrap() + beginning.len(); /* Ex.

                                                                        "rowSet:["
                                                                                ^
                                                                                Starting from and including [
                                                                       */

    let (prefix, _) = txt.split_at(end_of_start);

    let suffix = "}]}";

    format!("{}{}{}", prefix, new_data, suffix)
}

pub fn box_score_value_to_string(value: &Value) -> String {
    let slices = value.as_array().unwrap().as_slice();

    match slices {
        [season_id, player_id, player_name, team_id, team_abbr, team_name, game_id, game_date, matchup, wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, stl, blk, tov, pf, pts, plus_minus, fantasy_pts, video_available] =>
        {
            format!("[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
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
            format!("[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\n]",
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
        _ => {
            eprintln!("⚠️couldn't parse box score! unrecognized JSON format.");

            String::new()
        }
    }
}
