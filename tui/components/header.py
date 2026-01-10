from textual.app import ComposeResult
from textual.containers import Container, Horizontal
from textual.widgets import Header

from components.title import Title
from components.user_auth import UserAuth


class AppHeader(Container):
    """Header widget for the application."""

    DEFAULT_CSS = """
    AppHeader {
        height: auto;
        width: 100%;
    }
    """

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()

        yield Horizontal(Title(), UserAuth(), classes="header-container")
