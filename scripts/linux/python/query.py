import json
import os
import sys
from datetime import UTC, datetime, time, timedelta
from zoneinfo import ZoneInfo

import requests
from dotenv import load_dotenv

load_dotenv()

API_KEY = os.environ["ODDS_API_KEY"]
PT = ZoneInfo("America/Los_Angeles")

# today's window in Pacific, converted to UTC
today = datetime.now(PT).date()
start = datetime.combine(today, time.min, tzinfo=PT)
end = start + timedelta(days=1)
fmt = lambda d: d.astimezone(UTC).strftime("%Y-%m-%dT%H:%M:%SZ")

path = f"../../../data/nba/oddsportal/nba_odds_{today}.json"
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
