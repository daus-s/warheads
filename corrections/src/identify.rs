use stats::id::{Identifiable, Identity};
use crate::correction::Correction;

impl Identifiable for Correction {
    fn identity(&self) -> Option<Identity> {
        // eprintln!("parsing gameid as u64: {}", match self.gameid.replace("\"", "").parse::<u64>() {
        //     Ok(_) => "success",
        //     Err(_) => "failure"
        // });

        Some(Identity {
            season: self.season,
            game: self.gameid.replace("\"", "").parse().unwrap(),
            id: self.id,
        })
    }
}
