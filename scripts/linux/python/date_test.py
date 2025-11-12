from datetime import date, timedelta


inc = timedelta(days=1)
date = date(2025, 1, 1)

while date.year < 2026:
    y = date.year

    if date.month < 4:
        period = "RegularSeason"
        y -= 1
    elif date.month == 4:
        if date.day <= 14:
            period = "RegularSeason"
            y -= 1

        elif date.day <= 17:
            period = "PlayIn"
            y -= 1

        else:
            period = "PostSeason"
            y -= 1

    elif date.month < 10:
        period = "PostSeason"
        y -= 1

    elif date.month == 10:
        if date.day <= 20:
            period = "PreSeason"
        else:
            period = "RegularSeason"
    else:
        period = "RegularSeason"

    print(
        f"assert_eq!(get_era_by_date(GameDate(from_ymd_opt({date.year}, {date.month}, {date.day}).unwrap())),SeasonId::from(({y}, SeasonPeriod::{period})));"
    )
    date = date + inc
