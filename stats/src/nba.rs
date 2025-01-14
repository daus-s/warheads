use serde::{Deserialize, Serialize};

struct Resource {
    resource: String,

}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerBoxScore {
    season_id: String,
    player_id: String,
    player_name: String,
    team_id: String,
    team_abbreviation: String,
    team_name: String,
    game_id: String,
    game_date: String,
    matchup: String,
    wl: String,
    min: String,
    fgm: f32,
    fga: f32,
    fg_pct: f32,
    fg3m: f32,
    fg3a: f32,
    fg3_pct: f32,
    ftm: f32,
    fta: f32,
    ft_pct: f32,
    oreb: f32,
    dreb: f32,
    reb: f32,
    ast: f32,
    stl: f32,
    blk: f32,
    tov: f32,
    pf: f32,
    pts: f32,
    plus_minus: f32,
    fantasy_pts: f32,
    video_available: bool,
}