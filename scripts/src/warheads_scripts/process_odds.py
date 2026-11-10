import json
import os
import statistics
import sys
from datetime import datetime
from zoneinfo import ZoneInfo

import pandas as pd
from dotenv import load_dotenv

load_dotenv()

DIR = os.environ["SRC"]


class BettingData:
    def __init__(
        self,
        date,
        home,
        away,
        spread,
        total,
        moneyline_away,
        moneyline_home,
    ):
        self.date = date
        self.home = home
        self.away = away
        self.spread = spread
        self.total = total
        self.moneyline_away = moneyline_away
        self.moneyline_home = moneyline_home
        self.favorite = "home" if moneyline_away > moneyline_home else "away"

    def __str__(self) -> str:
        return (
            f"Date: {self.date}\n"
            f"Home Team: {self.home}\n"
            f"Away Team: {self.away}\n"
            f"Spread: {self.spread}\n"
            f"Total: {self.total}\n"
            f"Moneyline Away: {self.moneyline_away}\n"
            f"Moneyline Home: {self.moneyline_home}\n"
            f"Favorite: {self.favorite}"
        )


def main():

    args = sys.argv

    try:
        date = datetime.strptime(args[1], "%Y-%m-%d").date()
    except (IndexError, ValueError):
        print("couldnt parse args[1] as a date. must be in format YYYY-mm-dd")
        sys.exit(1)

    src = f"/{DIR}/data/nba/oddsportal/nba_odds_{date}.json"

    try:
        with open(src, "r") as f:
            data = json.load(f)
    except OSError as e:
        print(f"could not open file for date: {date}\ncause: {e}")
        sys.exit(1)
    odds = parse_odds(data)

    export_odds_to_csv(odds)


def parse_odds(data):
    books = ["fanduel", "betmgm", "draftkings"]
    game_odds = []

    for game in data:
        date = (
            datetime.fromisoformat(game["commence_time"])
            .astimezone(ZoneInfo("America/Los_Angeles"))
            .date()
        )
        home = game["home_team"]
        away = game["away_team"]
        spreads_home = []
        spreads_away = []
        h2h_home = []
        h2h_away = []
        overunders = []
        for book in game["bookmakers"]:
            if book["key"] in books:
                for market in book["markets"]:
                    for outcome in market["outcomes"]:
                        if outcome["name"] == home:
                            if market["key"] == "h2h":  # moneyline
                                h2h_home.append(outcome["price"])
                            elif market["key"] == "spreads":
                                spreads_home.append(outcome["point"])
                            elif market["key"] == "totals":
                                pass
                            else:
                                print(f"unknown market {market['key']}")
                        elif outcome["name"] == away:
                            if market["key"] == "h2h":  # moneyline
                                h2h_away.append(outcome["price"])
                            elif market["key"] == "spreads":
                                spreads_away.append(outcome["point"])
                            else:
                                print(f"unknown market {market['key']}")
                        elif outcome["name"] in ["Over", "Under"]:
                            overunders.append(outcome["point"])
                        else:
                            print("team named in outcome is not a participant")

        if len(spreads_home) == 0 or len(h2h_away) == 0 or len(overunders) == 0:
            print("betting lines not yet determined by selected books.")
            print(f"game: {away} @ {home} ({date})")
            continue
        spread = statistics.median(spreads_home)
        assert spread == -1 * statistics.median(spreads_away)
        home_dec = statistics.median(h2h_home)
        away_dec = statistics.median(h2h_away)
        total = statistics.median(overunders)

        odds = BettingData(
            date=date,
            home=home,
            away=away,
            spread=abs(spread),
            moneyline_home=home_dec,
            moneyline_away=away_dec,
            total=total,
        )

        game_odds.append(odds)

    return game_odds


def export_odds_to_csv(odds):
    dst = f"{DIR}/data/nba/odds/fetched_odds.csv"

    old = pd.read_csv(dst)
    old["date"] = pd.to_datetime(old["date"], format="%Y-%m-%d").dt.date
    old.set_index(["date", "home", "away"], inplace=True)

    new = pd.DataFrame.from_records(vars(o) for o in odds)
    new.set_index(["date", "home", "away"], inplace=True)

    new_rows = new.index.difference(old.index)
    existing_rows = new.index.intersection(old.index)

    old.update(new.loc[existing_rows])

    combined = pd.concat([old, new.loc[new_rows]])

    # sort by date
    combined.reset_index(inplace=True)
    combined["date"] = pd.to_datetime(combined["date"])
    combined.sort_values("date", inplace=True)
    combined["date"] = combined["date"].dt.date

    print(f"saving new odds to: {dst}")
    combined.to_csv(dst, index=False)
