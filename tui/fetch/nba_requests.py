from datetime import datetime

import requests
from requests import Response


def fetch_nba_history(
    year: int,
    stage: str,
    from_date: datetime | None = None,
    to_date: datetime | None = None,
    team: bool = False,
):
    year_f = f"{year}-{str(year + 1)[-2:]}"

    url: str = f"https://stats.nba.com/stats/leaguegamelog?Counter=1000\
&DateFrom={from_date.strftime('%m/%d/%Y') if from_date else ''}\
&DateTo={to_date.strftime('%m/%d/%Y') if to_date else ''}\
&Direction=DESC\
&ISTRound=\
&LeagueID=00\
&PlayerOrTeam={team and 'T' or 'P'}\
&Season={year_f}\
&SeasonType={stage}\
&Sorter=DATE"

    print(url)

    headers: dict[str, str] = {
        "Host": "stats.nba.com",
        "User-Agent": "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:146.0) Gecko/20100101 Firefox/146.0",
        "Accept": "*/*",
        "Accept-Language": "en-US,en;q=0.5",
        "Accept-Encoding": "gzip, deflate, br, zstd",
        "Referer": "https://www.nba.com/",
        "Origin": "https://www.nba.com",
        "Connection": "keep-alive",
        "Sec-Fetch-Dest": "empty",
        "Sec-Fetch-Mode": "cors",
        "Sec-Fetch-Site": "same-site",
        "Priority": "u=4",
    }

    response: Response = requests.get(url, headers=headers)
    data = response.json()

    print(data)
    return data["resultSets"][0]["rowSet"]
