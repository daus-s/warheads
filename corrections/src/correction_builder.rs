use crate::correction::Correction;
use chrono::NaiveDate;
use serde_json::Value;
use serde_json::Value::Null;
use stats::game_info::GameInfo;
use stats::percent::Percent;
use stats::stat_column::StatColumn;
use stats::types::{BoolInt, GameResult, MatchupString};
use tui::prompter::{prompt_and_delete, prompt_and_select, prompt_and_validate};

pub struct CorrectionBuilder(Correction, GameInfo);

impl CorrectionBuilder {
    pub fn new(correction: Correction, game_info: GameInfo) -> Self {
        CorrectionBuilder(correction, game_info)
    }

    pub fn create(&mut self) -> Correction {
        use std::io::{stdout, Write};

        let (corrections, game_info) = (&mut self.0, &self.1);
        let mut sorted_keys: Vec<StatColumn> = corrections.corrections.keys().cloned().collect();
        sorted_keys.sort();

        let mut stdout = stdout();

        println!("{}", game_info);

        let confirmation = &game_info.confirmation_string();

        let delete = prompt_and_delete(confirmation);

        corrections.set_delete(delete);

        if delete {
            return corrections.clone(); //if we are deleting we don't need any values for the corrections
        }

        for col in sorted_keys {
            if let Some(val) = corrections.corrections.get_mut(&col) {
                // Display the column name and current value (grayed out if not confirmed) //id like this to update after the new value is completed is that possible
                println!("\x1b[90m{}: {}\x1b[0m", col, val.val().unwrap_or(Null));
                stdout.flush().unwrap();

                /*

                this will be separated by both section and type

                */
                let value: Value = match col {
                    //team identification
                    StatColumn::TEAM_ABBREVIATION | StatColumn::TEAM_NAME => {
                        prompt_and_validate::<String>(format!("enter {}", col).as_str())
                    }
                    StatColumn::TEAM_ID => {
                        prompt_and_validate::<u64>(format!("enter {}", col).as_str())
                    }

                    //game data
                    StatColumn::SEASON_ID => {
                        prompt_and_validate::<i32>(format!("enter {}", col).as_str())
                    }
                    StatColumn::GAME_DATE => {
                        prompt_and_validate::<NaiveDate>("Enter game date (YYYY-MM-DD)")
                    }
                    StatColumn::GAME_ID => {
                        prompt_and_validate::<String>(format!("enter {}", col).as_str())
                    }
                    StatColumn::MATCHUP => {
                        prompt_and_validate::<MatchupString>(format!("enter {}", col).as_str())
                    }

                    //player data
                    StatColumn::PLAYER_ID => {
                        prompt_and_validate::<u64>(format!("enter {}", col).as_str())
                    }
                    StatColumn::PLAYER_NAME => {
                        prompt_and_validate::<String>(format!("enter {}", col).as_str())
                    }

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
                    | StatColumn::PTS => {
                        prompt_and_validate::<u32>(format!("enter {}", col).as_str())
                    }

                    //advanced statistics
                    StatColumn::FG_PCT | StatColumn::FG3_PCT | StatColumn::FT_PCT => {
                        prompt_and_validate::<Percent>(
                            format!("Enter {} (as a percentage, e.g., 45.6 for 45.6%)", col)
                                .as_str(),
                        )
                    }
                    StatColumn::PLUS_MINUS => {
                        prompt_and_validate::<i32>(format!("enter {}", col).as_str())
                    }
                    StatColumn::FANTASY_PTS => {
                        prompt_and_validate::<f32>(format!("Enter {}", col).as_str())
                    }

                    // video available
                    StatColumn::VIDEO_AVAILABLE => {
                        prompt_and_select::<BoolInt>(format!("enter {}", col).as_str())
                    }
                };

                // Lock in the value
                val.set(value.clone());

                // Display the confirmed value
                print!("\x1B[2A\x1B[0J");  // Move up 2 lines and clear from cursor to end
                println!("{}: {}", col, value); // New value
            }
        }
        corrections.clone()
    }
}
