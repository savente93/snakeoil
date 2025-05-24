"""
two.py

Module with a counter class.
"""


class Counter:
    """
    A simple counter class.

    Attributes:
        count (int): Current count.
    """

    def __init__(self, start: int = 0):
        """
        Initialize the counter.

        Args:
            start (int): Starting value of the counter.
        """
        self.count = start

    def increment(self) -> int:
        """
        Increment the count by 1.

        Returns:
            int: The new count.
        """
        self.count += 1
        return self.count

    def reset(self):
        """
        Reset the count to zero.
        """
        self.count = 0
