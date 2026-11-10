import json
import os
import sys
from datetime import datetime
from zoneinfo import ZoneInfo

import requests
from dotenv import load_dotenv

load_dotenv()


def main():
    API_KEY = os.environ["ODDS_API_KEY"]
    DIR = os.environ["SRC"]
    PT = ZoneInfo("America/Los_Angeles")

    # today's window in Pacific, converted to UTC
    today = datetime.now(PT).date()

    path = f"{DIR}/data/nba/oddsportal/nba_odds_{today}.json"
    # check if data is present so as to not burn credits
    try:
        with open(path, "r") as f:
            json.load(f)
        cont = False
    except (json.JSONDecodeError, PermissionError, FileNotFoundError):
        cont = True

    if not cont:
        print("data already exists")
        sys.exit(0)

    r = requests.get(
        "https://api.the-odds-api.com/v4/sports/basketball_nba/odds",
        params={
            "apiKey": API_KEY,
            "regions": "us",
            "markets": "h2h,spreads,totals",
            "oddsFormat": "decimal",
        },
        timeout=15,
    )
    r.raise_for_status()
    # header metadata
    for h in ("x-requests-remaining", "x-requests-used", "x-requests-last"):
        print(f"{h}: {r.headers.get(h)}")

    # save
    with open(path, "w") as f:
        json.dump(r.json(), f, indent=2)
    print(f"saved {len(r.json())} games -> {path}")


if __name__ == "__main__":
    main()
