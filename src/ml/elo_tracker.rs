use crate::constants::paths::data;
use crate::types::{GameId, PlayerId};
use csv::Writer;
use once_cell::sync::Lazy;

type PlayerGame = (GameId, PlayerId);

pub struct EloTracker {
    game_player_ratings: Vec<(PlayerGame, i64)>,
}

impl EloTracker {
    pub fn process_elo(&self) {
        // load season by season, dont nuke the memory will all tge history.
        todo!("assign elo values to players on a game by game basis")
    }

    pub fn save(&self) -> Result<(), String> {
        let filename = save_path("elo.csv"); //todo: add customizability for different models here

        let mut writer = Writer::from_path(&filename)
            .map_err(|e| format!("❌ failed to open a writer for {filename}: {e}"))?;

        for ((GameId(game), PlayerId(player)), elo) in &self.game_player_ratings {
            match writer.serialize(&[*game as i64, *player as i64, *elo]) {
                Ok(_) => {
                    eprintln!(
                        "✅ successfully wrote record for {player} in {game}: {}{elo}",
                        match *elo < 0 {
                            true => "",
                            false => "+",
                        }
                    );
                }
                Err(e) => {
                    return Err(format!(
                        "❌ failed to write record for {player} in {game}: {e}"
                    ));
                }
            };
        }

        Ok(())
    }
}

fn save_path(filename: &str) -> String {
    static DATA: Lazy<String> = Lazy::new(data);

    format!("{}/nba/elo/{}", *DATA, filename)
}
