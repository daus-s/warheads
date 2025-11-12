#[cfg(test)]
mod test_season_manager {
    use crate::dapi::season_manager::get_era_by_date;
    use crate::stats::season_period::SeasonPeriod;
    use crate::types::{GameDate, SeasonId};
    use chrono::NaiveDate;

    #[test]
    fn test_get_daily_era() {
        let from_ymd_opt = NaiveDate::from_ymd_opt;

        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 1, 31).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 2, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 3, 31).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PlayIn))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PlayIn))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PlayIn))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 4, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 5, 31).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 6, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 7, 31).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 8, 31).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 1).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 2).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 3).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 4).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 5).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 6).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 7).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 8).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 9).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 10).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 11).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 12).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 13).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 14).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 15).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 16).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 17).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 18).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 19).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 20).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 21).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 22).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 23).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 24).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 25).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 26).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 27).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 28).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 29).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 9, 30).unwrap())),
            SeasonId::from((2024, SeasonPeriod::PostSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 1).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 2).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 3).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 4).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 5).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 6).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 7).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 8).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 9).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 10).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 11).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 12).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 13).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 14).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 15).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 16).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 17).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 18).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 19).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 20).unwrap())),
            SeasonId::from((2025, SeasonPeriod::PreSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 21).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 22).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 23).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 24).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 25).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 26).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 27).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 28).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 29).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 30).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 10, 31).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 1).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 2).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 3).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 4).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 5).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 6).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 7).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 8).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 9).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 10).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 11).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 12).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 13).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 14).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 15).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 16).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 17).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 18).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 19).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 20).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 21).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 22).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 23).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 24).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 25).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 26).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 27).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 28).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 29).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 11, 30).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 1).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 2).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 3).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 4).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 5).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 6).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 7).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 8).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 9).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 10).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 11).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 12).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 13).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 14).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 15).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 16).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 17).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 18).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 19).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 20).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 21).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 22).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 23).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 24).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 25).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 26).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 27).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 28).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 29).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 30).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
        assert_eq!(
            get_era_by_date(GameDate(from_ymd_opt(2025, 12, 31).unwrap())),
            SeasonId::from((2025, SeasonPeriod::RegularSeason))
        );
    }
}
