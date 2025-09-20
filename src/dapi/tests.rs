#[cfg(test)]
mod test_urls {
    use crate::format::url_format::UrlFormatter;
    use crate::stats::nba_kind::NBAStatKind::{Player, Team};
    use crate::stats::season_period::SeasonPeriod::{PlayIn, PostSeason, PreSeason, RegularSeason};
    use crate::types::SeasonId;

    #[test]
    fn test_prep_2010() {
        let season = SeasonId::from((2010, PreSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season=2010-11&SeasonType=Pre%20Season&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Player.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_pret_2012() {
        let season = SeasonId::from((2012, PreSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=T&Season=2012-13&SeasonType=Pre%20Season&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Team.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_regp_1946() {
        let season = SeasonId::from((1946, RegularSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season=1946-47&SeasonType=Regular%20Season&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Player.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_regt_2000() {
        let season = SeasonId::from((2000, RegularSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=T&Season=2000-01&SeasonType=Regular%20Season&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Team.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_pinp_2021() {
        let season = SeasonId::from((2021, PlayIn));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season=2021-22&SeasonType=PlayIn&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Player.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_pint_2024() {
        let season = SeasonId::from((2024, PlayIn));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=T&Season=2024-25&SeasonType=PlayIn&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Team.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_posp_1979() {
        let season = SeasonId::from((1979, PostSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season=1979-80&SeasonType=Playoffs&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Player.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
    #[test]
    fn test_post_1980() {
        let season = SeasonId::from((1980, PostSeason));

        let exp = "https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=T&Season=1980-81&SeasonType=Playoffs&Sorter=DATE";

        let act = format!(
            "\
        https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&\
        Direction=DESC&ISTRound=&\
        LeagueID=00&\
        PlayerOrTeam={}&\
        Season={}&\
        SeasonType={}&\
        Sorter=DATE",
            Team.url(),
            season.year().url(),
            season.period().url()
        );

        assert_eq!(exp, act)
    }
}

