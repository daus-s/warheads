use crate::dapi::box_score_stat::BoxScoreStat;
use crate::stats::player_box_score::PlayerBoxScoreBuilder;
use crate::stats::team_box_score::TeamBoxScoreBuilder;

pub trait BoxScoreBuilder {
    fn add_stat(&mut self, stat: BoxScoreStat);
}

impl BoxScoreBuilder for PlayerBoxScoreBuilder {
    fn add_stat(&mut self, stat: BoxScoreStat) {
        match stat {
            BoxScoreStat::SeasonId(s) => {
                self.season_id(s);
            }
            BoxScoreStat::PlayerId(s) => {
                self.player_id(s);
            }
            BoxScoreStat::PlayerName(s) => {
                self.player_name(s);
            }
            BoxScoreStat::TeamId(s) => {
                self.team_id(s);
            }
            BoxScoreStat::TeamAbbreviation(s) => {
                self.team_abbreviation(s);
            }
            BoxScoreStat::TeamName(s) => {
                self.team_name(s);
            }
            BoxScoreStat::GameId(s) => {
                self.game_id(s);
            }
            BoxScoreStat::GameDate(s) => {
                self.game_date(s);
            }
            BoxScoreStat::MatchupString(s) => {
                self.matchup(s);
            }
            BoxScoreStat::GameResult(s) => {
                self.wl(s);
            }
            BoxScoreStat::Minutes(s) => {
                self.min(s);
            }
            BoxScoreStat::FieldGoalMakes(s) => {
                self.fgm(s);
            }
            BoxScoreStat::FieldGoalAttempts(s) => {
                self.fga(s);
            }
            BoxScoreStat::ThreePointMakes(s) => {
                self.fg3m(s);
            }
            BoxScoreStat::ThreePointAttempts(s) => {
                self.fg3a(s);
            }
            BoxScoreStat::FreeThrowMakes(s) => {
                self.ftm(s);
            }
            BoxScoreStat::FreeThrowAttempts(s) => {
                self.fta(s);
            }
            BoxScoreStat::OffensiveRebounds(s) => {
                self.oreb(s);
            }
            BoxScoreStat::DefensiveRebounds(s) => {
                self.dreb(s);
            }
            BoxScoreStat::Rebounds(s) => {
                self.reb(s);
            }
            BoxScoreStat::Assists(s) => {
                self.ast(s);
            }
            BoxScoreStat::Steals(s) => {
                self.stl(s);
            }
            BoxScoreStat::Blocks(s) => {
                self.blk(s);
            }
            BoxScoreStat::Turnovers(s) => {
                self.tov(s);
            }
            BoxScoreStat::PersonalFouls(s) => {
                self.pf(s);
            }
            BoxScoreStat::Points(s) => {
                self.pts(s);
            }
            BoxScoreStat::PlusMinus(s) => {
                self.plus_minus(s);
            }
            BoxScoreStat::FantasyPoints(s) => {
                self.fantasy_pts(s);
            }
            BoxScoreStat::Elo(s) => {
                self.elo(s);
            }
        }
    }
}

impl BoxScoreBuilder for TeamBoxScoreBuilder {
    fn add_stat(&mut self, stat: BoxScoreStat) {
        match stat {
            BoxScoreStat::SeasonId(s) => {
                self.season_id(s);
            }
            BoxScoreStat::TeamId(s) => {
                self.team_id(s);
            }
            BoxScoreStat::TeamAbbreviation(s) => {
                self.team_abbreviation(s);
            }
            BoxScoreStat::TeamName(s) => {
                self.team_name(s);
            }
            BoxScoreStat::GameId(s) => {
                self.game_id(s);
            }
            BoxScoreStat::GameDate(s) => {
                self.game_date(s);
            }
            BoxScoreStat::MatchupString(s) => {
                self.matchup(s);
            }
            BoxScoreStat::GameResult(s) => {
                self.wl(s);
            }
            BoxScoreStat::Minutes(s) => {
                self.min(s);
            }
            BoxScoreStat::FieldGoalMakes(s) => {
                self.fgm(s);
            }
            BoxScoreStat::FieldGoalAttempts(s) => {
                self.fga(s);
            }
            BoxScoreStat::ThreePointMakes(s) => {
                self.fg3m(s);
            }
            BoxScoreStat::ThreePointAttempts(s) => {
                self.fg3a(s);
            }
            BoxScoreStat::FreeThrowMakes(s) => {
                self.ftm(s);
            }
            BoxScoreStat::FreeThrowAttempts(s) => {
                self.fta(s);
            }
            BoxScoreStat::OffensiveRebounds(s) => {
                self.oreb(s);
            }
            BoxScoreStat::DefensiveRebounds(s) => {
                self.dreb(s);
            }
            BoxScoreStat::Rebounds(s) => {
                self.reb(s);
            }
            BoxScoreStat::Assists(s) => {
                self.ast(s);
            }
            BoxScoreStat::Steals(s) => {
                self.stl(s);
            }
            BoxScoreStat::Blocks(s) => {
                self.blk(s);
            }
            BoxScoreStat::Turnovers(s) => {
                self.tov(s);
            }
            BoxScoreStat::PersonalFouls(s) => {
                self.pf(s);
            }
            BoxScoreStat::Points(s) => {
                self.pts(s);
            }
            BoxScoreStat::PlusMinus(s) => {
                self.plus_minus(s);
            }
            BoxScoreStat::FantasyPoints(_)
            | BoxScoreStat::PlayerId(_)
            | BoxScoreStat::PlayerName(_)
            | BoxScoreStat::Elo(_) => {
                panic!("💀 cannot add a player only stat to a TeamBoxScore. ")
            }
        }
    }
}
