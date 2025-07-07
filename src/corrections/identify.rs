use crate::corrections::correction::Correction;
use crate::stats::id::{Identifiable, Identity};
use crate::stats::nba_kind::NBAStatKind;

impl Identifiable for Correction {
    fn identity(&self) -> Identity {
        // eprintln!("parsing gameid as u64: {}", match self.gameid.replace("\"", "").parse::<u64>() {
        //     Ok(_) => "success",
        //     Err(_) => "failure"
        // });

        match self.kind {
            NBAStatKind::Team => Identity {
                season_id: self.season,
                game_id: self.game_id.clone(),
                player_id: None,
                team_id: self.team_id,
                team_abbr: self.team_abbr.clone(),
            },
            NBAStatKind::Player => {
                Identity {
                    season_id: self.season,
                    game_id: self.game_id.clone(),
                    player_id: Some(self.player_id.unwrap_or_else(|| {
                        panic!("💀 no player id for a player correction object. ")
                    })),
                    team_id: self.team_id,
                    team_abbr: self.team_abbr.clone(),
                }
            }
            NBAStatKind::LineUp => todo!("lineup stats not yet implemented"),
        }
    }
}
