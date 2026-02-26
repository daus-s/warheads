use crate::edit::edit::Edit;

use crate::format::percent::PercentGeneric;

use crate::stats::game_display::GameDisplay;
use crate::stats::identity::Identifiable;
use crate::stats::stat_column::StatColumn;
use crate::stats::types::BoolInt;

use crate::tui::prompter::prompt_and_delete;
use crate::tui::prompter::prompt_and_select;
use crate::tui::prompter::prompt_and_validate;
use crate::tui::prompter::prompt_with_options;

use crate::types::GameDate;
use crate::types::GameId;
use crate::types::GameResult;
use crate::types::Matchup;
use crate::types::PlayerId;
use crate::types::SeasonId;
use crate::types::TeamAbbreviation;
use crate::types::TeamId;

use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct EditBuilder {
    edit: Edit,
    display: Option<GameDisplay>,
}

impl EditBuilder {
    pub fn new(
        game_id: GameId,
        season: SeasonId,
        player_id: Option<PlayerId>,
        team_id: TeamId,
        team_abbr: TeamAbbreviation,
        game_date: GameDate,
    ) -> Self {
        EditBuilder {
            edit: Edit {
                game_id,
                game_date,
                season,
                player_id,
                team_id,
                team_abbr: team_abbr.clone(),
                delete: false,
                corrections: HashMap::new(),
            },
            display: None,
        }
    }

    pub fn update_display(&mut self, meta: GameDisplay) {
        self.display = Some(meta);
    }

    pub fn add_missing_field(&mut self, col: StatColumn, val: Value) {
        self.edit.corrections.insert(col, val);
    }

    pub fn remove(&mut self, col: StatColumn) {
        self.edit.corrections.remove(&col);
    }

    pub fn build(&self) -> Option<Edit> {
        if self.edit.corrections.iter().any(|(_, val)| val.is_null()) {
            None
        } else {
            Some(self.edit.clone())
        }
    }

    pub fn prompt(&mut self) {
        use std::io::{stdout, Write};

        let (display_string, confirmation) = match self.display {
            Some(ref display) => (format!("{display}"), display.display_name()),
            None => (
                format!("🚫 no game display data"),
                String::from("GENERIC DELETE"),
            ),
        };

        println!("{}", display_string);

        //if the correctionbuilder is provided as deleting it is from a source that needs to delete the data so we should not check again.
        if self.edit.delete {
            println!("🗑️ deleting {}", self.edit.identity());
        } else {
            let delete = prompt_and_delete(&confirmation);

            self.edit.set_delete(delete);

            if delete {
                println!("🗑️ deleting {}", self.edit.identity());
            }
        }

        let mut fields_to_correct: Vec<StatColumn> =
            self.edit.corrections.keys().cloned().collect();

        fields_to_correct.sort();

        let mut stdout = stdout();

        for col in fields_to_correct {
            if let Some(val) = self.edit.corrections.get(&col) {
                // Display the column name and current value (grayed out if not confirmed) //id like this to update after the new value is completed is that possible
                println!("\x1b[90m{}: {}\x1b[0m", col, val);
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
                        let tm = self.edit.team_abbr();

                        if let Some(display) = self.display.clone() {
                            prompt_with_options::<(Matchup, TeamAbbreviation)>(
                                format!("enter {}", col).as_str(),
                                (display.matchup(), tm),
                            )
                        } else {
                            println!("❌ cannot correct matchup. assigning new matchup to Null");

                            Value::Null
                        }
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
                        prompt_and_validate::<PercentGeneric>(
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
                //

                let s = format!("{}: {}", col, value);
                self.edit.corrections.insert(col, value);

                // Display the confirmed value
                print!("\x1B[2A\x1B[0J"); // Move up 2 lines and clear from cursor to end
                println!("{s}");
            }
        }
    }

    /// Returns whether the correction builder has any corrections to apply. It does not specify whether the record should be deleted.
    pub fn has_corrections(&self) -> bool {
        self.edit.len() > 0 || self.edit.delete
    }

    pub fn set_delete(&mut self, delete: bool) {
        self.edit.delete = delete;
    }

    pub fn correction(&self) -> &Edit {
        &self.edit
    }

    pub fn date(&self) -> &GameDate {
        &self.edit.game_date
    }

    pub fn game_id(&self) -> GameId {
        self.edit.game_id
    }

    pub fn team_abbr(&self) -> &TeamAbbreviation {
        &self.edit.team_abbr
    }
}
