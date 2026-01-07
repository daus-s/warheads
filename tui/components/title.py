from textual.app import ComposeResult
from textual.widgets import Static


class Title(Static):
    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Title widget for the application."""
        yield Static("WARHeads", id="title")
