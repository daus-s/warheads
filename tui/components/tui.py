"""
A starter Textual TUI application demonstrating basic concepts.

Install: pip install textual
Run: python app.py
"""

from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Container
from textual.widgets import Footer, Header, Label, Static

from components.title import Title


class WARHeadsTUI(App):
    """WARHeads Text User Interface"""

    CSS = """
        Screen {
            align: center middle;
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

        with Container(id="main_container"):
            yield Title()
            yield Static(
                "hello, lisan al-gaib!", id="greeting"
            )  # todo: implement user auth

        yield Footer()
