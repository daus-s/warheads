use std::collections::{ HashSet};
use std::error::Error;
use std::fmt::Debug;
use std::fs;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::{BufWriter, Write};
use std::path::Path;
use serde::{Deserialize, Serialize};
use stats::kind::NBAStatKind;
use serde_json::Value;
use dapi::path_manager::filepath;
use dapi::rip::{process_nba_games, raw_extract};
use stats::format::{season_file, season_fmt};
use stats::stat_column::{column_index, StatEntry};

#[derive(Serialize, Deserialize)]
pub struct Correction {
    pub gameid: u64,
    pub playerid: u64,
    pub season: i32,
    pub kind: NBAStatKind,
    pub corrections: HashSet<StatEntry>,
}

impl Correction {
    ///
    /// apply the correction to the pre-processed data from raw data source in
    /// `data/nba/{season}/{NBAStatType}/{seasonf}.json`
    pub fn apply(&self) -> Result<(), String> {

        let file_string = filepath(self.season, self.kind);

        let file_path = Path::new(&file_string);

        let content = fs::read_to_string(file_path); //read the file


        match content {
            Ok(txt) => {

                let parsed: Value = serde_json::from_str(&txt).map_err(|_| format!("could not parse JSON for file {:?}.", &file_string))?;

                let mut games =  raw_extract(parsed).map_err(|_| format!("could not process nba games for the {} season", season_fmt(self.season)))?;

                for game in games.iter_mut() {

                    //find the players results
                    if self.identify(game.clone()) {
                        println!("updating game");
                        //edit the player vector
                        *game = self.correct(game.clone());
                    }

                }

                let box_scores = games.join(",");

                let updates = format!("[{}]", box_scores);

                let copy = partition(txt, updates);

                //write over the old file
                let file = OpenOptions::new().write(true).truncate(true).open(file_path).unwrap();

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
    fn identify(&self, s: String) -> bool {

        let columns = columns(s);

        match columns.as_slice() {
            [season_id, player_id, _, _, _, _, game_id, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, ] => {

                self.season + 20000 == season_id.replace("\"", "").parse::<i32>().unwrap() &&
                    self.playerid == player_id.parse::<u64>().unwrap() &&
                    self.gameid == game_id.replace("\"", "").parse::<u64>().unwrap()
            },
            _ => false, // no way can a bad object satisfy the condition
        }
    }

    fn correct(&self, game: String) -> String {
        let columns = columns(game.clone());

        match columns.as_slice() {
            [season_id, player_id, player_name, team_id, team_abbreviation, team_name, game_id, game_date, matchup,
            wl, min, fgm, fga, fg_pct, fg3m, fg3a, fg3_pct, ftm, fta, ft_pct, oreb, dreb,
            reb, ast, stl, blk, tov, pf, pts, plus_minus, fantasy_pts,  video_available,] => {

                let mut cs: Vec<String> = vec![ season_id.to_string(),
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

                for se in self.corrections.iter() {

                    if let Some(i) = column_index(&se.col()) {
                        cs[i] = se.val();
                    }
                }


                format!("[{:#}]", cs.join(","))
            },
            _ => {
                eprintln!("columns string was not formatted correctly");
                "".to_string()
            },
        }
    }

    pub fn save(&self) {
        let filename = format!("{}_{}.corr", self.gameid, self.playerid);

    }

    pub fn create(&self)  {

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

    format!("{}{}{}", before , new_data, "}]}")
}