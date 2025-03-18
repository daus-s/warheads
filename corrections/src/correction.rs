use format::path_manager::{correction_file, correction_path, data_path};
use format::season::season_fmt;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use stats::extract::nba_json_to_str;
use stats::kind::NBAStatKind;
use stats::stat_column::{column_index, StatColumn};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{fs, io};
use chrono::NaiveDate;
use serde_json::Value::Null;
use stats::percent::Percent;
use stats::stat_value::StatValue;
use tui::prompter::{prompt_and_select, prompt_and_validate};
use serde_with::chrono;
use stats::types::{BoolInt, GameResult, MatchupString};

#[derive(Serialize, Deserialize)]
pub struct Correction {
    pub gameid: String,
    pub id: u64,
    pub season: i32,
    pub kind: NBAStatKind,
    pub corrections: HashMap<StatColumn, StatValue>,
}

impl Correction {
    pub fn new(gameid: String, id: u64, season: i32, kind: NBAStatKind) -> Correction {
        Correction {
            gameid,
            id,
            season,
            kind,
            corrections: Default::default(),
        }
    }
    ///
    /// apply the correction to the pre-processed data from raw data source in
    ///
    pub fn apply(&self) -> Result<(), String> {
        let file_string = data_path(self.season, self.kind);

        let file_path = Path::new(&file_string);

        let content = fs::read_to_string(file_path); //read the file

        match content {
            Ok(txt) => {
                let parsed: Value = serde_json::from_str(&txt)
                    .map_err(|_| format!("could not parse JSON for file {:?}.", &file_string))?;

                let mut games = nba_json_to_str(parsed).map_err(|_| {
                    format!(
                        "could not process nba games for the {} season",
                        season_fmt(self.season)
                    )
                })?;

                for game in games.iter_mut() {
                    //find the players results
                    if self.is_game(game.clone()) {
                        println!("updating game");
                        //edit the player vector
                        *game = self.correct(game.clone());
                        break;
                    }
                }

                let box_scores = games.join(",");

                let updates = format!("[{}]", box_scores);

                let copy = partition(txt, updates);

                //write over the old file
                let file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(file_path)
                    .unwrap();

                let mut writer = BufWriter::new(file);

                writeln!(writer, "{}", copy).unwrap();

                writer.flush().unwrap();

                Ok(())
            }
            Err(_) => Err(format!("could not open file {}", file_string)),
        }
    }

    ///
    /// this function should only ever accept well formatted strings so it will panic if not passed well.
    /// thus it does not return a result but only a boolean
    ///
    /// more importantly this function is asked "is this the game that i correct?"
    /// so we answer true or false
    ///
    fn is_game(&self, s: String) -> bool {
        println!("row: {}", s);

        let columns = columns(s);

        match columns.as_slice() {
            [season_id, player_id, _, _, _, _, game_id, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => {

                println!("{}", game_id);

                self.season + 20000 == season_id.replace("\"", "").parse::<i32>().unwrap()
                    && self.id == player_id.parse::<u64>().unwrap()
                    && self.gameid.parse::<u64>().unwrap() == game_id.replace("\"", "").parse::<u64>().unwrap()
            }
            _ => false, // no way can a bad object satisfy the condition, *panic!* ?
        }
    }

    ///
    /// im worried about this function i might have first tried no tested for too long
    /// mayday mayday mayday
    ///
    fn correct(&self, game: String) -> String {
        let columns = columns(game.clone());

        match columns.as_slice() {
            [season_id, player_id, player_name, team_id, team_abbreviation, team_name, game_id, game_date, matchup, wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb, reb, ast, stl, blk, tov, pf, pts, plus_minus, fantasy_pts, video_available] =>
            {
                let mut cs: Vec<String> = vec![
                    season_id.to_string(),
                    player_id.to_string(),
                    player_name.to_string(),
                    team_id.to_string(),
                    team_abbreviation.to_string(),
                    team_name.to_string(),
                    game_id.to_string(),
                    game_date.to_string(),
                    matchup.to_string(),
                    wl.to_string(),
                    min.to_string(),
                    fgm.to_string(),
                    fga.to_string(),
                    fg_pct.to_string(),
                    fg3m.to_string(),
                    fg3a.to_string(),
                    fg3_pct.to_string(),
                    ftm.to_string(),
                    fta.to_string(),
                    ft_pct.to_string(),
                    oreb.to_string(),
                    dreb.to_string(),
                    reb.to_string(),
                    ast.to_string(),
                    stl.to_string(),
                    blk.to_string(),
                    tov.to_string(),
                    pf.to_string(),
                    pts.to_string(),
                    plus_minus.to_string(),
                    fantasy_pts.to_string(),
                    video_available.to_string(),
                ];

                for (&col, val) in self.corrections.iter() {
                    if let Some(i) = column_index(&col) {

                        let f_str = val.val().unwrap_or_else(|| Null).to_string();

                        cs[i] = f_str;
                    }
                }

                format!("[{:#}]", cs.join(","))
            }
            _ => {
                eprintln!("columns string was not formatted correctly");
                "".to_string()
            }
        }
    }

    /// Saves the correction to the file:
    /// `corrections/{season_file}/{NBAStatType}/{}.json`
    ///
    /// This is a private function and is called when `fn create ->` is completed.
    ///
    fn save(&self) -> io::Result<()> {
        let path = correction_path(self.season, self.kind);

        let file = correction_file(self.gameid.as_str(), self.id);

        fs::create_dir_all(&path)?;

        let json = serde_json::to_string_pretty(self)?;

        // Write the JSON string to the file
        fs::write(format!("{}{}", path, file), json)?;

        Ok(())
    }

    pub fn create(&mut self) {

        let mut sorted_keys: Vec<StatColumn> = self.corrections.keys().cloned().collect();
        sorted_keys.sort();

        for col in sorted_keys {
            if let Some(val) = self.corrections.get_mut(&col) {


                // Display the column name and current value (grayed out if not confirmed)
                println!(
                    "\x1b[90m{}: {}\x1b[0m", // ANSI escape code for gray text
                    col, val.val().unwrap_or_else(|| Null).to_string()
                );


                /*

                this will be separated by both section and type

                */
                let value: Value = match col {

                    //team identification
                    StatColumn::TEAM_ABBREVIATION |
                    StatColumn::TEAM_NAME => prompt_and_validate::<String>(format!("enter {}", col).as_str()),
                    StatColumn::TEAM_ID => prompt_and_validate::<u64>(format!("enter {}", col).as_str()),

                    //game data
                    StatColumn::SEASON_ID => prompt_and_validate::<i32>(format!("enter {}", col).as_str()),
                    StatColumn::GAME_DATE => prompt_and_validate::<NaiveDate>("Enter game date (YYYY-MM-DD)"),
                    StatColumn::GAME_ID => prompt_and_validate::<String>(format!("enter {}", col).as_str()),
                    StatColumn::MATCHUP => prompt_and_validate::<MatchupString>(format!("enter {}", col).as_str()),

                    //player data
                    StatColumn::PLAYER_ID => prompt_and_validate::<u64>(format!("enter {}", col).as_str()),
                    StatColumn::PLAYER_NAME => prompt_and_validate::<String>(format!("enter {}", col).as_str()),

                    //classic box score
                    StatColumn::WL => prompt_and_select::<GameResult>("select win/loss/draw"),

                    StatColumn::MIN
                    | StatColumn::FGM
                    | StatColumn::FGA
                    | StatColumn::FG3M
                    | StatColumn::FG3A
                    | StatColumn::FTM
                    | StatColumn::FTA
                    | StatColumn::OREB
                    | StatColumn::DREB
                    | StatColumn::REB
                    | StatColumn::AST
                    | StatColumn::STL
                    | StatColumn::BLK
                    | StatColumn::TOV
                    | StatColumn::PF
                    | StatColumn::PTS => prompt_and_validate::<u32>(format!("enter {}", col).as_str()),

                    //advanced statistics

                    StatColumn::FG_PCT | StatColumn::FG3_PCT | StatColumn::FT_PCT => prompt_and_validate::<Percent>(format!("Enter {} (as a percentage, e.g., 45.6 for 45.6%)", col).as_str()),
                    StatColumn::PLUS_MINUS => prompt_and_validate::<i32>(format!("enter {}", col).as_str()),
                    StatColumn::FANTASY_PTS => prompt_and_validate::<f32>(format!("Enter {}", col).as_str()),

                    // video available

                    StatColumn::VIDEO_AVAILABLE => prompt_and_select::<BoolInt>(format!("enter {}", col).as_str()),
                };

                // Lock in the value
                val.set(value.clone());

                // Display the confirmed value
                println!("{}: {}", &col, &value);
            }
        }
        // Save corrections to a .corr file
        self.save().expect("Failed to save corrections");

    }

    pub fn add_missing_field(&mut self, col: StatColumn, val: StatValue) {
        self.corrections.insert(col, val);
    }

    pub fn len(&self) -> usize {
        self.corrections.len()
    }
}

fn columns(s: String) -> Vec<String> {
    let columns = s.replace("[", "");

    let columns = columns.replace("]", "");

    columns.split(",").map(|x| x.to_string()).collect()
}

pub fn partition(txt: String, new_data: String) -> String {
    let beginning = "\"rowSet\":";

    let end_of_start = txt.find(beginning).unwrap() + beginning.len();

    let (before, _) = txt.split_at(end_of_start);

    format!("{}{}{}", before, new_data, "}]}")
}
