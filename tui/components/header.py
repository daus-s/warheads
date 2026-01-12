from textual.app import ComposeResult
from textual.containers import Container, Horizontal

from components.title import Title
from components.user_auth import UserAuth


class AppHeader(Container):
    """Header widget for the application."""

    DEFAULT_CSS = """
    AppHeader {
        height: auto;
        width: 100%;
    }

    .header-container {
        height: auto;
        width: 100%;
    }

    .app-title {
        height: auto;
    }

    .user-auth {
        height: auto;
    }
    """

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        with Horizontal(classes="header-container"):
            yield Title(classes="app-title")
            yield UserAuth(classes="user-auth")
