espn_to_abbr = {
    "lac": "LAC",
    "cha": "CHA",
    "phi": "PHI",
    "gs": "GSW",
    "utah": "UTA",
    "cle": "CLE",
    "no": "NOP",
    "wsh": "WAS",
    "lal": "LAL",
    "tor": "TOR",
    "hou": "HOU",
    "min": "MIN",
    "mem": "MEM",
    "bos": "BOS",
    "det": "DET",
    "den": "DEN",
    "ny": "NYK",
    "mia": "MIA",
    "okc": "OKC",
    "dal": "DAL",
    "atl": "ATL",
    "sa": "SAS",
    "mil": "MIL",
    "phx": "PHX",
    "ind": "IND",
    "chi": "CHI",
    "sac": "SAC",
    "bkn": "BKN",
    "por": "POR",
    "orl": "ORL",
}

id_map = {
    "ATL": 1610612737,
    "BOS": 1610612738,
    "CLE": 1610612739,
    "NOP": 1610612740,
    "CHI": 1610612741,
    "DAL": 1610612742,
    "DEN": 1610612743,
    "GSW": 1610612744,
    "HOU": 1610612745,
    "LAC": 1610612746,
    "LAL": 1610612747,
    "MIA": 1610612748,
    "MIL": 1610612749,
    "MIN": 1610612750,
    "BKN": 1610612751,
    "NYK": 1610612752,
    "ORL": 1610612753,
    "IND": 1610612754,
    "PHI": 1610612755,
    "PHX": 1610612756,
    "POR": 1610612757,
    "SAC": 1610612758,
    "SAS": 1610612759,
    "OKC": 1610612760,
    "TOR": 1610612761,
    "UTA": 1610612762,
    "MEM": 1610612763,
    "WAS": 1610612764,
    "DET": 1610612765,
    "CHA": 1610612766,
}

city_map = {
    "LA Lakers": "LAL",
    "Orlando": "ORL",
    "Chicago": "CHI",
    "Indiana": "IND",
    "Houston": "HOU",
    "Charlotte": "CHA",
    "Boston": "BOS",
    "Sacramento": "SAC",
    "Oklahoma City": "OKC",
    "Brooklyn": "BKN",
    "Utah": "UTA",
    "Milwaukee": "MIL",
    "Miami": "MIA",
    "San Antonio": "SAS",
    "Phoenix": "PHX",
    "Golden State": "GSW",
    "Detroit": "DET",
    "Atlanta": "ATL",
    "Portland": "POR",
    "Memphis": "MEM",
    "Minnesota": "MIN",
    "Cleveland": "CLE",
    "Toronto": "TOR",
    "Philadelphia": "PHI",
    "Washington": "WAS",
    "Dallas": "DAL",
    "Denver": "DEN",
    "New Orleans": "NOP",
    "LA Clippers": "LAC",
    "New York": "NYK",
}

full_name_map = {
    "Los Angeles Lakers": "LAL",
    "Orlando Magic": "ORL",
    "Chicago Bulls": "CHI",
    "Indiana Pacers": "IND",
    "Houston Rockets": "HOU",
    "Charlotte Hornets": "CHA",
    "Boston Celtics": "BOS",
    "Sacramento Kings": "SAC",
    "Oklahoma City Thunder": "OKC",
    "Brooklyn Nets": "BKN",
    "Utah Jazz": "UTA",
    "Milwaukee Bucks": "MIL",
    "Miami Heat": "MIA",
    "San Antonio Spurs": "SAS",
    "Phoenix Suns": "PHX",
    "Golden State Warriors": "GSW",
    "Detroit Pistons": "DET",
    "Atlanta Hawks": "ATL",
    "Portland Trail Blazers": "POR",
    "Memphis Grizzlies": "MEM",
    "Minnesota Timberwolves": "MIN",
    "Cleveland Cavaliers": "CLE",
    "Toronto Raptors": "TOR",
    "Philadelphia 76ers": "PHI",
    "Washington Wizards": "WAS",
    "Dallas Mavericks": "DAL",
    "Denver Nuggets": "DEN",
    "New Orleans Pelicans": "NOP",
    "Los Angeles Clippers": "LAC",
    "New York Knicks": "NYK",
}

id_to_abbr_map = {v: k for k, v in id_map.items()}
name_by_abbr = {v: k for k, v in city_map.items()}
full_name_by_abbr = {v: k for k, v in full_name_map.items()}


def espn_to_id(abbr):
    return id_map[espn_to_abbr[abbr]]


def name_to_id(name):
    return id_map[city_map[name]]


def full_name_to_id(name):
    return id_map[full_name_map[name]]


def id_to_name(id):
    return name_by_abbr[id_to_abbr_map[id]]


def id_to_abbr(id):
    return id_to_abbr_map[id]


def id_to_full_name(id):
    return full_name_by_abbr[id_to_abbr(id)]


assert set(espn_to_abbr.values()) == set(id_map) == set(city_map.values())
