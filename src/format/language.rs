use crate::stats::stat_column::StatColumn::*;
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
            .filter(|s| s != "")
            .collect()
    }
}

/// partition overwrites previous data with newly corrected data based on the pattern of the NBA
/// stats api JSON response.
///
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
        [
            season_id,
            player_id,
            player_name,
            team_id,
            team_abbr,
            team_name,
            game_id,
            game_date,
            matchup,
            wl,
            min,
            fgm,
            fga,
            fg_pct,
            fg3m,
            fg3a,
            fg3_pct,
            ftm,
            fta,
            ft_pct,
            oreb,
            dreb,
            reb,
            ast,
            stl,
            blk,
            tov,
            pf,
            pts,
            plus_minus,
            fantasy_pts,
            video_available,
        ] => {
            format!(
                "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
                StatPair(SEASON_ID, season_id.clone()),
                StatPair(PLAYER_ID, player_id.clone()),
                StatPair(PLAYER_NAME, player_name.clone()),
                StatPair(TEAM_ID, team_id.clone()),
                StatPair(TEAM_ABBREVIATION, team_abbr.clone()),
                StatPair(TEAM_NAME, team_name.clone()),
                StatPair(GAME_ID, game_id.clone()),
                StatPair(GAME_DATE, game_date.clone()),
                StatPair(MATCHUP, matchup.clone()),
                StatPair(WL, wl.clone()),
                StatPair(MIN, min.clone()),
                StatPair(FGM, fgm.clone()),
                StatPair(FGA, fga.clone()),
                StatPair(FG_PCT, fg_pct.clone()),
                StatPair(FG3M, fg3m.clone()),
                StatPair(FG3A, fg3a.clone()),
                StatPair(FG3_PCT, fg3_pct.clone()),
                StatPair(FTM, ftm.clone()),
                StatPair(FTA, fta.clone()),
                StatPair(FT_PCT, ft_pct.clone()),
                StatPair(OREB, oreb.clone()),
                StatPair(DREB, dreb.clone()),
                StatPair(REB, reb.clone()),
                StatPair(AST, ast.clone()),
                StatPair(STL, stl.clone()),
                StatPair(BLK, blk.clone()),
                StatPair(TOV, tov.clone()),
                StatPair(PF, pf.clone()),
                StatPair(PTS, pts.clone()),
                StatPair(PLUS_MINUS, plus_minus.clone()),
                StatPair(FANTASY_PTS, fantasy_pts.clone()),
                StatPair(VIDEO_AVAILABLE, video_available.clone()),
            )
        }
        [
            season_id,
            team_id,
            team_abbr,
            team_name,
            game_id,
            game_date,
            matchup,
            wl,
            min,
            fgm,
            fga,
            fg_pct,
            fg3m,
            fg3a,
            fg3_pct,
            ftm,
            fta,
            ft_pct,
            oreb,
            dreb,
            reb,
            ast,
            stl,
            blk,
            tov,
            pf,
            pts,
            plus_minus,
            video_available,
        ] => {
            format!(
                "[{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
                StatPair(SEASON_ID, season_id.clone()),
                StatPair(TEAM_ID, team_id.clone()),
                StatPair(TEAM_ABBREVIATION, team_abbr.clone()),
                StatPair(TEAM_NAME, team_name.clone()),
                StatPair(GAME_ID, game_id.clone()),
                StatPair(GAME_DATE, game_date.clone()),
                StatPair(MATCHUP, matchup.clone()),
                StatPair(WL, wl.clone()),
                StatPair(MIN, min.clone()),
                StatPair(FGM, fgm.clone()),
                StatPair(FGA, fga.clone()),
                StatPair(FG_PCT, fg_pct.clone()),
                StatPair(FG3M, fg3m.clone()),
                StatPair(FG3A, fg3a.clone()),
                StatPair(FG3_PCT, fg3_pct.clone()),
                StatPair(FTM, ftm.clone()),
                StatPair(FTA, fta.clone()),
                StatPair(FT_PCT, ft_pct.clone()),
                StatPair(OREB, oreb.clone()),
                StatPair(DREB, dreb.clone()),
                StatPair(REB, reb.clone()),
                StatPair(AST, ast.clone()),
                StatPair(STL, stl.clone()),
                StatPair(BLK, blk.clone()),
                StatPair(TOV, tov.clone()),
                StatPair(PF, pf.clone()),
                StatPair(PTS, pts.clone()),
                StatPair(PLUS_MINUS, plus_minus.clone()),
                StatPair(VIDEO_AVAILABLE, video_available.clone()),
            )
        }
        _ => {
            eprintln!("⚠️couldn't parse box score! unrecognized JSON format.");

            String::new()
        }
    }
}
