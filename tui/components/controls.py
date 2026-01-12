from textual.containers import Container, Vertical
from textual.widgets import Static


class Controls(Container):
    DEFAULT_CSS = """
    Controls {
       text-style: bold;
       padding: 0 0 0 1;
    }
    """

    def compose(self):
        with Vertical():
            yield Static("History")
            yield Static("Predictions")
