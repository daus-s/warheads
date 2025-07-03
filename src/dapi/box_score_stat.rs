use crate::types::{
    Assists, Blocks, DefensiveRebounds, Elo, FantasyPoints, FieldGoalAttempts, FieldGoalMakes,
    FreeThrowAttempts, FreeThrowMakes, GameDate, GameId, GameResult, MatchupString, Minutes,
    OffensiveRebounds, PersonalFouls, PlayerId, PlayerName, PlusMinus, Points, Rebounds, SeasonId,
    Steals, TeamAbbreviation, TeamId, TeamName, ThreePointAttempts, ThreePointMakes, Turnovers,
};

/// box_score_stat is an enum that contains every type that may be used by a `BoxScore`
/// all fields are parsed and translated into this format from the data source

pub enum BoxScoreStat {
    SeasonId(SeasonId),
    PlayerId(PlayerId),
    PlayerName(PlayerName),
    TeamId(TeamId),
    TeamAbbreviation(TeamAbbreviation),
    TeamName(TeamName),
    GameId(GameId),
    GameDate(GameDate),
    MatchupString(MatchupString),
    GameResult(GameResult),
    Minutes(Minutes),
    FieldGoalMakes(FieldGoalMakes),
    FieldGoalAttempts(FieldGoalAttempts),
    ThreePointMakes(ThreePointMakes),
    ThreePointAttempts(ThreePointAttempts),
    FreeThrowMakes(FreeThrowMakes),
    FreeThrowAttempts(FreeThrowAttempts),
    OffensiveRebounds(OffensiveRebounds),
    DefensiveRebounds(DefensiveRebounds),
    Rebounds(Rebounds),
    Assists(Assists),
    Steals(Steals),
    Blocks(Blocks),
    Turnovers(Turnovers),
    PersonalFouls(PersonalFouls),
    Points(Points),
    PlusMinus(PlusMinus),
    FantasyPoints(FantasyPoints),
    Elo(Elo),
}

impl From<SeasonId> for BoxScoreStat {
    fn from(value: SeasonId) -> Self {
        BoxScoreStat::SeasonId(value)
    }
}
impl From<PlayerId> for BoxScoreStat {
    fn from(value: PlayerId) -> Self {
        BoxScoreStat::PlayerId(value)
    }
}
impl From<PlayerName> for BoxScoreStat {
    fn from(value: PlayerName) -> Self {
        BoxScoreStat::PlayerName(value)
    }
}
impl From<TeamId> for BoxScoreStat {
    fn from(value: TeamId) -> Self {
        BoxScoreStat::TeamId(value)
    }
}
impl From<TeamAbbreviation> for BoxScoreStat {
    fn from(value: TeamAbbreviation) -> Self {
        BoxScoreStat::TeamAbbreviation(value)
    }
}
impl From<TeamName> for BoxScoreStat {
    fn from(value: TeamName) -> Self {
        BoxScoreStat::TeamName(value)
    }
}
impl From<GameId> for BoxScoreStat {
    fn from(value: GameId) -> Self {
        BoxScoreStat::GameId(value)
    }
}
impl From<GameDate> for BoxScoreStat {
    fn from(value: GameDate) -> Self {
        BoxScoreStat::GameDate(value)
    }
}
impl From<MatchupString> for BoxScoreStat {
    fn from(value: MatchupString) -> Self {
        BoxScoreStat::MatchupString(value)
    }
}
impl From<GameResult> for BoxScoreStat {
    fn from(value: GameResult) -> Self {
        BoxScoreStat::GameResult(value)
    }
}
impl From<Minutes> for BoxScoreStat {
    fn from(value: Minutes) -> Self {
        BoxScoreStat::Minutes(value)
    }
}
impl From<FieldGoalMakes> for BoxScoreStat {
    fn from(value: FieldGoalMakes) -> Self {
        BoxScoreStat::FieldGoalMakes(value)
    }
}
impl From<FieldGoalAttempts> for BoxScoreStat {
    fn from(value: FieldGoalAttempts) -> Self {
        BoxScoreStat::FieldGoalAttempts(value)
    }
}
impl From<ThreePointMakes> for BoxScoreStat {
    fn from(value: ThreePointMakes) -> Self {
        BoxScoreStat::ThreePointMakes(value)
    }
}
impl From<ThreePointAttempts> for BoxScoreStat {
    fn from(value: ThreePointAttempts) -> Self {
        BoxScoreStat::ThreePointAttempts(value)
    }
}
impl From<FreeThrowMakes> for BoxScoreStat {
    fn from(value: FreeThrowMakes) -> Self {
        BoxScoreStat::FreeThrowMakes(value)
    }
}
impl From<FreeThrowAttempts> for BoxScoreStat {
    fn from(value: FreeThrowAttempts) -> Self {
        BoxScoreStat::FreeThrowAttempts(value)
    }
}
impl From<OffensiveRebounds> for BoxScoreStat {
    fn from(value: OffensiveRebounds) -> Self {
        BoxScoreStat::OffensiveRebounds(value)
    }
}
impl From<DefensiveRebounds> for BoxScoreStat {
    fn from(value: DefensiveRebounds) -> Self {
        BoxScoreStat::DefensiveRebounds(value)
    }
}
impl From<Rebounds> for BoxScoreStat {
    fn from(value: Rebounds) -> Self {
        BoxScoreStat::Rebounds(value)
    }
}
impl From<Assists> for BoxScoreStat {
    fn from(value: Assists) -> Self {
        BoxScoreStat::Assists(value)
    }
}
impl From<Steals> for BoxScoreStat {
    fn from(value: Steals) -> Self {
        BoxScoreStat::Steals(value)
    }
}
impl From<Blocks> for BoxScoreStat {
    fn from(value: Blocks) -> Self {
        BoxScoreStat::Blocks(value)
    }
}
impl From<Turnovers> for BoxScoreStat {
    fn from(value: Turnovers) -> Self {
        BoxScoreStat::Turnovers(value)
    }
}
impl From<PersonalFouls> for BoxScoreStat {
    fn from(value: PersonalFouls) -> Self {
        BoxScoreStat::PersonalFouls(value)
    }
}
impl From<Points> for BoxScoreStat {
    fn from(value: Points) -> Self {
        BoxScoreStat::Points(value)
    }
}
impl From<PlusMinus> for BoxScoreStat {
    fn from(value: PlusMinus) -> Self {
        BoxScoreStat::PlusMinus(value)
    }
}
impl From<FantasyPoints> for BoxScoreStat {
    fn from(value: FantasyPoints) -> Self {
        BoxScoreStat::FantasyPoints(value)
    }
}
impl From<Elo> for BoxScoreStat {
    fn from(value: Elo) -> Self {
        BoxScoreStat::Elo(value)
    }
}
