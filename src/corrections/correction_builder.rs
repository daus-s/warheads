use crate::corrections::correction::Correction;
use crate::corrections::correction_loader::load_single_correction;
use crate::stats::game_display::GameDisplay;
use crate::stats::identity::Identifiable;
use crate::stats::nba_kind::NBAStatKind;
use crate::stats::percent::PercentGeneric;
use crate::stats::stat_column::StatColumn;
use crate::stats::types::BoolInt;
use crate::tui::prompter::{
    prompt_and_delete, prompt_and_select, prompt_and_validate, prompt_with_options,
};
use crate::types::{
    GameDate, GameId, GameResult, Matchup, PlayerId, SeasonId, TeamAbbreviation, TeamId,
};
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CorrectionBuilder {
    correction: Correction,
    display: Option<GameDisplay>,
}

impl CorrectionBuilder {
    pub fn new(
        game_id: GameId,
        season: SeasonId,
        player_id: Option<PlayerId>,
        team_id: TeamId,
        team_abbr: TeamAbbreviation,
        kind: NBAStatKind,
        game_date: GameDate,
    ) -> Self {
        CorrectionBuilder {
            correction: Correction {
                game_id,
                game_date,
                season,
                player_id,
                team_id,
                team_abbr: team_abbr.clone(),
                period: season.period(),
                kind,
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
        self.correction.corrections.insert(col, val);
    }

    pub fn remove(&mut self, col: StatColumn) {
        self.correction.corrections.remove(&col);
    }

    pub fn create_and_save(&mut self) -> Correction {
        use std::io::{stdout, Write};

        let (mut corrections, display_option) = (self.correction.clone(), self.display.clone());

        let (display_string, confirmation) = match display_option {
            Some(display) => (format!("{display}"), display.display_name()),
            None => (
                format!("🚫 no game display data 🚫"),
                String::from("GENERIC DELETE"),
            ),
        };

        if let Ok(preexisting) = load_single_correction(&self.correction.identity()) {
            corrections.merge(preexisting);
        }

        println!("{}", display_string);

        //if the correctionbuilder is provided as deleting it is from a source that needs to delete the data so we should not check again.
        if corrections.delete {
            println!("🗑️ deleting {}", corrections.identity());

            save_correction(&corrections);

            return corrections.clone();
        } else {
            let delete = prompt_and_delete(&confirmation);

            corrections.set_delete(delete);

            if delete {
                println!("🗑️ deleting {}", corrections.identity());

                save_correction(&corrections);

                return corrections.clone(); //if we are deleting we don't need any values for the corrections
            }
        }

        let mut fields_to_correct: Vec<StatColumn> =
            corrections.corrections.keys().cloned().collect();

        fields_to_correct.sort();

        let mut stdout = stdout();

        for col in fields_to_correct {
            if let Some(val) = corrections.corrections.get(&col) {
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
                        let tm = corrections.team_abbr();

                        if let Some(display) = self.display.clone() {
                            prompt_with_options::<(Matchup, TeamAbbreviation)>(
                                format!("enter {}", col).as_str(),
                                (display.matchup(), tm),
                            )
                        } else {
                            eprintln!("❌ cannot correct matchup. assigning new matchup to Null");

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
                corrections.corrections.insert(col, value.clone()); //only cloned for display below

                // Display the confirmed value
                print!("\x1B[2A\x1B[0J"); // Move up 2 lines and clear from cursor to end
                println!("{}: {}", col, value); // New value
            }
        }
        save_correction(&corrections);

        corrections
    }

    /// Returns whether the correction builder has any corrections to apply. It does not specify whether the record should be deleted.
    pub fn has_corrections(&self) -> bool {
        self.correction.len() > 0 || self.correction.delete
    }

    pub fn set_delete(&mut self, delete: bool) {
        self.correction.delete = delete;
    }

    pub fn correction(&self) -> &Correction {
        &self.correction
    }
}

fn save_correction(correction: &Correction) {
    match correction.save() {
        Ok(_) => {
            println!(
                "✅ successfully saved corrections for {}",
                correction.identity()
            );
        }
        Err(e) => {
            eprintln!(
                "❌ failed to save corrections for {}: {e}",
                correction.identity()
            );
        }
    };
}
