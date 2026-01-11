from textual.app import ComposeResult
from textual.containers import Container
from textual.widgets import Static


class Title(Container):
    DEFAULT_CSS = """
    Title {
        padding: 0 0 0 1;
    }
    """

    def compose(self) -> ComposeResult:
        """Title widget for the application."""
        yield Static("WARHeadsAPI", id="title", shrink=True)
