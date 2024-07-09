#!/usr/bin/env python3

from pprint import pprint as pp
import subprocess
import sys
from typing import Any, Iterable, Optional


def linear_steps(start: complex, end: complex, count: int) -> Iterable[float]:
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


class MandelbrotZoom:
    centre_end: complex
    centre_start: complex
    image_height = 1080
    image_width = 1920
    mandlebrot_height = 2.4

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
            centre_start = complex(-0.7, 0)
        self.centre_start = centre_start
        self.zoom_end = zoom_end
        self.zoom_start = zoom_start

    def arguments(self, num_frames: int) -> Iterable[tuple[str, complex, complex]]:
        """
        Generate series of arguments for out mandlebrot program.

        Each generated argument is a 3-tuple consisting of a resolution string,
        then the top-left and bottom-right coordinates, in the complex plane.

        For example:

            ('1920x1080' complex(-3, 2), complex(2,-2))

        Returns:
            Generator over 3-tuples, as described above.
        """
        height = self.mandlebrot_height
        resolution = f"{self.image_width}x{self.image_height}"
        centre_steps = linear_steps(self.centre_start, self.centre_end, num_frames)
        zoom_steps = linear_steps(self.zoom_start, self.zoom_end, num_frames)
        for centre, zoom in zip(centre_steps, zoom_steps):
            top_left, bottom_right = self._corners(centre, height)
            yield (
                resolution,
                f"{top_left.real},{top_left.imag}",
                f"{bottom_right.real},{bottom_right.imag}",
            )
            height = self.mandlebrot_height / zoom

    def _corners(self, centre: complex, height: float) -> tuple[complex, complex]:
        """
        Calculate top-left and bottom-right corners within complex plane.

        Args:
            centre:
                Desired centre of viewpoint.
            height:
                Height of viewpoint.
        """
        width = height * (self.image_width / self.image_height)
        top_left = complex(-width/2 + centre.real, height/2 + centre.imag)
        bottom_right = complex(width/2 + centre.real, -height/2 + centre.imag)
        return top_left, bottom_right


def main(centre: complex, zoom_end: float, num_frames: int) -> int:
    series = MandelbrotZoom(centre, zoom_end, centre_start=centre).arguments(num_frames)
    for index, (resolution, top_left, bottom_right) in enumerate(series):
        args = [
            'target/release/mandelbrot',
            f"animation{index:>06}.png",
            resolution,
            top_left,
            bottom_right,
        ]
        pp(args)
        subprocess.run(args, check=True)
    return 0



if __name__ == '__main__':
    centre_end = complex(-1.0366876213584, -0.34652974743624)
    num_frames = 1000
    zoom_end = 1e5
    sys.exit(main(centre_end, zoom_end, num_frames))
