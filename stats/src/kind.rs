use crate::player_box_score::PlayerBoxScore;
use crate::team_box_score::TeamBoxScore;
use format::stat_path_formatter::StatPathFormatter;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum NBAStatKind {
    Team,
    Player,
    LineUp, //todo: develop this later-this is not a priority yet but may be very useful for elo and win-sharing.
}

pub enum NBAStat {
    Player(PlayerBoxScore),
    Team(TeamBoxScore),
    Players(Vec<PlayerBoxScore>),
    Teams(Vec<TeamBoxScore>),
}

impl StatPathFormatter for NBAStatKind {
    /// Returns the directory path associated with the `NBAStatKind`.
    ///
    /// # Returns
    /// A string slice (`&'static str`) representing the directory path.
    fn epath(&self) -> &'static str {
        match self {
            NBAStatKind::Team => "teamgames",
            NBAStatKind::Player => "playergames",
            NBAStatKind::LineUp => panic!("lineup stats are not supported yet."),
        }
    }

    /// Returns the file extension associated with the `NBAStatKind`.
    ///
    /// # Returns
    /// A string slice (`&'static str`) representing the file extension.
    fn ext(&self) -> &'static str {
        match self {
            NBAStatKind::Team => "tg.json",
            NBAStatKind::Player => "pg.json",
            NBAStatKind::LineUp => panic!("lineup stats are not supported yet."),
        }
    }

    /// Generates an error message for file opening failures.
    ///
    /// # Arguments
    /// * `season` - The season year (e.g., `2023` for the 2023-2024 season).
    ///
    /// # Returns
    /// A formatted error message string.
    fn dbg_open(&self, season: i32) -> String {
        let stat_description = match self {
            NBAStatKind::Team => "team",
            NBAStatKind::Player => "player",
            NBAStatKind::LineUp => panic!("lineup stats are not supported yet."),
        };

        format!(
            "ERROR: could not open file for the {}-{} season viewing {} box scores",
            season,
            (season + 1) % 100,
            stat_description
        )
    }

    /// Generates an error message for file writing failures.
    ///
    /// # Arguments
    /// * `season` - The season year (e.g., `2023` for the 2023-2024 season).
    ///
    /// # Returns
    /// A formatted error message string.
    fn dbg_write(&self, season: i32) -> String {
        let stat_description = match self {
            NBAStatKind::Team => "team",
            NBAStatKind::Player => "player",
            NBAStatKind::LineUp => panic!("lineup stats are not supported yet."),
        };

        format!(
            "ERROR:could not write the contents of the file for the {}-{} season viewing {} box scores",
            season,
            season % 100,
            stat_description
        )
    }
}
