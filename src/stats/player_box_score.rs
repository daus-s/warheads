use crate::stats::box_score::BoxScore;
use crate::stats::stat_column::StatColumn;
use crate::stats::stat_value::StatValue;
use crate::stats::team_box_score::TeamBoxScore;
use crate::stats::visiting::Visiting;
use crate::types::*;
use derive_builder::Builder;
use serde::Serialize;
use std::fmt::Formatter;

#[derive(Builder, Clone, Debug, Serialize)]
pub struct PlayerBoxScore {
    //team identification
    team_abbreviation: TeamAbbreviation,
    team_name: TeamName,
    team_id: TeamId,

    // game data
    season_id: SeasonId,
    game_date: GameDate,
    game_id: GameId,
    matchup: MatchupString,

    // player data
    player_id: PlayerId,
    player_name: PlayerName,

    // classic box score
    wl: GameResult,

    min: Minutes,

    fgm: FieldGoalMakes,
    fga: FieldGoalAttempts,

    fg3m: ThreePointMakes,
    fg3a: ThreePointAttempts,

    ftm: FreeThrowMakes,
    fta: FreeThrowAttempts,

    oreb: OffensiveRebounds,
    dreb: DefensiveRebounds,
    reb: Rebounds,

    ast: Assists,

    stl: Steals,

    blk: Blocks,

    tov: Turnovers,

    pf: PersonalFouls, //personal fouls
    pts: Points,

    //advanced stats
    plus_minus: PlusMinus,
    fantasy_pts: FantasyPoints,
    elo: Elo, // decisions, decisions
}

impl std::fmt::Display for PlayerBoxScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\npts:{}\tfgs:{}/{}\t3ps:{}/{}\tft: {}/{}\nast:{}\nreb:{} (off {},def {})\nstl: {}\t blk:{}\ttov:{}\nfantasy: {}\n",
               self.player_name.to_string().to_ascii_uppercase(), self.pts, self.fgm, self.fga,
               self.fg3m, self.fg3a, self.ftm, self.fta, self.ast, self.reb, self.oreb, self.dreb,
               self.stl, self.blk, self.tov, self.fantasy_pts
        )
    }
}

impl PlayerBoxScore {
    pub fn team(&self) -> TeamAbbreviation {
        self.team_abbreviation.clone()
    }

    pub fn played_in(&self, game: &TeamBoxScore) -> bool {
        self.game_id == game.game_id() && self.team_abbreviation == game.team_abbr()
    }
}

impl BoxScore for PlayerBoxScore {
    fn season(&self) -> SeasonId {
        self.season_id
    }

    fn game_id(&self) -> &GameId {
        &self.game_id
    }

    fn player_id(&self) -> Option<PlayerId> {
        Some(self.player_id)
    }

    fn team_id(&self) -> TeamId {
        self.team_id
    }

    fn team_abbr(&self) -> &TeamAbbreviation {
        &self.team_abbreviation
    }

    fn home_or_away(&self) -> Visiting {
        self.matchup.home_or_away()
    }

    fn set(&mut self, col: &StatColumn, val: &StatValue) {
        // match col {
        //     StatColumn::SEASON_ID => self.season_id = SeasonId::from(val),
        //     StatColumn::PLAYER_ID => {}
        //     StatColumn::PLAYER_NAME => {}
        //     StatColumn::TEAM_ID => {}
        //     StatColumn::TEAM_ABBREVIATION => {}
        //     StatColumn::TEAM_NAME => {}
        //     StatColumn::GAME_ID => {}
        //     StatColumn::GAME_DATE => {}
        //     StatColumn::MATCHUP => {}
        //     StatColumn::WL => {}
        //     StatColumn::MIN => {}
        //     StatColumn::FGM => {}
        //     StatColumn::FGA => {}
        //     StatColumn::FG_PCT => {}
        //     StatColumn::FG3M => {}
        //     StatColumn::FG3A => {}
        //     StatColumn::FG3_PCT => {}
        //     StatColumn::FTM => {}
        //     StatColumn::FTA => {}
        //     StatColumn::FT_PCT => {}
        //     StatColumn::OREB => {}
        //     StatColumn::DREB => {}
        //     StatColumn::REB => {}
        //     StatColumn::AST => {}
        //     StatColumn::STL => {}
        //     StatColumn::BLK => {}
        //     StatColumn::TOV => {}
        //     StatColumn::PF => {}
        //     StatColumn::PTS => {}
        //     StatColumn::PLUS_MINUS => {}
        //     StatColumn::FANTASY_PTS => {}
        //     StatColumn::VIDEO_AVAILABLE => {}
        // }
        todo!()
    }
}
