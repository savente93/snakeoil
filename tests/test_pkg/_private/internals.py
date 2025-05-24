"""
internals.py

Internal helper functions and classes.
"""

HIDDEN_CONSTANT = 7
DEFAULT_FACTOR = 2


def calculate_secret_value(x: int, y: int) -> int:
    """
    Calculate a secret value by multiplying inputs and adding a constant.

    Args:
        x (int): First number.
        y (int): Second number.

    Returns:
        int: Secret value.

    Example:
        >>> calculate_secret_value(2, 3)
        13
    """
    return x * y + HIDDEN_CONSTANT


def _double_value(value: int) -> int:
    """
    Double the input value (private helper).

    Args:
        value (int): Value to double.

    Returns:
        int: Doubled value.
    """
    return value * 2


class InternalHelper:
    """
    Helper class for internal computations.

    Attributes:
        factor (int): Multiplier factor.
    """

    def __init__(self, factor: int = DEFAULT_FACTOR):
        """
        Initialize with a multiplication factor.

        Args:
            factor (int): Factor to multiply values by.
        """
        self.factor = factor

    def amplify(self, value: int) -> int:
        """
        Multiply value by factor.

        Args:
            value (int): Value to amplify.

        Returns:
            int: Amplified value.
        """
        doubled = _double_value(value)
        return doubled * self.factor

    def reset_factor(self):
        """
        Reset factor to default value.
        """
        self.factor = DEFAULT_FACTOR
