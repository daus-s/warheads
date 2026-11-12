from pathlib import Path

from dotenv import load_dotenv


def whs_env():
    path = Path.home() / ".config" / "warheads" / ".env"
    assert load_dotenv(path), f"no env file at {path}"
    load_dotenv(path)
