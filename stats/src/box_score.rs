pub trait BoxScore {
    fn season(&self) -> i32;

    fn game_id(&self) -> String;

    fn player_id(&self) -> Option<u64>;

    fn team_id(&self) -> u64;

    fn team_abbr(&self) -> String;
}