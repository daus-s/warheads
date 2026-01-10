from textual.app import ComposeResult
from textual.containers import Container
from textual.widgets import Static


class UserAuth(Container):
    """user authentication widget"""

    DEFAULT_CSS = """
    UserAuth {
        margin: 0 1 0 0;
        height: auto;
        width: auto;
    }
    #greeting {
        height: auto;
        width: auto;
    }
    """

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        # user: str = "lisan al-gaib"
        yield Static("hello, lisan al-gaib!", id="greeting")
        # yield Static("login", id="greeting", shrink=True)
