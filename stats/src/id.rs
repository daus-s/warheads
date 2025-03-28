use format::language::columns;
use crate::box_score::BoxScore;

pub trait Identifiable {
    fn identity(&self) -> Option<Identity>;
}

#[derive(Eq, PartialEq)]
pub struct Identity {
    ///
    /// season: i32 is the year,
    /// from the original data source we subtract 20000
    ///
    /// Ex:
    ///
    ///     (1946-47 season -> 1946)
    ///
    pub season: i32,


    pub id: u64,
    pub game: u64,
}

///
/// this function should only ever accept well formatted strings so it will panic if not passed well.
/// thus it does not return a result but only a boolean
///
/// more importantly this function is asked "is this the game that i correct?"
/// so we answer true or false
///
impl Identifiable for String {
    fn identity(&self) -> Option<Identity> {
        let columns = columns(self.clone());

        match columns.as_slice() {
            [season_id, player_id, _, _, _, _, game_id, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _] => {

                let season = season_id.replace("\"", "").parse::<i32>().ok()? - 20000;

                let id = player_id.parse::<u64>().ok()?;

                let game = game_id.replace("\"", "").parse::<u64>().ok()?;

                Some(Identity {
                    season,
                    id,
                    game,
                })
            }
            _ => None
        }
    }
}

impl<T: BoxScore> Identifiable for T {
    fn identity(&self) -> Option<Identity> {
        Some(Identity{
            season: self.season(),
            id: self.id(),
            game: self.game().parse().unwrap(),
        })
    }
}

