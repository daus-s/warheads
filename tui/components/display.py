from textual.app import ComposeResult
from textual.containers import Container
from textual.widgets import Static


class Display(Container):
    DEFAULT_CSS = """
    Display {
        color: $text;
        border: white;
        width: 80;
        height: 24;
    }
    """

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        yield Static("Hello, World!")
