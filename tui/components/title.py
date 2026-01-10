from textual.app import ComposeResult
from textual.containers import Container
from textual.widgets import Static


class Title(Container):
    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Title widget for the application."""
        yield Static("WARHeadsAPI", id="title", shrink=True)
