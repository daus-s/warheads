use crate::stats::player_box_score::PlayerBoxScoreBuilder;
use crate::stats::stat_column::StatColumn;
use crate::stats::team_box_score::TeamBoxScoreBuilder;

pub trait BoxScoreBuilder {
    fn add_stat<T>(&mut self, stat: T);
}

impl BoxScoreBuilder for PlayerBoxScoreBuilder {
    fn add_stat(&mut self, stat: Stat)
    {         match stat {
                    StatColumn::SEASON_ID => {
                        self.season_id(stat);
                    }
                    StatColumn::PLAYER_ID => {
                        self.player_id(stat);
                    }
                    StatColumn::PLAYER_NAME => {
                        self.player_name(stat);
                    }
                    StatColumn::TEAM_ID => {
                        self.team_id(stat);
                    }
                    StatColumn::TEAM_ABBREVIATION => {
                        self.team_abbreviation(stat);
                    }
                    StatColumn::TEAM_NAME => {
                        self.team_name(stat);
                    }
                    StatColumn::GAME_ID => {
                        self.game_id(stat);
                    }
                    StatColumn::GAME_DATE => {
                        self.game_date(stat);
                    }
                    StatColumn::MATCHUP => {
                        self.matchup(stat);
                    }
                    StatColumn::WL => {
                        self.wl(stat);
                    }
                    StatColumn::MIN => {
                        self.min(stat);
                    }
                    StatColumn::FGM => {
                        self.fgm(stat);
                    }
                    StatColumn::FGA => {
                        self.fga(stat);
                    }
                    StatColumn::FG3M => {
                        self.fg3m(stat);
                    }
                    StatColumn::FG3A => {
                        self.fg3a(stat);
                    }
                    StatColumn::FTM => {
                        self.ftm(stat);
                    }
                    StatColumn::FTA => {
                        self.fta(stat);
                    }
                    StatColumn::OREB => {
                        self.oreb(stat);
                    }
                    StatColumn::DREB => {
                        self.dreb(stat);
                    }
                    StatColumn::REB => {
                        self.reb(stat);
                    }
                    StatColumn::AST => {
                        self.ast(stat);
                    }
                    StatColumn::STL => {
                        self.stl(stat);
                    }
                    StatColumn::BLK => {
                        self.blk(stat);
                    }
                    StatColumn::TOV => {
                        self.tov(stat);
                    }
                    StatColumn::PF => {
                        self.pf(stat);
                    }
                    StatColumn::PTS => {
                        self.pts(stat);
                    }
                    StatColumn::PLUS_MINUS => {
                        self.plus_minus(stat);
                    }
                    StatColumn::FANTASY_PTS => {
                        self.fantasy_pts(stat);
                    }
                    StatColumn::VIDEO_AVAILABLE => {
                        panic!("💀 VideoAvailable is not a stat. (not implemented for BoxScore). ")
                    }
                    StatColumn::ELO => {
                        self.elo(stat);
                    }
                    StatColumn::FG_PCT  |
                    StatColumn::FG3_PCT |
                    StatColumn::FT_PCT  => {
                        panic!("💀 percentages are not implemented for BoxScores. ")
                    }
                }
            }
        }

impl BoxScoreBuilder for TeamBoxScoreBuilder {
    fn add_stat<T>(&mut self, stat: T) {
        match stat.col() {
            StatColumn::SEASON_ID => {
                self.season_id(stat);
            }
            StatColumn::TEAM_ID => {
                self.team_id(stat);
            }
            StatColumn::TEAM_ABBREVIATION => {
                self.team_abbreviation(stat);
            }
            StatColumn::TEAM_NAME => {
                self.team_name(stat);
            }
            StatColumn::GAME_ID => {
                self.game_id(stat);
            }
            StatColumn::GAME_DATE => {
                self.game_date(stat);
            }
            StatColumn::MATCHUP => {
                self.matchup(stat);
            }
            StatColumn::WL => {
                self.wl(stat);
            }
            StatColumn::MIN => {
                self.min(stat);
            }
            StatColumn::FGM => {
                self.fgm(stat);
            }
            StatColumn::FGA => {
                self.fga(stat);
            }
            StatColumn::FG3M => {
                self.fg3m(stat);
            }
            StatColumn::FG3A => {
                self.fg3a(stat);
            }
            StatColumn::FTM => {
                self.ftm(stat);
            }
            StatColumn::FTA => {
                self.fta(stat);
            }
            StatColumn::OREB => {
                self.oreb(stat);
            }
            StatColumn::DREB => {
                self.dreb(stat);
            }
            StatColumn::REB => {
                self.reb(stat);
            }
            StatColumn::AST => {
                self.ast(stat);
            }
            StatColumn::STL => {
                self.stl(stat);
            }
            StatColumn::BLK => {
                self.blk(stat);
            }
            StatColumn::TOV => {
                self.tov(stat);
            }
            StatColumn::PF => {
                self.pf(stat);
            }
            StatColumn::PTS => {
                self.pts(stat);
            }
            StatColumn::PLUS_MINUS => {
                self.plus_minus(stat);
            }
            StatColumn::VIDEO_AVAILABLE => {
                panic!("💀 VideoAvailable is not a stat. (not implemented for BoxScore). ")
            }
            StatColumn::PLAYER_ID   |
            StatColumn::PLAYER_NAME |
            StatColumn::FANTASY_PTS |
            StatColumn::ELO         => {
                panic!("💀 {} is not a team stat. PlayerName, PlayerId, FantasyPoints and ELO are all individual stats that cannot be added to a TeamBoxScore. ", stat.col())
            }
            StatColumn::FG_PCT  |
            StatColumn::FG3_PCT |
            StatColumn::FT_PCT  => {
                panic!("💀 percentages are not recorded for BoxScores. ")
            }
        }
    }
}