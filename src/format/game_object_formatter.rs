use std::fmt::{Debug, Display};

use crate::stats::game_obj::GameObject;
use crate::stats::id::Identity;

impl Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let away = self.away();
        let home = self.home();

        let winner = if self.winner() == away.team_id {
            away.team_name()
        } else if self.winner() == home.team_id {
            home.team_name()
        } else {
            panic!("💀 if this error is arising check that your input box scores have opposite game result states to this function")
        };

        writeln!(
            f,
            //           "\033[1m{} @ {}\033[22m {}, {} win.", // todo: implement cooler cli.
            /*headline*/
            "{} @ {} {}, {} win.",
            away.team_abbr(),
            home.team_abbr(),
            self.game_date,
            winner,
        )?;
        write!(f, "\n{}\n{}", home, away)
    }
}

impl Debug for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}\n{}", self.game_identity(), self)
    }
}

// more advanced identity handling stuff for gameobject to both correction
// files specifically.
pub struct GameIdentity {
    home: Identity,
    away: Identity,
}
impl GameIdentity {
    pub fn new(home: Identity, away: Identity) -> Self {
        Self { home, away }
    }
}

impl Debug for GameIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "home: {:?}\naway: {:?}", self.home, self.away)
    }
}
