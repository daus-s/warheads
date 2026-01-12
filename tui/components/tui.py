"""
A starter Textual TUI application demonstrating basic concepts.

Install: pip install textual
Run: python app.py
"""

from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Vertical
from textual.widgets import Footer, Header

from components.controls import Controls
from components.display import Display
from components.header import AppHeader


class WARHeadsTUI(App):
    """WARHeads Text User Interface"""

    DEFAULT_CSS = """
    .app {
        padding: 0 1;
    }
    """

    BINDINGS = [
        Binding("q", "quit", "Quit", priority=True),
        ("ctrl+c", "quit", "Quit"),
    ]

    def __init__(self):
        super().__init__()

    def compose(self) -> ComposeResult:
        """Create child widgets for the app."""
        yield Header()

        with Vertical(classes="app"):
            yield AppHeader()
            yield Display()
            yield Controls()
        yield Footer()
