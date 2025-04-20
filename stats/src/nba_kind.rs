use format::stat_path_formatter::StatPathFormatter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum NBAStatKind {
    Team,
    Player,
    LineUp, //todo: develop this later-this is not a priority yet but may be very useful for elo and win-sharing.
}
///
/// **display**
///
impl Display for NBAStatKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NBAStatKind::Team => "T",
                NBAStatKind::Player => "P",
                NBAStatKind::LineUp => panic!("lineup url formatting is not supported"),
            }
        )
    }
}

impl StatPathFormatter for NBAStatKind {
    /// Returns the directory path associated with the `NBAStatKind`.
    ///
    /// # Returns
    /// A string slice (`&'static str`) representing the directory path.
    fn path_specifier(&self) -> &'static str {
        match self {
            NBAStatKind::Team => "teamgames",
            NBAStatKind::Player => "playergames",
            NBAStatKind::LineUp => todo!("lineup stats are not supported yet."),
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
}

impl NBAStatKind {
    /// Generates an error message for file opening failures.
    ///
    /// # Arguments
    /// * `season` - The season year (e.g., `2023` for the 2023-2024 season).
    ///
    /// # Returns
    /// A formatted error message string.
    pub fn dbg_open(&self, season: i32) -> String {
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
    pub fn dbg_write(&self, season: i32) -> String {
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
