"""
bar.py

Example module with a function and a class.
"""

DEFAULT_GREETING = "Hello"


def greet(name: str) -> str:
    """
    Return a greeting message.

    Args:
        name (str): Name to greet.

    Returns:
        str: Greeting message.

    Example:
        >>> greet("Anna")
        'Hello, Anna!'
    """
    return f"{DEFAULT_GREETING}, {name}!"


def greet_undocumented(name):
    return f"{DEFAULT_GREETING}, {name}!"


def _format_name(name: str) -> str:
    """
    Format the name string to title case (private helper).

    Args:
        name (str): Name string.

    Returns:
        str: Formatted name.
    """
    return name.title()


class Greeter:
    """
    Greeter class that holds a name and greets.

    Attributes:
        name (str): The name to greet.
    """

    DEFAULT_PERSONAL_GREETING = "Hi"

    def __init__(self, name: str):
        """
        Initialize with a name.

        Args:
            name (str): Name to greet.
        """
        self.name = _format_name(name)

    def greet(self) -> str:
        """
        Generate a greeting message.

        Returns:
            str: Greeting message.
        """
        return f"{self.DEFAULT_PERSONAL_GREETING}, {self.name}!"
