use stats::id::{Identifiable, Identity};
use stats::nba_kind::NBAStatKind;
use crate::correction::Correction;

impl Identifiable for Correction {
    fn identity(&self) -> Option<Identity> {
        // eprintln!("parsing gameid as u64: {}", match self.gameid.replace("\"", "").parse::<u64>() {
        //     Ok(_) => "success",
        //     Err(_) => "failure"
        // });

        match self.kind {
            NBAStatKind::Team =>
                Some(
                    Identity {
                        szn: self.season,
                        game_id: self.game_id.replace("\"", ""),
                        player_id: None,
                        team_id: self.team_id,
                        team_abbr: self.team_abbr.clone(),
                    }
                )
            ,
            NBAStatKind::Player => {
                Some(Identity {
                    szn: self.season,
                    game_id: self.game_id.replace("\"", "").parse().unwrap(),
                    player_id: Some(self.player_id.unwrap_or_else(|| panic!("no player id for a player correction object. "))),
                    team_id: self.team_id,
                    team_abbr: self.team_abbr.clone(),
                })
            },
            NBAStatKind::LineUp => todo!("lineup stats not yet implemented"),
        }
    }
}
