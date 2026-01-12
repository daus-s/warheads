import pytest

from fetch.nba_requests import fetch_nba_history


def test_fetch_nba_history():
    response = fetch_nba_history(2024, "Playoffs")

    print(response)

    assert False
