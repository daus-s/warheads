#!/bin/bash

# Loop through valid years values
for PARAM in {1999..2024}; do
    FORMATTED=$(printf "%02d" "$(((PARAM+1)%100))")
    GEN_PARAM="${PARAM}-${FORMATTED}"

    URL="https://stats.nba.com/stats/leaguegamelog?Counter=1000&DateFrom=&DateTo=&Direction=DESC&ISTRound=&LeagueID=00&PlayerOrTeam=P&Season=${GEN_PARAM}&SeasonType=Regular%20Season&Sorter=DATE"
    OUTPUT_FILE="../data/nba/${PARAM}_${FORMATTED}_pg.json"

    echo "@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"
    echo "$URL"
    echo "$OUTPUT_FILE"
    echo "@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@"

    # Fetch the URL and save the response
    echo "Fetching $URL"
    curl "$URL" \
      -H 'Accept: */*' \
      -H 'Accept-Language: en-US,en;q=0.9,de;q=0.8' \
      -H 'Cache-Control: no-cache' \
      -H 'Connection: keep-alive' \
      -H 'Origin: https://www.nba.com' \
      -H 'Pragma: no-cache' \
      -H 'Referer: https://www.nba.com/' \
      -H 'Sec-Fetch-Dest: empty' \
      -H 'Sec-Fetch-Mode: cors' \
      -H 'Sec-Fetch-Site: same-site' \
      -H 'User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36' \
      -H 'sec-ch-ua: "Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24"' \
      -H 'sec-ch-ua-mobile: ?0' \
      -H 'sec-ch-ua-platform: "macOS"' \
      -o "$OUTPUT_FILE"

      echo "Saved to $OUTPUT_FILE"
      sleep 5
done
