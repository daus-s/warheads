from datetime import datetime

import pandas as pd

import odds
import teams

# module to aggregate multiple source of historical NBA odds together

# shoutout these goats
espn_data_path = "/home/daus/Documents/warheads/data/nba/odds/nba_2008-2026.csv"
mgm_data_path = "/home/daus/Documents/warheads/data/nba/odds/all_odds.csv"

# local files to fill in missing data.
local_data_path = "/home/daus/Documents/warheads/data/nba/odds/missing_odds.csv"

#file to input missing data
missing_data_path = "/home/daus/Documents/warheads/data/nba/odds/missing.csv"


# aggregated data file
historic_data_path = "/home/daus/Documents/warheads/data/nba/odds/historic_odds.csv"


espn_df = pd.read_csv(espn_data_path)
mgm_df = pd.read_csv(mgm_data_path)
local_df = pd.read_csv(local_data_path)


# combine data frames into one (they dont have the same schema)
espn_df = (espn_df[['date', 'away', 'home', 'spread', 'total', 'moneyline_away', 'moneyline_home', 'whos_favored']]
    .assign(
        away=espn_df['away'].map(teams.abbr_to_id),
        home=espn_df['home'].map(teams.abbr_to_id),
        moneyline_away=espn_df['moneyline_away'].map(odds.am_to_dec),
        moneyline_home=espn_df['moneyline_home'].map(odds.am_to_dec),
    ).rename(
        columns={'whos_favored': 'favorite'}
    ))


no_moneyline = espn_df['moneyline_away'].isna().sum()
print(f'espn bet data processed. {no_moneyline} record{'' if no_moneyline == 1 else 's' } missing moneyline odds')

mgm_df = (mgm_df[['game_date', 'away_team', 'home_team', 'spread_home_points', 'total_over_points', 'money_away_decimal_odds', 'money_home_decimal_odds']]
    .assign(
        game_date=mgm_df['game_date'].map(lambda s: datetime.strptime(s, '%Y-%m-%d-%H:%M').strftime("%Y-%m-%d")),
        away_team=mgm_df['away_team'].map(teams.name_to_id),
        home_team=mgm_df['home_team'].map(teams.name_to_id),
        spread_home_points=mgm_df['spread_home_points'].map(lambda x: abs(x)),
        favorite=lambda x: x.apply(lambda row: 'home' if row['money_home_decimal_odds'] < row['money_away_decimal_odds'] else 'away', axis=1),
    )
    .rename(
        columns={
            'game_date': "date",
            'away_team': "away",
            'home_team': "home",
            'spread_home_points': "spread",
            'total_over_points': "total",
            'money_away_decimal_odds': "moneyline_away",
            'money_home_decimal_odds': "moneyline_home",
        }
    ))

local_df = (local_df.assign(
    home=local_df['home'].map(teams.name_to_id),
    away=local_df['away'].map(teams.name_to_id),
))

espn_df = espn_df.set_index(['date', 'away', 'home'])
mgm_df = mgm_df.set_index(['date', 'away', 'home'])
local_df = local_df.set_index(['date', 'home', 'away'])

merged_df = espn_df.combine_first(mgm_df)

no_moneyline = merged_df['moneyline_away'].isna().sum()
print(f'mgm bet data processed and merged with espn data. {no_moneyline} record{'' if no_moneyline == 1 else 's' } missing moneyline odds')

merged_df = merged_df.combine_first(local_df)
no_moneyline = merged_df['moneyline_away'].isna().sum()
print(f'mgm and espn bet data processed and merged with local data. {no_moneyline} record{'' if no_moneyline == 1 else 's' } missing moneyline odds')

# export to our own version of the csv
# schema:
# date home away favorite moneyline_home, moneyline_away, spread, total
missing = merged_df[merged_df['moneyline_away'].isna() | merged_df['moneyline_home'].isna()]
missing = missing.reset_index()
missing = missing.assign(
    home=missing['home'].map(teams.id_to_name),
    away=missing['away'].map(teams.id_to_name)
)
# for (d, a, h) in zip(missing['date'], missing['away'], missing['home']):
#     print(d, a, '@', h)
missing.to_csv(missing_data_path, index=False)

ml_cols = ['moneyline_away', 'moneyline_home']

merged_df = merged_df.dropna()
merged_df[ml_cols] = merged_df[ml_cols].map('{:.2f}'.format)
merged_df = merged_df.reset_index()
merged_df.to_csv(historic_data_path, index=False)

print(f'betting data aggregated for {len(merged_df)} games and saved to {historic_data_path}')
