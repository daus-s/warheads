from textual.app import ComposeResult
from textual.containers import Container
from textual.widgets import Static


class Display(Container):
    DEFAULT_CSS = """
    Display {
        border: white;
        width: 82;
        height: 26;
        padding: 0 1;
    }
    """

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        yield Static("Hello, World!")
