#[cfg(test)]
mod test_serde_type_support {
    use crate::stats::percent::PercentageFormatter;
    use crate::stats::visiting::Visiting::{Away, Home};
    use crate::types::fantasy_points::FantasyFormatter;
    use crate::types::*;
    use chrono::NaiveDate;
    use serde::{Serialize, Serializer};
    use std::str::FromStr;

    #[test]
    fn test_serialize_ast() {
        let assists = Assists(Some(1));

        let json_str = serde_json::to_string(&assists).expect("couldn't serialize assists. ");

        assert_eq!(json_str, "1")
    }

    #[test]
    fn test_serialize_blk() {
        let blocks = Blocks(Some(1));

        let json_str = serde_json::to_string(&blocks).expect("couldn't serialize blocks. ");

        assert_eq!(json_str, "1")
    }

    #[test]
    fn test_serialize_fantasy_pts() {
        let fantasy_points = FantasyPoints(Some(69.420));

        let mut out: Vec<u8> = Vec::with_capacity(4);

        let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, FantasyFormatter);

        fantasy_points.serialize(&mut ser).unwrap();

        let slices = out.as_slice();

        let json_str = String::from_utf8_lossy(slices);

        assert_eq!(json_str, "69.4")
    }

    #[test]
    fn test_serialize_fgm() {
        let fgm = FieldGoalMakes(10);

        let json_str = serde_json::to_string(&fgm).expect("couldn't serialize field goal makes. ");

        assert_eq!(json_str, "10")
    }

    #[test]
    fn test_serialize_fga() {
        let fga = FieldGoalAttempts(Some(15));

        let json_str =
            serde_json::to_string(&fga).expect("couldn't serialize field goal attempts. ");

        assert_eq!(json_str, "15")
    }

    #[test]
    fn test_serialize_fg_pct() {
        let fg_pct = FieldGoalPercentage(Some(10. / 15.));

        let mut out: Vec<u8> = Vec::with_capacity(8);

        let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, PercentageFormatter);

        fg_pct.serialize(&mut ser).unwrap();

        let slices = out.as_slice();

        let json_str = String::from_utf8_lossy(slices);

        assert_eq!(json_str, "0.666667")
    }

    #[test]
    fn test_serialize_ftm() {
        let ftm = FreeThrowMakes(3);

        let json_str = serde_json::to_string(&ftm).expect("couldn't serialize free throw makes. ");

        assert_eq!(json_str, "3")
    }

    #[test]
    fn test_serialize_fta() {
        let fta = FreeThrowAttempts(Some(4));

        let json_str =
            serde_json::to_string(&fta).expect("couldn't serialize free throw attempts. ");

        assert_eq!(json_str, "4")
    }

    #[test]
    fn test_serialize_ft_pct() {
        let ft_pct = FieldGoalPercentage(Some(3. / 4.));

        let mut out: Vec<u8> = Vec::with_capacity(8);

        let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, PercentageFormatter);

        ft_pct.serialize(&mut ser).unwrap();

        let slices = out.as_slice();

        let json_str = String::from_utf8_lossy(slices);

        assert_eq!(json_str, "0.750000")
    }

    #[test]
    fn test_serialize_game_id() {
        let game_id = GameId::from("0001010101");

        let json_str = serde_json::to_string(&game_id).expect("couldn't serialize assists. ");

        assert_eq!(json_str, "\"0001010101\"")
    }

    #[test]
    fn test_serialize_wl() {
        let win = GameResult::Win;
        let loss = GameResult::Loss;

        let json_str_w = serde_json::to_string(&win).expect("couldn't serialize GameResult::Win. ");
        let json_str_l =
            serde_json::to_string(&loss).expect("couldn't serialize GameResult::Loss. ");

        assert_eq!(json_str_w, "\"W\"");
        assert_eq!(json_str_l, "\"L\"");
    }

    #[test]
    fn test_serialize_game_date() {
        let assists = GameDate(NaiveDate::from_ymd_opt(2000, 2, 5).unwrap());

        let json_str = serde_json::to_string(&assists).expect("couldn't serialize game date. ");

        assert_eq!(json_str, "\"2000-02-05\"")
    }

    #[test]
    fn test_serialize_matchup() {
        let sonics =
            Matchup::from_matchup("SON".parse().unwrap(), "OKC".parse().unwrap());

        let bums =
            Matchup::from_matchup("SON".parse().unwrap(), "OKC".parse().unwrap());

        let sonics_str =
            serde_json::to_string(&sonics).expect("couldn't serialize Sonic's matchup. ");

        let bum_str =
            serde_json::to_string(&bums).expect("couldn't serialize the BUMS's matchup. ");

        assert_eq!(sonics_str, "\"SON vs. OKC\"");
        assert_eq!(bum_str, "\"OKC @ SON\"");
    }

    #[test]
    fn test_serialize_min() {
        let minutes = Minutes(48);

        let json_str = serde_json::to_string(&minutes).expect("couldn't serialize minutes. ");

        assert_eq!(json_str, "48")
    }

    #[test]
    fn test_serialize_pf() {
        let personal_fouls = PersonalFouls(6);

        let json_str =
            serde_json::to_string(&personal_fouls).expect("couldn't serialize personal fouls. ");

        assert_eq!(json_str, "6")
    }

    #[test]
    fn test_serialize_player_name() {
        let player_name = PlayerName("LeGoat Al-Gaib".to_string());

        let json_str =
            serde_json::to_string(&player_name).expect("couldn't serialize player_name. ");

        assert_eq!(json_str, "\"LeGoat Al-Gaib\"")
    }

    #[test]
    fn test_serialize_player_id() {
        let player_id = PlayerId(112358321);

        let json_str = serde_json::to_string(&player_id).expect("couldn't serialize assists. ");

        assert_eq!(json_str, "112358321")
    }

    #[test]
    fn test_serialize_plus_minus() {
        let neg = PlusMinus(Some(-40));
        let pos = PlusMinus(Some(23));

        let json_str_n =
            serde_json::to_string(&neg).expect("couldn't serialize negative plus minus. ");
        let json_str_p =
            serde_json::to_string(&pos).expect("couldn't serialize positive plus minus. ");

        assert_eq!(json_str_n, "-40");
        assert_eq!(json_str_p, "23");
    }

    #[test]
    fn test_serialize_pts() {
        let points = Points(23);

        let json_str = serde_json::to_string(&points).expect("couldn't serialize points. ");

        assert_eq!(json_str, "23")
    }

    #[test]
    fn test_serialize_reb() {
        let rebounds = Rebounds(Some(10));

        let json_str = serde_json::to_string(&rebounds).expect("couldn't serialize rebounds. ");

        assert_eq!(json_str, "10");
    }

    #[test]
    fn test_serialize_oreb() {
        let oreb = OffensiveRebounds(Some(10));

        let json_str =
            serde_json::to_string(&oreb).expect("couldn't serialize offensive rebounds. ");

        assert_eq!(json_str, "10");
    }

    #[test]
    fn test_serialize_dreb() {
        let dreb = DefensiveRebounds(Some(10));

        let json_str =
            serde_json::to_string(&dreb).expect("couldn't serialize defensive rebounds. ");

        assert_eq!(json_str, "10")
    }

    #[test]
    fn test_serialize_season_id() {
        let season_id = SeasonId::from(22000);

        let json_str = serde_json::to_string(&season_id).expect("couldn't serialize season id. ");

        assert_eq!(json_str, "\"22000\"")
    }

    #[test]
    fn test_serialize_stl() {
        let steals = Steals(Some(5));

        let json_str = serde_json::to_string(&steals).expect("couldn't serialize steals. ");

        assert_eq!(json_str, "5")
    }

    #[test]
    fn test_serialize_team_id() {
        let team_id = TeamId(14141);

        let json_str = serde_json::to_string(&team_id).expect("couldn't serialize team id. ");

        assert_eq!(json_str, "14141")
    }

    #[test]
    fn test_serialize_team_name() {
        let assists = TeamName("Seattle Supersonics".to_string());

        let json_str = serde_json::to_string(&assists).expect("couldn't serialize team name. ");

        assert_eq!(json_str, "\"Seattle Supersonics\"")
    }

    #[test]
    fn test_serialize_team_abbr() {
        let assists = TeamAbbreviation::from_str("SON").unwrap();

        let json_str =
            serde_json::to_string(&assists).expect("couldn't serialize team abbreviation. ");

        assert_eq!(json_str, "\"SON\"")
    }

    #[test]
    fn test_serialize_fg3m() {
        let fg3m = ThreePointMakes(Some(4));

        let json_str =
            serde_json::to_string(&fg3m).expect("couldn't serialize three point makes. ");

        assert_eq!(json_str, "4")
    }

    #[test]
    fn test_serialize_fg3a() {
        let fg3a = ThreePointAttempts(Some(7));

        let json_str =
            serde_json::to_string(&fg3a).expect("couldn't serialize three point attempts. ");

        assert_eq!(json_str, "7")
    }

    #[test]
    fn test_serialize_fg3_pct() {
        let fg3_pct = ThreePointPercentage(Some(4. / 7.));

        let mut out: Vec<u8> = Vec::with_capacity(8);

        let mut ser = serde_json::ser::Serializer::with_formatter(&mut out, PercentageFormatter);

        fg3_pct.serialize(&mut ser).unwrap();

        let slices = out.as_slice();

        let json_str = String::from_utf8_lossy(slices);

        assert_eq!(json_str, "0.571429")
    }

    #[test]
    fn test_serialize_tov() {
        let tov = Turnovers(Some(2));

        let json_str = serde_json::to_string(&tov).expect("couldn't serialize turnovers. ");

        assert_eq!(json_str, "2")
    }
}
