#!/usr/bin/env python3

from pprint import pprint as pp
from typing import Any, Iterable, Optional


height = 1080
ratio = 16 / 9


class LinearSeries:
    height = 1080
    ratio = 16 / 9
    centre_start: complex
    centre_end: complex

    def __init__(
        self,
        centre_end: complex,
        zoom_end: float,
        centre_start: Optional[complex] = None,
        zoom_start: float = 1.0,
    ):
        """
        Initialise series.

        Args:
            centre_end:
                Coordinates of centre by end of series.
        """
        self.centre_end = centre_end
        if centre_start is None:
            centre_start = complex()
        self.centre_start = centre_start
        self.zoom_end = zoom_end
        self.zoom_start = zoom_start

    def generate(self, num_frames: int) -> Any:
        yield from linear_steps(self.zoom_start, self.zoom_end, num_frames)


def linear_steps(start: float, end: float, count: int) -> Iterable[float]:
    """
    Increment from start to end using count steps.

        >>> list(linear_steps(1, 10, 4))
        [1.0, 4.0, 7.0, 10.0]

    Args:
        start:
            First value to output. The only value output if count is less
            than two.
        end:
            Last value to output
        count:
            Number of values to output.

    Returns:
        Generator containing at least the start value.
    """
    if count < 2:
        yield start
        return

    value = start
    step = (end - start) / (count - 1)
    for index in range(count):
        yield start
        start += step


if __name__ == '__main__':
    centre_end = complex(-1.0366876213584, -0.34652974743624)
    zoom_end = 10
    series = LinearSeries(centre_end, zoom_end).generate(10)
    pp(list(series))
